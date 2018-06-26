#![feature(test)]
extern crate crc32c_hw;
extern crate rand;
extern crate test;
use rand::{thread_rng, RngCore};

#[bench]
fn crc_8(b: &mut test::Bencher) {
  let mut rng = thread_rng();
  let mut buf = vec![0u8; 8];
  rng.fill_bytes(&mut buf);

  b.bytes = 8;
  b.iter(|| test::black_box(crc32c_hw::update(0, &buf)));
}

#[bench]
fn crc_65000(b: &mut test::Bencher) {
  let mut rng = thread_rng();
  let mut buf = vec![0u8; 65_000];
  rng.fill_bytes(&mut buf);

  b.bytes = 65_000;
  b.iter(|| test::black_box(crc32c_hw::update(0, &buf)));
}

#[bench]
fn crc_1000000(b: &mut test::Bencher) {
  let mut rng = thread_rng();
  let mut buf = vec![0u8; 1_000_000];
  rng.fill_bytes(&mut buf);

  b.bytes = 1_000_000;
  b.iter(|| test::black_box(crc32c_hw::update(0, &buf)));
}
