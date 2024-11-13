//! An HTTP server buildable with either [`tokio::sync::Mutex`] or [`async_lock::Mutex`].
//!
//! It's intended to compare performance at build-time and runtime between the two libraries.
//!
//! Run with:
//!
//! ```notrust,sh
//! cargo run --features tokio
//! ```
//!
//! or
//!
//! ```notrust,sh
//! cargo run --features async-lock
//! ```
//!
//! Open in the browser any of these addresses:
//!
//! - http://localhost:8000/

#[cfg(all(feature = "tokio", feature = "async-lock"))]
compile_error!("tokio and async-lock are incompatible features; cannot build");

#[cfg(not(any(feature = "tokio", feature = "async-lock")))]
compile_error!("at least one of the `tokio` and `async-lock` features is required; cannot build");

#[cfg(feature = "tokio")]
use tokio::sync::Mutex;

#[cfg(feature = "async-lock")]
use async_lock::Mutex;

use smol::{
    future, io,
    net::{TcpListener, TcpStream},
    prelude::*,
    Executor,
};

use std::{io::Write as _, sync::Arc};

struct App {
    counter: Mutex<u32>,
}

impl App {
    fn new() -> Self {
        Self {
            counter: Mutex::new(0),
        }
    }

    /// Get the current value of the counter and increment it.
    /// Returns the value prior to incrementing.
    async fn get_and_increment_counter(&self) -> u32 {
        let mut lock = self.counter.lock().await;
        let value = *lock;
        *lock += 1;
        value
    }

    async fn generate_response(self: Arc<Self>, mut stream: TcpStream) -> io::Result<()> {
        let value = self.get_and_increment_counter().await;

        let mut out = Vec::new();
        write!(&mut out, "HTTP/1.0 200 OK\r\n").expect("writing status line");
        write!(&mut out, "\r\n\r\n").expect("writing blank line");
        write!(&mut out, "<html><body>{value}</body></html>\n").expect("writing body");

        stream.write_all(&out).await?;

        #[cfg(feature = "stdout")]
        println!("responded with {value}");

        Ok(())
    }
}

fn main() -> io::Result<()> {
    let ex = Executor::new();

    future::block_on(ex.run(async {
        let listener = TcpListener::bind("127.0.0.1:8000").await?;
        #[cfg(feature = "stdout")]
        println!("listening on {}", listener.local_addr()?);

        let app = Arc::new(App::new());

        // continuously loop over incoming connections and handle them
        let mut incoming = listener.incoming();
        while let Some(stream) = incoming.next().await {
            let stream = stream?;
            let app = app.clone();
            ex.spawn(app.generate_response(stream)).detach();
        }
        Ok(())
    }))
}
