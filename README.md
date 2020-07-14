Simpler Timer
=============

[![Crates.io](https://img.shields.io/crates/v/simpler_timer)][crates_url]
[![Docs.rs](https://docs.rs/simpler_timer/badge.svg)][docs_url]

[crates_url]: https://crates.io/crates/simpler_timer
[docs_url]: https://docs.rs/simpler_timer

This library provides a very simple, poll based timer.

To use, include the following in `Cargo.toml`
```toml
[dependencies]
simpler_timer = "0.1.0"
```

```rust
use simpler_timer::Timer;
use std::time::Duration;

fn main() {
    let periodic = Timer::with_duration(Duration::from_millis(100));
    let timeout = Timer::with_duration(Duration::from_secs(2));
    

    loop {
        if periodic.expired() {
            println!("tick");
            periodic.reset();
        }

        if timeout.expired() {
            break;
        }
    }

    println!("total elapsed time: {}ms", timeout.elapsed());
}
```