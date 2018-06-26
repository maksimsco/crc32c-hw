CRC32C implementation with support for CPU-specific acceleration instructions (SSE 4.2)
and software fallback.

[![crates.io](https://img.shields.io/crates/v/crc32c-hw.svg)](https://crates.io/crates/crc32c-hw) [![crc32c_hw](https://docs.rs/crc32c_hw/badge.svg)](https://docs.rs/crc32c-hw) [![Build Status](https://travis-ci.org/maksimsco/crc32c-hw.svg?branch=master)](https://travis-ci.org/maksimsco/crc32c-hw)

## Usage

To use `crc32c-hw`, add this to your `Cargo.toml`:

```toml
[dependencies]
crc32c-hw = "0.1.3"
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

## Licence

Distributed under the terms of both the MIT license and the Apache License (Version 2.0), with portions covered by various BSD-like licenses.

See LICENSE-APACHE, and LICENSE-MIT for details.
