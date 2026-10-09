//! Independent public-contract consumer for the Perfect Constants math namespace.
//! The production dependency points to an exact Git revision, not a registry crate.
#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]

use perfect_constants::math::{ALL, ConstantId};

/// The canonical Q-PC-MATH-V1 FNV-1a 64-bit fingerprint of the public API.
///
/// Serialization: ASCII `PF-CONSTANTS-MATH-V1\0`, then, in `ALL` order,
/// one byte of UTF-8 name length, name bytes, binary32 bits as little-endian
/// u32, and binary64 bits as little-endian u64. No formatting, allocation,
/// host-endianness, or Rust hasher identity is involved.
#[must_use]
pub fn semantic_fingerprint() -> u64 {
    const OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
    const PRIME: u64 = 0x0000_0100_0000_01b3;
    let mut hash = OFFSET;
    for byte in b"PF-CONSTANTS-MATH-V1\0" {
        hash = (hash ^ u64::from(*byte)).wrapping_mul(PRIME);
    }
    for id in ALL {
        let name = id.name().as_bytes();
        let count = u8::try_from(name.len()).expect("seven fixed documented constant IDs");
        hash = (hash ^ u64::from(count)).wrapping_mul(PRIME);
        for byte in name {
            hash = (hash ^ u64::from(*byte)).wrapping_mul(PRIME);
        }
        for byte in id.as_f32().to_bits().to_le_bytes() {
            hash = (hash ^ u64::from(byte)).wrapping_mul(PRIME);
        }
        for byte in id.as_f64().to_bits().to_le_bytes() {
            hash = (hash ^ u64::from(byte)).wrapping_mul(PRIME);
        }
    }
    hash
}

/// Witness that documented identifiers can be consumed without allocation.
#[must_use]
pub const fn first_const_value() -> (f32, f64) {
    (ConstantId::E.as_f32(), ConstantId::E.as_f64())
}
