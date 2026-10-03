#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#![doc = "Independent Perfect Qualification consumer for Perfect Numeric."]

use perfect_numeric::{Conversion, ConversionLoss, ConversionStatus, Exactness, RoundingMode};

const FLAGS: [ConversionLoss; 5] = [
    ConversionLoss::ROUNDED,
    ConversionLoss::TRUNCATED,
    ConversionLoss::SATURATED,
    ConversionLoss::UNDERFLOWED,
    ConversionLoss::OVERFLOWED,
];

fn loss_from_mask(mask: u8) -> ConversionLoss {
    let mut loss = ConversionLoss::NONE;
    for (bit, flag) in FLAGS.into_iter().enumerate() {
        if mask & (1 << bit) != 0 {
            loss |= flag;
        }
    }
    loss
}

fn mix(hash: &mut u64, value: u8) {
    *hash ^= u64::from(value).wrapping_add(0x9e);
    *hash = hash.rotate_left(11).wrapping_mul(0x9e37_79b1_85eb_ca87);
}

/// Returns a deterministic qualification fingerprint derived only from public
/// Perfect Numeric semantic observations.
#[must_use]
pub fn semantic_fingerprint() -> u64 {
    let mut hash = 0x6a09_e667_f3bc_c909_u64;

    let modes = [
        RoundingMode::TowardNegative,
        RoundingMode::TowardPositive,
        RoundingMode::TowardZero,
        RoundingMode::NearestTiesToEven,
        RoundingMode::NearestTiesAway,
    ];

    for (index, mode) in modes.into_iter().enumerate() {
        mix(&mut hash, u8::try_from(index).unwrap());
        mix(&mut hash, u8::from(mode.is_directed()));
        mix(&mut hash, u8::from(mode.is_nearest()));
    }

    for mask in 0_u8..32 {
        let loss = loss_from_mask(mask);
        let status = ConversionStatus::from_loss(loss);

        mix(&mut hash, mask);
        mix(&mut hash, u8::from(loss.is_empty()));
        mix(
            &mut hash,
            match status.exactness() {
                Exactness::Exact => 0,
                Exactness::Inexact => 1,
            },
        );
        mix(&mut hash, u8::from(status.is_exact()));

        for flag in FLAGS {
            mix(&mut hash, u8::from(loss.contains(flag)));
        }

        for other_mask in 0_u8..32 {
            let other = loss_from_mask(other_mask);
            let union = loss.union(other);
            let composed = status.compose(ConversionStatus::from_loss(other));

            mix(&mut hash, other_mask);
            mix(&mut hash, u8::from(union.is_empty()));
            mix(&mut hash, u8::from(composed.is_exact()));

            for flag in FLAGS {
                mix(&mut hash, u8::from(union.contains(flag)));
                mix(&mut hash, u8::from(composed.loss().contains(flag)));
            }
        }
    }

    let exact = Conversion::exact(0x1234_u16);
    mix(&mut hash, u8::try_from(exact.into_value() & 0xff).unwrap());
    mix(&mut hash, u8::from(exact.status().is_exact()));

    hash
}
