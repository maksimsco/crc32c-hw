use byteorder::{ByteOrder, LittleEndian};
use crc32c_sw_consts::{
  STRIDE_TABLE_0, STRIDE_TABLE_1, STRIDE_TABLE_2, STRIDE_TABLE_3, BYTE_TABLE,
};

fn read_u32(buf: &[u8], pos: usize) -> u32 {
  assert!(buf.len() >= pos + 4);

  let src = &buf[pos..];
  LittleEndian::read_u32(src)
}

fn compute_u8(crc: u32, buf: &[u8], pos: usize) -> u32 {
  assert!(buf.len() > pos);

  BYTE_TABLE[((crc as u8) ^ buf[pos]) as usize] ^ (crc >> 8)
}

fn compute_u32(crc: u32, buf: &[u8], pos: usize) -> u32 {
  let t0 = crc as u8;
  let t1 = (crc >> 8) as u8;
  let t2 = (crc >> 16) as u8;
  let t3 = (crc >> 24) as u8;

  read_u32(buf, pos)
    ^ STRIDE_TABLE_3[t0 as usize]
    ^ STRIDE_TABLE_2[t1 as usize]
    ^ STRIDE_TABLE_1[t2 as usize]
    ^ STRIDE_TABLE_0[t3 as usize]
}

fn combine(crc1: u32, crc2: u32) -> u32 {
  let mut result = crc2 ^ crc1;
  for _ in 0..4 {
    result = (result >> 8) ^ BYTE_TABLE[(result & 0xff) as usize];
  }

  result
}

#[inline]
pub fn crc32c_update<T>(crc: u32, buf: T) -> u32
where
  T: AsRef<[u8]>,
{
  let mut crc = !crc;
  let mut buf = buf.as_ref();

  while buf.len().trailing_zeros() < 2 {
    crc = compute_u8(crc, buf, 0);
    buf = &buf[1..];
  }

  if buf.len() >= 16 {
    let mut crc0 = read_u32(buf, 0) ^ crc;
    let mut crc1 = read_u32(buf, 4);
    let mut crc2 = read_u32(buf, 8);
    let mut crc3 = read_u32(buf, 12);
    buf = &buf[16..];

    while buf.len() >= 16 {
      crc0 = compute_u32(crc0, buf, 0);
      crc1 = compute_u32(crc1, buf, 4);
      crc2 = compute_u32(crc2, buf, 8);
      crc3 = compute_u32(crc3, buf, 12);
      buf = &buf[16..];
    }

    while buf.len() >= 4 {
      crc0 = compute_u32(crc0, buf, 0);
      let tmp0 = crc0;
      crc0 = crc1;
      crc1 = crc2;
      crc2 = crc3;
      crc3 = tmp0;
      buf = &buf[4..];
    }

    crc = 0;
    crc = combine(crc, crc0);
    crc = combine(crc, crc1);
    crc = combine(crc, crc2);
    crc = combine(crc, crc3);
  }

  while !buf.is_empty() {
    crc = compute_u8(crc, buf, 0);
    buf = &buf[1..];
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
