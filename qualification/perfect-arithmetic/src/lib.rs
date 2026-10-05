#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#![doc = "Independent Perfect Qualification consumer for Perfect Arithmetic."]

use perfect_arithmetic::{Integer, Natural};

fn mix_byte(hash: &mut u64, value: u8) {
    *hash ^= u64::from(value).wrapping_add(0x9e);
    *hash = hash.rotate_left(13).wrapping_mul(0x9e37_79b1_85eb_ca87);
}

fn mix_u64(hash: &mut u64, value: u64) {
    for byte in value.to_le_bytes() {
        mix_byte(hash, byte);
    }
}

fn natural_with_bits(bits: u64, low: u64) -> Natural {
    let high = Natural::one().shl_bits(bits - 1);
    let mask = if bits == 1 {
        0
    } else if bits <= 64 {
        (1_u64 << (bits - 1)) - 1
    } else {
        u64::MAX
    };
    high.add(&Natural::from(low & mask))
}

fn mix_natural(hash: &mut u64, value: &Natural) {
    mix_u64(hash, value.bit_length());
    match value.trailing_zeros() {
        Some(bits) => {
            mix_byte(hash, 1);
            mix_u64(hash, bits);
        }
        None => mix_byte(hash, 0),
    }

    for probe in [257_u64, 65_537, 1_000_003] {
        let (_, remainder) = value.div_rem(&Natural::from(probe)).unwrap();
        mix_u64(hash, remainder.try_to_u64().unwrap().into_value());
    }
}

fn mix_integer(hash: &mut u64, value: &Integer) {
    mix_byte(hash, u8::from(value.is_negative()));
    mix_natural(hash, &value.unsigned_abs());
}

/// Returns a deterministic qualification fingerprint derived only from public
/// Perfect Arithmetic semantic observations.
#[must_use]
pub fn semantic_fingerprint() -> u64 {
    let mut hash = 0x243f_6a88_85a3_08d3_u64;

    mix_natural(&mut hash, &Natural::zero());
    mix_natural(&mut hash, &Natural::one());
    mix_integer(&mut hash, &Integer::zero());

    for left in 0_u64..32 {
        for right in 0_u64..32 {
            let a = Natural::from(left);
            let b = Natural::from(right);
            mix_natural(&mut hash, &a.add(&b));
            mix_natural(&mut hash, &a.mul(&b));
            mix_natural(&mut hash, &a.gcd(&b));
            mix_byte(&mut hash, u8::from(a.checked_sub(&b).is_ok()));
            if right != 0 {
                let (q, r) = a.div_rem(&b).unwrap();
                mix_natural(&mut hash, &q);
                mix_natural(&mut hash, &r);
            }
        }
    }

    for left in -16_i64..=16 {
        for right in -16_i64..=16 {
            let a = Integer::from(left);
            let b = Integer::from(right);
            mix_integer(&mut hash, &a.add(&b));
            mix_integer(&mut hash, &a.sub(&b));
            mix_integer(&mut hash, &a.mul(&b));
            if right != 0 {
                let (q, r) = a.div_rem_trunc(&b).unwrap();
                mix_integer(&mut hash, &q);
                mix_integer(&mut hash, &r);
            }
        }
    }

    let boundary_pairs = [
        (2_047_u64, 1_024_u64),
        (2_048, 1_024),
        (8_191, 2_048),
        (8_192, 2_048),
        (16_384, 4_096),
        (32_768, 8_192),
    ];

    for (index, (left_bits, right_bits)) in boundary_pairs.into_iter().enumerate() {
        let salt = u64::try_from(index).unwrap();
        let left = natural_with_bits(left_bits, 0x9e37_79b9_7f4a_7c15 ^ salt);
        let right = natural_with_bits(right_bits, 0xc2b2_ae3d_27d4_eb4f ^ salt);
        let product = left.mul(&right);
        let (quotient, remainder) = left.div_rem(&right).unwrap();

        for value in [&left, &right, &product, &quotient, &remainder] {
            mix_natural(&mut hash, value);
        }

        for left_negative in [false, true] {
            for right_negative in [false, true] {
                let mut signed_left = Integer::from(left.clone());
                let mut signed_right = Integer::from(right.clone());
                if left_negative {
                    signed_left = signed_left.neg();
                }
                if right_negative {
                    signed_right = signed_right.neg();
                }
                mix_integer(&mut hash, &signed_left.mul(&signed_right));
                let (q, r) = signed_left.div_rem_trunc(&signed_right).unwrap();
                mix_integer(&mut hash, &q);
                mix_integer(&mut hash, &r);
            }
        }
    }

    hash
}
