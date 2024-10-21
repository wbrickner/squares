#![allow(unused)]

use rand_core::{impls::fill_bytes_via_next, RngCore};
use crate::Key;

/// An RNG compatible with `rand`.
/// Increments counter internally.
#[derive(Clone, Copy, Debug)]
struct Squares {
  key: Key,
  index: u64
}

impl Squares {
  #[must_use] #[inline(always)]
  pub const fn with_key(key: Key) -> Self {
    Self { key, index: 0 }
  }

  /// Set the location in the RNG sequence
  #[must_use] #[inline(always)]
  pub const fn with_index(self, index: u64) -> Self {
    Self { key: self.key, index }
  }

  /// Get the current location in the RNG sequence
  #[must_use] #[inline(always)]
  pub const fn index(&self) -> u64 { self.index }

  /// Skip ahead in the sequence
  #[must_use] #[inline(always)]
  pub const fn skip(mut self, n: u64) -> Self {
    self.index += n;
    self
  }
}

impl RngCore for Squares {
  fn next_u32(&mut self) -> u32 {
    let r = super::u32(self.key, self.index);
    self.index += 1;
    r
  }

  fn next_u64(&mut self) -> u64 {
    let r = super::u64(self.key, self.index);
    self.index += 1;
    r
  }

  fn fill_bytes(&mut self, dest: &mut [u8]) {
    fill_bytes_via_next(self, dest);
  }

  fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
    fill_bytes_via_next(self, dest);
    Ok(())
  }
}