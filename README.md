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

## Is there any significant performance difference?

TBD