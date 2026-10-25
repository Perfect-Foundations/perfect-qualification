#![no_main]

use libfuzzer_sys::fuzz_target;
use perfect_arithmetic::{Integer, Natural};

const SHAPES: &[(u64, u64)] = &[
    (2_047, 1_024),
    (2_048, 1_024),
    (2_049, 1_024),
    (8_191, 2_048),
    (8_192, 2_048),
    (8_193, 2_048),
    (16_383, 4_096),
    (16_384, 4_096),
    (16_385, 4_096),
];

fn exact_bits(bits: u64, salt: u64) -> Natural {
    let high = Natural::one().shl_bits(bits - 1);
    high.add(&Natural::from(salt))
}

fn salt(bytes: &[u8]) -> u64 {
    let mut value = 0x9e37_79b9_7f4a_7c15_u64;
    for &byte in bytes.iter().take(32) {
        value ^= u64::from(byte);
        value = value.rotate_left(11).wrapping_mul(0xc2b2_ae3d_27d4_eb4f);
    }
    value
}

fuzz_target!(|data: &[u8]| {
    let control = data.first().copied().unwrap_or(0);
    let (left_bits, right_bits) = SHAPES[usize::from(control) % SHAPES.len()];
    let seed = salt(data.get(1..).unwrap_or_default());

    let left = exact_bits(left_bits, seed);
    let right = exact_bits(right_bits, seed.rotate_left(17) | 1);

    assert_eq!(left.bit_length(), left_bits);
    assert_eq!(right.bit_length(), right_bits);

    let product = left.mul(&right);
    let (product_q, product_r) = product.div_rem(&right).unwrap();
    assert_eq!(product_q, left);
    assert!(product_r.is_zero());

    let (quotient, remainder) = left.div_rem(&right).unwrap();
    assert_eq!(quotient.mul(&right).add(&remainder), left);
    assert!(remainder < right);

    let negative_left = Integer::from(left).neg();
    let signed_right = Integer::from(right);
    let (signed_q, signed_r) = negative_left.div_rem_trunc(&signed_right).unwrap();
    assert_eq!(signed_q.mul(&signed_right).add(&signed_r), negative_left);
    assert!(signed_r.unsigned_abs() < signed_right.unsigned_abs());
    if !signed_r.is_zero() {
        assert!(signed_r.is_negative());
    }
});
