//! CRC32C implementation with support for CPU-specific acceleration instructions (SSE 4.2)
//! and software fallback
//!
//! [![crates.io](https://img.shields.io/crates/v/crc32c-hw.svg)]
//! (https://crates.io/crates/crc32c-hw)
//! [![Build Status](https://travis-ci.org/maksimsco/crc32c-hw.svg?branch=master)]
//! (https://travis-ci.org/maksimsco/crc32c-hw)
//!
//! [Documentation](https://docs.rs/crc32c-hw)
//!
//! ## Usage
//!
//! To use `crc32c-hw`, add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! crc32c-hw = "0.1.2"
//! ```
//!
//! ## Example
//!
//! ```no_run
//! extern crate crc32c_hw;
//!
//! let mut crc = 0;
//! crc = crc32c_hw::update(crc, b"123");
//! crc = crc32c_hw::update(crc, b"456");
//! crc = crc32c_hw::update(crc, b"789");
//! assert_eq!(crc, 0xe3069283);
//!
//! assert_eq!(crc32c_hw::compute(b"123456789"), 0xe3069283);
//! ```
//!
//! The easiest way to build binaries with CPU-specific instructions support is via
//! environment variable:
//!
//! ```bash
//! RUSTFLAGS="-C target_cpu=native" cargo build --release
//! ```
//!
//! ## Performance
//!
//! `cargo bench` on `MacBook Pro Intel Core i5 2,7 GHz` results in `~23.0 GBps` (hardware) /
//! `~2.5 GBps` (software) throughput.
//!
//! ```bash
//! test crc32c_hw::tests::crc_0_065_000 ... bench:       2,808 ns/iter (+/- 398)
//! test crc32c_hw::tests::crc_1_000_000 ... bench:      41,915 ns/iter (+/- 6,100)
//! test crc32c_sw::tests::crc_0_065_000 ... bench:      25,686 ns/iter (+/- 19,423)
//! test crc32c_sw::tests::crc_1_000_000 ... bench:     384,286 ns/iter (+/- 53,529)
//! ```
#![allow(unused_attributes)]
#![feature(cfg_target_feature, custom_attribute, link_llvm_intrinsics, test)]
#![no_std]
#[cfg(not(feature = "no-stdlib"))]
#[macro_use]
extern crate std;
#[cfg(test)]
extern crate rand;
extern crate stdsimd;
#[cfg(test)]
extern crate test;
#[cfg(target_feature = "sse4.2")]
mod crc32c_hw;
#[cfg(target_feature = "sse4.2")]
mod crc32c_hw_consts;
#[allow(dead_code)]
mod crc32c_sw;
#[allow(dead_code)]
mod crc32c_sw_consts;
use crc32c_implementation::*;
#[cfg(not(feature = "no-stdlib"))]
pub use stdlib_implementation::*;


#[cfg(not(feature = "no-stdlib"))]
mod stdlib_implementation {
  use super::*;
  use std::hash::Hasher;

  /// Implements `Hasher` trait and usually represent state that is changed while hashing data.
  ///
  /// ## Example
  /// ```no_run
  /// use crc32c_hw::Digest;
  /// use std::hash::Hasher;
  ///
  /// let mut s = Digest::new(0);
  /// s.write(b"123");
  /// s.write(b"456");
  /// s.write(b"789");
  ///
  /// assert_eq!(s.finish(), 0xe3069283);
  /// ```
  #[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
  pub struct Digest(u32);

  impl Digest {
    /// Creates new digest with CRC32C value.
    pub fn new(crc: u32) -> Digest {
      Digest(crc)
    }
  }

  impl Hasher for Digest {
    #[inline]
    fn write(&mut self, buf: &[u8]) {
      self.0 = crc32c_update(self.0, buf);
    }

    #[inline]
    fn finish(&self) -> u64 {
      u64::from(self.0)
    }
  }
}

/// Computes the CRC32C for the data.
pub fn compute<T>(buf: T) -> u32
where
  T: AsRef<[u8]>,
{
  crc32c_update(0, buf)
}

/// Computes the CRC32C for the data, starting with a previous CRC32C value.
pub fn update<T>(crc: u32, buf: T) -> u32
where
  T: AsRef<[u8]>,
{
  crc32c_update(crc, buf)
}

#[cfg(target_feature = "sse4.2")]
mod crc32c_implementation {
  use super::*;

  #[inline]
  pub fn crc32c_update<T>(crc: u32, buf: T) -> u32
  where
    T: AsRef<[u8]>,
  {
    crc32c_hw::crc32c_update(crc, buf)
  }
}

#[cfg(not(target_feature = "sse4.2"))]
mod crc32c_implementation {
  use super::*;

  #[inline]
  pub fn crc32c_update<T>(crc: u32, buf: T) -> u32
  where
    T: AsRef<[u8]>,
  {
    crc32c_sw::crc32c_update(crc, buf)
  }
}


#[cfg(all(test, not(feature = "no-stdlib"), target_feature = "sse4.2"))]
mod tests {
  use super::*;
  use rand::{OsRng, Rng};

  macro_rules! compare_test {
    (iterations=$iterations:expr, min=$min:expr, max=$max:expr) => ({
      let mut rng = OsRng::new().expect("rng");

      for _ in 0..$iterations {
        let mut buf = vec![0u8; rng.gen_range($min, $max)];
        rng.fill_bytes(&mut buf);

        let crc = rng.gen();
        let crc_hw = crc32c_hw::crc32c_update(crc, &buf);
        let crc_sw = crc32c_sw::crc32c_update(crc, &buf);

        assert_eq!(crc_hw, crc_sw);
      }
    })
  }

  #[test]
  fn compare_hw_and_sw() {
    compare_test!(iterations = 9000, min = 0, max = 430);
    compare_test!(iterations = 4000, min = 0, max = 3900);
    compare_test!(iterations = 1000, min = 0, max = 65000);
  }
}
