# Tokio Lock Experiment

This project attempts to answer these questions:

- Is it possible to use the sync primitives (`Mutex` etc) from tokio with an arbitrary non-tokio executor?
- Is there any significant build time difference between tokio and async-lock?
- Is there any significant performance difference between tokio and async-lock?

## How to run

First start the server (see below), then check a local browser at `localhost:8000`.

### Async-Lock

```sh
cargo run --features async-lock
```

### Tokio

```sh
cargo run --features tokio
```


## Is it possible?

Yes, this project works properly both with either `tokio` or `async-lock` features enabled.

## Is there a significant build-time difference?

| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| `cargo build --release --features async-lock` | 5.486 ± 0.240 | 5.101 | 5.844 | 1.00 |
| `cargo build --release --features tokio` | 6.022 ± 0.148 | 5.822 | 6.293 | 1.10 ± 0.06 |

The difference is measurable--about 1.1x--but not significant.

## Is there a significant build-size difference?

```sh
$ cargo build --release --no-default-features --features tokio && mv target/release/tokio-lock-experiment ex.tokio
   Compiling tokio-lock-experiment v0.1.0 (/home/coriolinus/projects/coriolinus/tokio-lock-experiment)
    Finished `release` profile [optimized] target(s) in 6.38s
$ cargo build --release --no-default-features --features async-lock && mv target/release/tokio-lock-experiment ex.async-
lock
   Compiling tokio-lock-experiment v0.1.0 (/home/coriolinus/projects/coriolinus/tokio-lock-experiment)
    Finished `release` profile [optimized] target(s) in 0.55s
$ file ex.*
ex.async-lock: ELF 64-bit LSB shared object, x86-64, version 1 (SYSV), dynamically linked, interpreter /lib64/ld-linux-x86-64.so.2, BuildID[sha1]=21790d2150c653fb0b89b2dfbf4277fde32d0872, for GNU/Linux 3.2.0, not stripped
ex.tokio:      ELF 64-bit LSB shared object, x86-64, version 1 (SYSV), dynamically linked, interpreter /lib64/ld-linux-x86-64.so.2, BuildID[sha1]=9097d32a203dc2c3ffa3dc777fceb4190cbf4be9, for GNU/Linux 3.2.0, not stripped
$ ls -l ex.*
-rwxr-xr-x 2 coriolinus coriolinus 800032 Nov 13 18:42 ex.async-lock
-rwxr-xr-x 2 coriolinus coriolinus 800248 Nov 13 18:41 ex.tokio
```

No.

## Is there any significant performance difference?

TBD
