CRC32C implementation with support for CPU-specific acceleration instructions (SSE 4.2)
and software fallback.

[![crates.io](https://img.shields.io/crates/v/crc32c-hw.svg)](https://crates.io/crates/crc32c-hw) [![Build Status](https://travis-ci.org/maksimsco/crc32c-hw.svg?branch=master)](https://travis-ci.org/maksimsco/crc32c-hw)

[Documentation](https://docs.rs/crc32c-hw)

## Usage

To use `crc32c-hw`, add this to your `Cargo.toml`:

```toml
[dependencies]
crc32c-hw = "0.1.2"
```

## Example

```rust
extern crate crc32c_hw;

let mut crc = 0;
crc = crc32c_hw::update(crc, b"123");
crc = crc32c_hw::update(crc, b"456");
crc = crc32c_hw::update(crc, b"789");
assert_eq!(crc, 0xe3069283);

assert_eq!(crc32c_hw::compute(b"123456789"), 0xe3069283);
```

The easiest way to build binaries with CPU-specific instructions support is via environment variable:

```bash
RUSTFLAGS="-C target_cpu=native" cargo build --release
```

## Performance

`cargo bench` on `MacBook Pro Intel Core i5 2,7 GHz` results in `~23.0 GBps` (hardware) /
`~2.5 GBps` (software) throughput.

```bash
test crc32c_hw::tests::crc_0_065_000 ... bench:       2,808 ns/iter (+/- 398)
test crc32c_hw::tests::crc_1_000_000 ... bench:      41,915 ns/iter (+/- 6,100)
test crc32c_sw::tests::crc_0_065_000 ... bench:      25,686 ns/iter (+/- 19,423)
test crc32c_sw::tests::crc_1_000_000 ... bench:     384,286 ns/iter (+/- 53,529)
```

## Licence

Distributed under the terms of both the MIT license and the Apache License (Version 2.0), with portions covered by various BSD-like licenses.

See LICENSE-APACHE, and LICENSE-MIT for details.
