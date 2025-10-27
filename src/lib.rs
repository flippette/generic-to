#![doc = include_str!("../README.md")]
#![cfg_attr(
  feature = "nightly",
  feature(const_trait_impl, const_convert)
)]
#![no_std]
#![forbid(unsafe_code)]

#[cfg(feature = "nightly")]
mod nightly;
#[cfg(feature = "nightly")]
pub use nightly::*;

#[cfg(not(feature = "nightly"))]
mod stable;
#[cfg(not(feature = "nightly"))]
pub use stable::*;
