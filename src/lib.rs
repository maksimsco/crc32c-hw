//! CRC32C implementation with support for CPU-specific acceleration instructions (SSE 4.2)
//! and software fallback
//!
//! ![crates.io](https://img.shields.io/crates/v/crc32c-hw.svg)
//! ![crc32c_hw](https://docs.rs/mio/badge.svg)
//! ![Build Status](https://travis-ci.org/maksimsco/crc32c-hw.svg?branch=master)
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
#![no_std]
#[cfg(not(feature = "no-stdlib"))]
#[macro_use]
extern crate std;
extern crate byteorder;
#[cfg(test)]
extern crate rand;
#[cfg(target_feature = "sse4.2")]
mod crc32c_hw;
#[cfg(target_feature = "sse4.2")]
mod crc32c_hw_consts;
#[cfg(target_feature = "sse4.2")]
use crc32c_hw::crc32c_update;
#[cfg(any(test, not(target_feature = "sse4.2")))]
mod crc32c_sw;
#[cfg(any(test, not(target_feature = "sse4.2")))]
mod crc32c_sw_consts;
#[cfg(not(target_feature = "sse4.2"))]
use crc32c_sw::crc32c_update;
#[cfg(not(feature = "no-stdlib"))]
pub use stdlib_implementation::*;

#[cfg(not(feature = "no-stdlib"))]
mod stdlib_implementation {
  use super::*;
  use std::hash::Hasher;

  /// Implements `Hasher` trait and usually represent state that is changed while hashing data.
  ///
  /// ## Example
  /// ```
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

#[cfg(all(test, not(feature = "no-stdlib"), target_feature = "sse4.2"))]
mod tests {
  use super::crc32c_hw;
  use super::crc32c_sw;
  use rand::{thread_rng, Rng, RngCore};

  macro_rules! compare_test {
    (iterations = $iterations:expr,min = $min:expr,max = $max:expr) => {{
      let mut rng = thread_rng();

      for _ in 0..$iterations {
        let mut buf = vec![0u8; rng.gen_range($min, $max)];
        rng.fill_bytes(&mut buf);

        let crc = rng.gen();
        let crc_hw = crc32c_hw::crc32c_update(crc, &buf);
        let crc_sw = crc32c_sw::crc32c_update(crc, &buf);

        assert_eq!(crc_hw, crc_sw);
      }
    }};
  }

  #[test]
  fn compare_hw_and_sw() {
    compare_test!(iterations = 1000, min = 0, max = 430);
    compare_test!(iterations = 2000, min = 0, max = 3900);
    compare_test!(iterations = 3000, min = 0, max = 65000);
  }
}
