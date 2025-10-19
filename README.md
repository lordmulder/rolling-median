# rolling-median

[![no_std](https://img.shields.io/badge/rust-no__std-orchid?logo=rust)](https://docs.rust-embedded.org/book/intro/no-std.html)
[![Crates.io](https://img.shields.io/crates/v/rolling-median)](https://crates.io/crates/rolling-median)
[![Downloads](https://img.shields.io/crates/d/rolling-median)](https://crates.io/crates/rolling-median)
[![Release Date](https://img.shields.io/github/release-date/lordmulder/rolling-median)](https://crates.io/crates/rolling-median/versions)
[![Docs.rs](https://img.shields.io/docsrs/rolling-median)](https://docs.rs/rolling-median/latest/)
[![License](https://img.shields.io/crates/l/rolling-median)](https://opensource.org/license/0BSD)

Computes the [**median**](https://en.wikipedia.org/wiki/Median) of a data set, using a "rolling" (online) algorithm.

### Complexity:

The `push()` opreration has a complexity of: **O(log(n))**

The `get()` opreration has a complexity of: **O(1)**

### Installation

In order to use this crate, add it under `[dependencies]` to your **`Cargo.toml`**:

```
[dependencies]
rolling-median = "1.0.0"
```

### Usage

Here is a simple example that demonstrates how to use it:

```rust
use rolling_median::Median;

fn main() {
    let mut rolling_median = Median::new();

    while let Some(value) = get_data() {
        rolling_median.push(value);
        println!("Median, so far: {:?}", rolling_median.get())
    }

    println!("Final median: {:?}", rolling_median.get())
}

fn get_data() -> Option<f64> { /* ... */ }
```

## License

This software is released under the BSD Zero Clause (“0BSD”) License.

Copyright (C) 2025 by LoRd_MuldeR &lt;mulder2@gmx.de&gt;.
