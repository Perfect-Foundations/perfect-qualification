use perfect_arithmetic::{ArithmeticError, Integer, Natural};
use qualify_perfect_arithmetic::semantic_fingerprint;

const NATURAL_VECTORS: &str = include_str!("../vectors/natural.tsv");
const INTEGER_VECTORS: &str = include_str!("../vectors/integer.tsv");
const GCD_VECTORS: &str = include_str!("../vectors/gcd.tsv");

fn data_lines(text: &str) -> impl Iterator<Item = &str> {
    text.lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
}

fn gcd_u8(mut left: u8, mut right: u8) -> u8 {
    while right != 0 {
        let remainder = left % right;
        left = right;
        right = remainder;
    }
    left
}

#[test]
fn qualification_vectors_match_public_natural_api() {
    let mut checked = 0_usize;
    for line in data_lines(NATURAL_VECTORS) {
        let fields: Vec<_> = line.split_whitespace().collect();
        assert_eq!(fields.len(), 9, "malformed natural qualification vector: {line}");

        let seed: u64 = fields[0].parse().unwrap();
        let mul: u64 = fields[1].parse().unwrap();
        let add: u64 = fields[2].parse().unwrap();
        let inc: u64 = fields[3].parse().unwrap();
        let steps: usize = fields[4].parse().unwrap();
        let divisor: u64 = fields[5].parse().unwrap();
        let probe: u64 = fields[6].parse().unwrap();
        let expected_remainder: u64 = fields[7].parse().unwrap();
        let expected_q_probe: u64 = fields[8].parse().unwrap();

        let multiplier = Natural::from(mul);
        let mut value = Natural::from(seed);
        for step in 0..steps {
            let term = add + u64::try_from(step).unwrap() * inc;
            value = value.mul(&multiplier).add(&Natural::from(term));
        }

        let (quotient, remainder) = value.div_rem(&Natural::from(divisor)).unwrap();
        assert_eq!(
            remainder.try_to_u64().unwrap().into_value(),
            expected_remainder
        );
        assert_eq!(
            quotient
                .div_rem(&Natural::from(probe))
                .unwrap()
                .1
                .try_to_u64()
                .unwrap()
                .into_value(),
            expected_q_probe
        );
        checked += 1;
    }
    assert_eq!(checked, 24);
}

#[test]
fn qualification_vectors_match_public_integer_api() {
    let mut checked = 0_usize;
    for line in data_lines(INTEGER_VECTORS) {
        let fields: Vec<_> = line.split_whitespace().collect();
        assert_eq!(fields.len(), 9, "malformed integer qualification vector: {line}");

        let seed: i64 = fields[0].parse().unwrap();
        let mul: i64 = fields[1].parse().unwrap();
        let add: i64 = fields[2].parse().unwrap();
        let inc: i64 = fields[3].parse().unwrap();
        let steps: usize = fields[4].parse().unwrap();
        let divisor: i64 = fields[5].parse().unwrap();
        let probe: i64 = fields[6].parse().unwrap();
        let expected_remainder: i64 = fields[7].parse().unwrap();
        let expected_q_probe: i64 = fields[8].parse().unwrap();

        let multiplier = Integer::from(mul);
        let mut value = Integer::from(seed);
        for step in 0..steps {
            let term = add + i64::try_from(step).unwrap() * inc;
            value = value.mul(&multiplier).add(&Integer::from(term));
        }

        let (quotient, remainder) = value.div_rem_trunc(&Integer::from(divisor)).unwrap();
        assert_eq!(
            remainder.try_to_i64().unwrap().into_value(),
            expected_remainder
        );
        assert_eq!(
            quotient
                .div_rem_trunc(&Integer::from(probe))
                .unwrap()
                .1
                .try_to_i64()
                .unwrap()
                .into_value(),
            expected_q_probe
        );
        checked += 1;
    }
    assert_eq!(checked, 24);
}

#[test]
fn qualification_vectors_match_public_gcd_api() {
    let mut checked = 0_usize;
    for line in data_lines(GCD_VECTORS) {
        let fields: Vec<_> = line.split_whitespace().collect();
        assert_eq!(fields.len(), 7, "malformed GCD qualification vector: {line}");

        let seed: u64 = fields[0].parse().unwrap();
        let mul: u64 = fields[1].parse().unwrap();
        let add: u64 = fields[2].parse().unwrap();
        let inc: u64 = fields[3].parse().unwrap();
        let steps: usize = fields[4].parse().unwrap();
        let factor: u64 = fields[5].parse().unwrap();
        let expected: u64 = fields[6].parse().unwrap();

        let multiplier = Natural::from(mul);
        let mut value = Natural::from(seed);
        for step in 0..steps {
            let term = add + u64::try_from(step).unwrap() * inc;
            value = value.mul(&multiplier).add(&Natural::from(term));
        }

        let factor = Natural::from(factor);
        let left = value.mul(&factor);
        let right = value.add(&Natural::one()).mul(&factor);
        assert_eq!(
            left.gcd(&right).try_to_u64().unwrap().into_value(),
            expected
        );
        checked += 1;
    }
    assert_eq!(checked, 24);
}

#[test]
fn exhaustive_small_domains_match_independent_primitives() {
    for left in 0_u8..=u8::MAX {
        for right in 0_u8..=u8::MAX {
            let a = Natural::from(left);
            let b = Natural::from(right);

            assert_eq!(
                a.add(&b).try_to_u16().unwrap().into_value(),
                u16::from(left) + u16::from(right)
            );
            assert_eq!(
                a.mul(&b).try_to_u16().unwrap().into_value(),
                u16::from(left) * u16::from(right)
            );
            assert_eq!(
                a.gcd(&b).try_to_u8().unwrap().into_value(),
                gcd_u8(left, right)
            );

            match left.checked_sub(right) {
                Some(expected) => {
                    assert_eq!(
                        a.checked_sub(&b).unwrap().try_to_u8().unwrap().into_value(),
                        expected
                    );
                }
                None => assert_eq!(
                    a.checked_sub(&b),
                    Err(ArithmeticError::NegativeNaturalResult)
                ),
            }

            if right != 0 {
                let (q, r) = a.div_rem(&b).unwrap();
                assert_eq!(q.try_to_u8().unwrap().into_value(), left / right);
                assert_eq!(r.try_to_u8().unwrap().into_value(), left % right);
            }
        }
    }
}

#[test]
fn signed_semantics_and_domain_failures_are_explicit() {
    for left in i8::MIN..=i8::MAX {
        for right in i8::MIN..=i8::MAX {
            let a = Integer::from(left);
            let b = Integer::from(right);
            assert_eq!(
                a.add(&b).try_to_i16().unwrap().into_value(),
                i16::from(left) + i16::from(right)
            );
            assert_eq!(
                a.sub(&b).try_to_i16().unwrap().into_value(),
                i16::from(left) - i16::from(right)
            );
            assert_eq!(
                a.mul(&b).try_to_i16().unwrap().into_value(),
                i16::from(left) * i16::from(right)
            );

            if right != 0 {
                let (q, r) = a.div_rem_trunc(&b).unwrap();
                let expected_q = i16::from(left) / i16::from(right);
                let expected_r = i16::from(left) % i16::from(right);
                assert_eq!(q, Integer::from(expected_q));
                assert_eq!(r, Integer::from(expected_r));
            } else {
                assert_eq!(a.div_rem_trunc(&b), Err(ArithmeticError::DivisionByZero));
            }
        }
    }
}

#[test]
fn sign_magnitude_bit_structure_and_conversion_boundaries_compose() {
    let magnitude = Natural::one()
        .shl_bits(16_384)
        .add(&Natural::from(0x9e37_79b9_u64));
    assert_eq!(magnitude.bit_length(), 16_385);
    assert_eq!(magnitude.shl_bits(19).shr_bits(19), magnitude);

    let signed = Integer::from(magnitude.clone()).neg();
    assert!(signed.is_negative());
    assert_eq!(signed.unsigned_abs(), magnitude);
    assert_eq!(signed.neg().unsigned_abs(), magnitude);

    assert!(Natural::from(u64::MAX).try_to_u64().is_ok());
    assert!(
        Natural::from(u64::MAX)
            .add(&Natural::one())
            .try_to_u64()
            .is_err()
    );
    assert!(Integer::from(-1_i8).try_to_u64().is_err());
}

#[test]
fn retained_qualification_fingerprint_matches() {
    assert_eq!(semantic_fingerprint(), 0xf203_3362_4f98_0b77);
}
