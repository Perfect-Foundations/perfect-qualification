#![cfg_attr(not(feature = "std"), no_std)]
//! External source-exact consumer of the five numerical foundations.
//! No mathematical implementation from a production crate is copied here.

pub use perfect_numeric::RoundingMode;

#[cfg(test)]
mod tests {
    use core::cmp::Ordering;
    use perfect_arithmetic::{Integer, Natural};
    use perfect_float::{Float, FloatClass, FloatError, Precision};
    use perfect_numeric::{ConversionStatus, RoundingMode};
    use perfect_rational::{Rational, RationalError};

    fn precision(bits: u32) -> Precision {
        Precision::new(bits).expect("nonzero test precision")
    }

    #[test]
    fn rational_canonical_pipeline_has_primitive_reference() {
        for n in -11_i128..=11 {
            for d in 1_i128..=9 {
                let value = Rational::new(Integer::from(n), Integer::from(d))
                    .expect("positive denominator");
                let scaled = Rational::new(Integer::from(n * 18), Integer::from(d * 18))
                    .expect("positive scaled denominator");
                assert_eq!(value, scaled);
                assert_eq!(value.add(&Rational::zero()), value);
                assert_eq!(value.sub(&value), Rational::zero());
                let multiplier = Rational::from_naturals(Natural::from(d as u128), Natural::one())
                    .expect("positive denominator");
                let product = value.mul(&multiplier);
                assert_eq!(
                    product,
                    Rational::new(Integer::from(n), Integer::one()).expect("denominator one")
                );
                let magnitude = Integer::from(Integer::from(n).unsigned_abs());
                assert_eq!(
                    if n < 0 { magnitude.neg() } else { magnitude },
                    Integer::from(n)
                );
            }
        }
    }

    #[test]
    fn shared_rounding_mode_handles_halfway_cases() {
        let input = Rational::new(Integer::from(5_i32), Integer::from(4_i32))
            .expect("positive denominator");
        let one = Float::from_integer_round(
            &Integer::one(),
            precision(2),
            RoundingMode::NearestTiesToEven,
        )
        .expect("exact one")
        .into_value();
        let one_half = Rational::new(Integer::from(3_i32), Integer::from(2_i32))
            .expect("positive denominator");
        let high =
            Float::from_rational_round(&one_half, precision(2), RoundingMode::NearestTiesToEven)
                .expect("exact high")
                .into_value();
        let nearest_even =
            Float::from_rational_round(&input, precision(2), RoundingMode::NearestTiesToEven)
                .expect("finite rounded");
        assert_eq!(nearest_even.ordering(), Ordering::Less);
        assert_eq!(nearest_even.value().total_cmp(&one), Ordering::Equal);
        let nearest_away =
            Float::from_rational_round(&input, precision(2), RoundingMode::NearestTiesAway)
                .expect("finite rounded");
        assert_eq!(nearest_away.ordering(), Ordering::Greater);
        assert_eq!(nearest_away.value().total_cmp(&high), Ordering::Equal);
        for (mode, expected) in [
            (RoundingMode::TowardZero, &one),
            (RoundingMode::TowardNegative, &one),
            (RoundingMode::TowardPositive, &high),
        ] {
            let rounded = Float::from_rational_round(&input, precision(2), mode)
                .expect("finite directed rounding");
            assert_eq!(rounded.value().total_cmp(expected), Ordering::Equal);
        }
    }

    #[test]
    fn negative_midpoint_respects_direction_and_sign() {
        let source = Rational::new(Integer::from(-5_i32), Integer::from(4_i32))
            .expect("positive denominator");
        let minus_one = Float::from_integer_round(
            &Integer::from(-1_i32),
            precision(2),
            RoundingMode::TowardZero,
        )
        .expect("exact negative one")
        .into_value();
        let minus_three_halves = Float::from_rational_round(
            &Rational::new(Integer::from(-3_i32), Integer::from(2_i32))
                .expect("nonzero denominator"),
            precision(2),
            RoundingMode::TowardZero,
        )
        .expect("exact negative 3/2")
        .into_value();

        let even =
            Float::from_rational_round(&source, precision(2), RoundingMode::NearestTiesToEven)
                .expect("round negative midpoint");
        assert_eq!(even.ordering(), Ordering::Greater);
        assert_eq!(even.value().total_cmp(&minus_one), Ordering::Equal);

        let away = Float::from_rational_round(&source, precision(2), RoundingMode::NearestTiesAway)
            .expect("round negative midpoint");
        assert_eq!(away.ordering(), Ordering::Less);
        assert_eq!(away.value().total_cmp(&minus_three_halves), Ordering::Equal);

        for (mode, expected) in [
            (RoundingMode::TowardNegative, &minus_three_halves),
            (RoundingMode::TowardPositive, &minus_one),
            (RoundingMode::TowardZero, &minus_one),
        ] {
            let rounded = Float::from_rational_round(&source, precision(2), mode)
                .expect("directed negative midpoint");
            assert_eq!(rounded.value().total_cmp(expected), Ordering::Equal);
        }
    }

    #[test]
    fn finite_composition_uses_the_same_numeric_contract() {
        let half =
            Rational::new(Integer::one(), Integer::from(2_i32)).expect("positive denominator");
        let rounded_half =
            Float::from_rational_round(&half, precision(16), RoundingMode::TowardZero)
                .expect("exact dyadic");
        assert_eq!(rounded_half.ordering(), Ordering::Equal);
        assert_eq!(rounded_half.value().class(), FloatClass::Finite);
        let sum = rounded_half
            .value()
            .add_round(
                rounded_half.value(),
                precision(16),
                RoundingMode::NearestTiesToEven,
            )
            .expect("finite small addition");
        let one =
            Float::from_integer_round(&Integer::one(), precision(16), RoundingMode::TowardZero)
                .expect("one exact");
        assert_eq!(sum.ordering(), Ordering::Equal);
        assert_eq!(sum.value().total_cmp(one.value()), Ordering::Equal);
        assert_eq!(sum.value().precision().bits(), 16);
    }

    #[test]
    fn error_and_special_value_semantics_are_explicit() {
        assert!(ConversionStatus::EXACT.is_exact());
        let zero = Rational::new(Integer::zero(), Integer::from(-37_i32))
            .expect("negative nonzero denominator");
        assert_eq!(zero, Rational::zero());
        assert!(!zero.is_negative());
        assert_eq!(zero.denominator(), &Natural::one());
        assert_eq!(
            Rational::new(Integer::one(), Integer::zero()).unwrap_err(),
            RationalError::ZeroDenominator
        );
        assert_eq!(Precision::new(0), Err(FloatError::InvalidPrecision));
        assert_eq!(
            Precision::with_limit(65, 64),
            Err(FloatError::PrecisionLimitExceeded {
                requested: 65,
                maximum: 64
            })
        );
        let negative_zero = Float::zero(precision(53), true);
        let positive_zero = Float::zero(precision(53), false);
        assert!(negative_zero.is_zero() && negative_zero.is_negative());
        assert!(positive_zero.is_zero() && !positive_zero.is_negative());
        assert_eq!(negative_zero.total_cmp(&positive_zero), Ordering::Less);
        assert!(Float::nan(precision(53)).is_nan());
        assert!(Float::infinity(precision(53), false).is_infinite());
    }
}
