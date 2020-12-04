# randge

![Rust](https://github.com/2A5F/randge/workflows/Rust/badge.svg)
[![version](https://img.shields.io/crates/v/randge)](https://crates.io/crates/randge)
[![documentation](https://docs.rs/randge/badge.svg)](https://docs.rs/randge)
![LICENSE](https://img.shields.io/crates/l/randge)

Generate unique random numbers

## Example

```rust
use rand::thread_rng;

let v = randge(-15..15, 5, thread_rng());
let v: Vec<_> = v.collect();
// output: like [13, -3, -14, 5, 3]
```
