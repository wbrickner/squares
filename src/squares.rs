use crate::Key;

/// exchange the u32 halves of a u64
const fn swap(x: u64) -> u64 {
  (x >> 32) | (x << 32)
}

/// perform `x * x + o`
const fn sq(x: u64, o: u64) -> u64 {
  x.wrapping_mul(x).wrapping_add(o)
}

/// one round of Squares
const fn round(x: u64, o: u64) -> u64 {
  swap(sq(x, o))
}

/// initial step of all squares variants
const fn init(key: Key, counter: u64) -> (u64, u64, u64) {
  let k = key.inner();
  let x = counter.wrapping_mul(k);
  let z = x.wrapping_add(k);
  (x, x, z)
}

/// Produces a `u32` of random bits.
/// Less efficient than [`squares::u64`].
#[must_use] #[inline(always)]
pub const fn u32(key: Key, index: u64) -> u32 {
  let (x, y, z) = init(key, index);

  let x = round(x, y);
  let x = round(x, z);
  let x = round(x, y);

  (sq(x, z) >> 32) as u32
}

/// Produces a `u64` of random bits.
#[must_use] #[inline(always)]
pub const fn u64(key: Key, index: u64) -> u64 {
  let (x, y, z) = init(key, index);

  let x = round(x, y);
  let x = round(x, z);
  let x = round(x, y);
  let t = sq(x, z);
  let x = swap(t);

  t ^ (sq(x, y) >> 32)
}
