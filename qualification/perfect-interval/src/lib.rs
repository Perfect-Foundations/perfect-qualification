//! Qualification-only independent MPFR/Big Rational corpus consumer.
//! Never a production dependency and never a registry package.
#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
pub use perfect_interval::{EnclosureContext, Interval, Precision};
