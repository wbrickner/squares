#![no_std]
#![doc = include_str!("../README.md")]
#![allow(unused_imports)]

mod key; pub use key::*;
mod squares; pub use squares::*;

#[cfg(feature = "rand")] mod rand;
#[cfg(feature = "rand")] pub use rand::*;