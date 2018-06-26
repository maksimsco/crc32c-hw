use byteorder::{ByteOrder, LittleEndian};
use core::arch::x86_64::{_mm_crc32_u64, _mm_crc32_u8};
use crc32c_hw_consts::{BLOCK_0_TABLE, BLOCK_1_TABLE, BLOCK_2_TABLE};

const BLOCK_0_LEN: usize = 5440;
const BLOCK_1_LEN: usize = 1360;
const BLOCK_2_LEN: usize = 336;

#[inline]
fn compute_u8(crc: u32, buf: &[u8], pos: usize) -> u32 {
  assert!(buf.len() > pos);

  unsafe { _mm_crc32_u8(crc, buf[pos]) }
}

fn compute_u64<T>(crc: T, buf: &[u8], pos: usize) -> u64
where
  T: Into<u64>,
{
  assert!(buf.len() >= pos + 8);

  let src = &buf[pos..];
  unsafe { _mm_crc32_u64(crc.into(), LittleEndian::read_u64(src)) }
}

fn shift(crc: u32, block_table: &[[u32; 16]; 8]) -> u32 {
  let t0 = crc & 0xf;
  let t1 = (crc >> 4) & 0xf;
  let t2 = (crc >> 8) & 0xf;
  let t3 = (crc >> 12) & 0xf;
  let t4 = (crc >> 16) & 0xf;
  let t5 = (crc >> 20) & 0xf;
  let t6 = (crc >> 24) & 0xf;
  let t7 = (crc >> 28) & 0xf;

  block_table[0][t0 as usize]
    ^ block_table[1][t1 as usize]
    ^ block_table[2][t2 as usize]
    ^ block_table[3][t3 as usize]
    ^ block_table[4][t4 as usize]
    ^ block_table[5][t5 as usize]
    ^ block_table[6][t6 as usize]
    ^ block_table[7][t7 as usize]
}

#[inline]
pub fn crc32c_update<T>(crc: u32, buf: T) -> u32
where
  T: AsRef<[u8]>,
{
  let mut crc = !crc;
  let mut buf = buf.as_ref();

  while buf.len().trailing_zeros() < 3 {
    crc = compute_u8(crc, buf, 0);
    buf = &buf[1..];
  }

  while buf.len() >= BLOCK_0_LEN * 3 {
    let mut crc0 = u64::from(crc);
    let mut crc1 = 0;
    let mut crc2 = 0;
    for _ in 0..BLOCK_0_LEN / 8 {
      crc0 = compute_u64(crc0, buf, 0);
      crc1 = compute_u64(crc1, buf, BLOCK_0_LEN);
      crc2 = compute_u64(crc2, buf, BLOCK_0_LEN * 2);
      buf = &buf[8..];
    }

    buf = &buf[BLOCK_0_LEN * 2..];

    crc = crc0 as u32;
    crc = shift(crc, &BLOCK_0_TABLE) ^ crc1 as u32;
    crc = shift(crc, &BLOCK_0_TABLE) ^ crc2 as u32;
  }

  while buf.len() >= BLOCK_1_LEN * 3 {
    let mut crc0 = u64::from(crc);
    let mut crc1 = 0;
    let mut crc2 = 0;
    for _ in 0..BLOCK_1_LEN / 8 {
      crc0 = compute_u64(crc0, buf, 0);
      crc1 = compute_u64(crc1, buf, BLOCK_1_LEN);
      crc2 = compute_u64(crc2, buf, BLOCK_1_LEN * 2);
      buf = &buf[8..];
    }

    buf = &buf[BLOCK_1_LEN * 2..];

    crc = crc0 as u32;
    crc = shift(crc, &BLOCK_1_TABLE) ^ crc1 as u32;
    crc = shift(crc, &BLOCK_1_TABLE) ^ crc2 as u32;
  }

  while buf.len() >= BLOCK_2_LEN * 3 {
    let mut crc0 = u64::from(crc);
    let mut crc1 = 0;
    let mut crc2 = 0;
    for _ in 0..BLOCK_2_LEN / 8 {
      crc0 = compute_u64(crc0, buf, 0);
      crc1 = compute_u64(crc1, buf, BLOCK_2_LEN);
      crc2 = compute_u64(crc2, buf, BLOCK_2_LEN * 2);
      buf = &buf[8..];
    }

    buf = &buf[BLOCK_2_LEN * 2..];

    crc = crc0 as u32;
    crc = shift(crc, &BLOCK_2_TABLE) ^ crc1 as u32;
    crc = shift(crc, &BLOCK_2_TABLE) ^ crc2 as u32;
  }

  while buf.len() >= 8 {
    crc = compute_u64(crc, buf, 0) as u32;
    buf = &buf[8..];
  }

  !crc
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn crc() {
    assert_eq!(crc32c_update(0, b"123456789"), 0xe3069283);
  }

  #[test]
  fn crc_update() {
    let mut crc = 0;
    crc = crc32c_update(crc, b"123");
    crc = crc32c_update(crc, b"456");
    crc = crc32c_update(crc, b"789");
    assert_eq!(crc, 0xe3069283);
  }
}
