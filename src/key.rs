//! # Keys
//! 
//! Squares keys *are not arbitrary*. To produce good output, keys must have specific structure.
//! The [`key`] function accepts an index and produces a random admissible key for you.
//! 
//! ### Properties of Admissible Keys:
//! 
//! 1. All nibbles in the key are non-zero (0x1 to 0xF).
//! 2. Lower 8 nibbles:
//!   - All these 8 nibbles are unique; no nibble repeats among them.
//!   - The first nibble must be odd: `{ 1, 3, 5, 7, 9, B, D, F }`
//! 3. 9th nibble: must differ from the 8th nibble
//! 4. Upper 7 nibbles:
//!   - repetition of nibbles is allowed, both from the lower 8, and other values in the upper 7, however 
//!     the upper 7 nibbles must not contain the 9th nibble.
//! 5. Inter-key:
//!   - For any two keys, at least one nibble among the lower 9 will differ.

/// Holds an admissible Squares key
/// 
/// ### Properties of Admissible Keys:
/// 
/// 1. All nibbles in the key are non-zero (0x1 to 0xF).
/// 2. Lower 8 nibbles:
///   - All these 8 nibbles are unique; no nibble repeats among them.
///   - The first nibble must be odd: `{ 1, 3, 5, 7, 9, B, D, F }`
/// 3. 9th nibble: must differ from the 8th nibble
/// 4. Upper 7 nibbles:
///   - repetition of nibbles is allowed, both from the lower 8, and other values in the upper 7, however 
///     the upper 7 nibbles must not contain the 9th nibble.
/// 5. Inter-key:
///   - For any two keys, at least one nibble among the lower 9 will differ.
#[derive(Clone, Copy, Debug)]
pub struct Key(u64);

impl Key {
  /// Make a Key without checking for admissibility. 
  /// It is up to you to ensure the key is valid (see [`Key`]).
  #[must_use] #[inline(always)]
  pub const fn unchecked(key: u64) -> Self { Self(key) }

  /// Checks admissibility of the key value
  #[must_use] #[inline(always)]
  pub const fn checked(key: u64) -> Result<Self, Inadmissible> {
    match check_admissibility(key) {
      Ok(_) => Ok(Self(key)),
      Err(e) => Err(e)
    }
  }

  /// Accepts an index and produces a random admissible key.
  /// Think of `index` as your seed: any value is acceptable, 
  /// and the same `index` will produce the same `key`.
  #[must_use] #[inline(always)]
  pub const fn with_index(index: u64) -> Self { key(index) }

  /// Provides the naked key value
  #[must_use] #[inline(always)]
  pub const fn inner(self) -> u64 { self.0 }
}

/// the key used to produce random admissible keys
const MASTER_KEY_A: Key = Key::unchecked(0xec13a6976ecf14ad);
/// the key used to produce random admissible keys
const MASTER_KEY_B: Key = Key::unchecked(0x72f9c8a323a5e4f1);

/// Deterministically produces a high entropy admissible key determined by `index`.
/// Think of `index` as your "seed", which determines the key.
/// 
/// ## Warning:
/// If your seed (`index`) is a small number, it may be possible for an adversary 
/// to brute-force guess the key this function produces using only RNG outputs,
/// allowing them to predict all RNG outputs.
#[must_use]
pub const fn key(index: u64) -> Key {
  // init list as 1..=15.
  let mut nibbles: [u8; 15] = [
    0x1, 0x2, 0x3, 0x4,
    0x5, 0x6, 0x7, 0x8,
    0x9, 0xA, 0xB, 0xC,
    0xD, 0xE, 0xF
  ];

  // shuffle nibbles randomly
  let destinations = super::u64(MASTER_KEY_A, index);
  let mut i = 14;
  while i > 0 {
    let dst = ((destinations >> (i * 4)) % 15) as usize;
    let dv = nibbles[dst];
    nibbles[dst] = nibbles[i as usize];
    nibbles[i as usize] = dv;
    i -= 1;
  }

  // ensure first nibble odd
  if nibbles[0] % 2 == 0 {
    let mut i = 1;
    while i < 15 {
      if nibbles[i] % 2 == 1 {
        let v = nibbles[0];
        nibbles[0] = nibbles[i];
        nibbles[i] = v;
        break;
      }
      i += 1;
    }
  }

  // assemble lower nibbles
  let mut output = 0;
  let mut i = 0;
  while i < 8 {
    let n = nibbles[i];
    output |= (n as u64) << (i * 4);
    i += 1;
  }

  // ensure nibbles 8 and 9 do not match (swap from upper nibbles to fix)
  let nib_8 = nibbles[7];
  let mut nib_9 = nibbles[8];
  if nib_8 == nib_9 {
    let mut i = 9;
    while i < 15 {
      if nibbles[i] != nib_8 {
        let t = nibbles[8];
        nibbles[8] = nibbles[i];
        nibbles[i] = t;
        nib_9 = t;
        break;
      }
      i += 1;
    }
  }

  output |= (nib_9 as u64) << 32;

  // assign upper 7 nibbles (bits 36-63)
  let mut rng_idx = index;
  let mut upper = super::u32(MASTER_KEY_B, rng_idx);
  let mut i = 9;
  let mut j = 0;
  while i < 16 {
    let mut nib = 1 + ((upper >> (j * 4)) % 15);
    while nib == nib_9 as u32 {
      j += 1;
      if j == 8 {
        j = 0;
        rng_idx += 1;
        upper = super::u32(MASTER_KEY_B, rng_idx)
      }
      nib = 1 + ((upper >> (j * 4)) % 15);
    }

    j += 1;
    if j == 8 {
      j = 0;
      rng_idx += 1;
      upper = super::u32(MASTER_KEY_B, rng_idx)
    }

    output |= (nib as u64) << (i * 4);
    i += 1;
  }

  Key(output)
}

/// ### Properties of Admissible Keys:
/// 
/// 1. All nibbles in the key are non-zero (0x1 to 0xF).
/// 2. Lower 8 nibbles:
///   - All these 8 nibbles are unique; no nibble repeats among them.
///   - The first nibble must be odd: `{ 1, 3, 5, 7, 9, B, D, F }`
/// 3. 9th nibble: must differ from the 8th nibble
/// 4. Upper 7 nibbles:
///   - repetition of nibbles is allowed, both from the lower 8, and other values in the upper 7, however 
///     the upper 7 nibbles must not contain the 9th nibble.
/// 5. Inter-key:
///   - For any two keys, at least one nibble among the lower 9 will differ.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Inadmissible {
  ContainsZeroNibble { position: usize, nibble: u8 },
  DuplicateNibbleInLower8 { position: usize, nibble: u8 },
  FirstNibbleEven { nibble: u8 },
  NinthNibbleEqualsEighth { nibble: u8 },
  NinthNibbleRepeated { position: usize, nibble: u8 },
}

impl Inadmissible {
  pub const fn message(&self) -> &'static str {
    match self {
      Inadmissible::ContainsZeroNibble { .. } => "Nibble is out of valid range (1 to 15)",
      Inadmissible::DuplicateNibbleInLower8 { .. } => "Duplicate nibble found in lower 8 nibbles",
      Inadmissible::FirstNibbleEven { .. } => "First nibble must be odd",
      Inadmissible::NinthNibbleEqualsEighth { .. } => "9th nibble is equal to the 8th nibble",
      Inadmissible::NinthNibbleRepeated { .. } => "9th nibble is repeated in upper nibbles",
    }
  }
}

#[must_use]
const fn check_admissibility(key: u64) -> Result<(), Inadmissible> {
  let mut nibbles: [u8; 16] = [0u8; 16];
  let mut i = 0;
  while i < 16 {
    nibbles[i] = ((key >> (i * 4)) & 0xF) as u8;
    i += 1;
  }

  // check: all nibbles are non-zero (1 to 15)
  i = 0;
  while i < 16 {
    let nibble = nibbles[i];
    if nibble < 1 || nibble > 15 {
      return Err(Inadmissible::ContainsZeroNibble { position: i, nibble });
    }
    i += 1;
  }

  // check: lower 8 nibbles (positions 0-7) are unique
  let mut used_digits = [false; 16]; // Index 0 unused
  i = 0;
  while i < 8 {
    let nibble = nibbles[i];
    if used_digits[nibble as usize] {
      return Err(Inadmissible::DuplicateNibbleInLower8 { position: i, nibble });
    }
    used_digits[nibble as usize] = true;
    i += 1;
  }

  // check: first nibble must be odd
  if nibbles[0] % 2 == 0 {
    return Err(Inadmissible::FirstNibbleEven { nibble: nibbles[0] });
  }

  // check: 9th nibble must differ from the 8th nibble
  if nibbles[8] == nibbles[7] {
    return Err(Inadmissible::NinthNibbleEqualsEighth { nibble: nibbles[8] });
  }

  // check: 9th nibble must not be repeated in upper 7 nibbles
  i = 9;
  while i < 16 {
    let nibble = nibbles[i];
    if nibble == nibbles[8] {
      return Err(Inadmissible::NinthNibbleRepeated { position: i, nibble });
    }
    i += 1;
  }

  // all checks pass, key is admissible
  Ok(())
}

#[cfg(test)]
mod tests {
  use crate::u64;
  use super::{check_admissibility, key, Key};

  #[test]
  fn key_properties_100m() {
    let idx_key = Key::checked(0x16d7358fe8d9a17b).unwrap();

    for i in 0..100_000_000 {
      // produce a random index
      let index = u64(idx_key, i);

      // make a key with the index
      let k = key(index);
      if let Err(e) = check_admissibility(k.inner()) {
        panic!("Key at index {} is invalid: {:?}", index, e);
      }
    }
  }
}