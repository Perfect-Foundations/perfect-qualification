//! Independent bounded-i128 oracle for all five published foundations.
//! This is integration evidence for this exact source graph, not Q0-Q6 completion.
use core::cmp::Ordering;

use perfect_arithmetic::{Integer, Natural};
use perfect_decimal::{Decimal, DecimalExponent, DecimalLimits};
use perfect_float::{Float, Precision};
use perfect_numeric::RoundingMode;
use perfect_rational::Rational;

fn decimal(coefficient: i128, exponent: i64) -> Decimal {
    Decimal::from_parts(
        coefficient.is_negative(),
        Natural::from(coefficient.unsigned_abs()),
        DecimalExponent::new(exponent),
    )
}

fn expected(num: i128, den: i128) -> Rational {
    Rational::new(Integer::from(num), Integer::from(den)).unwrap()
}

#[test]
fn exact_decimal_arithmetic_agrees_with_primitive_rational_oracle() {
    let limits = DecimalLimits::new(40).unwrap();
    let mut cases = 0_u64;
    for x in -23_i128..=23 {
        for y in -23_i128..=23 {
            let exp_x = i64::try_from((x + 23).rem_euclid(7)).unwrap() - 3;
            let exp_y = i64::try_from((y * 3 + 71).rem_euclid(7)).unwrap() - 3;
            let x_scaled = x * 10_i128.pow(u32::try_from(exp_x + 3).unwrap());
            let y_scaled = y * 10_i128.pow(u32::try_from(exp_y + 3).unwrap());

            let dx = decimal(x, exp_x);
            let dy = decimal(y, exp_y);
            let a = dx.add_exact(&dy, limits).unwrap();
            let s = dx.sub_exact(&dy, limits).unwrap();
            let m = dx.mul_exact(&dy, limits).unwrap();

            assert_eq!(
                a.to_rational(limits).unwrap(),
                expected(x_scaled + y_scaled, 1000),
                "add: x={x}, y={y}, exponents {exp_x},{exp_y}"
            );
            assert_eq!(
                s.to_rational(limits).unwrap(),
                expected(x_scaled - y_scaled, 1000),
                "sub: x={x}, y={y}, exponents {exp_x},{exp_y}"
            );
            assert_eq!(
                m.to_rational(limits).unwrap(),
                expected(x_scaled * y_scaled, 1_000_000),
                "mul: x={x}, y={y}, exponents {exp_x},{exp_y}"
            );
            cases += 3;
        }
    }
    assert_eq!(cases, 6627);
}

#[test]
fn all_five_crates_preserve_value_and_direction_for_signed_dyadics() {
    let limits = DecimalLimits::new(35).unwrap();
    let prec = Precision::new(128).unwrap();
    let modes = [
        RoundingMode::NearestTiesToEven,
        RoundingMode::NearestTiesAway,
        RoundingMode::TowardNegative,
        RoundingMode::TowardPositive,
        RoundingMode::TowardZero,
    ];
    let mut cases = 0_u64;
    for numerator in -47_i128..=47 {
        for denominator in [2_i128, 4, 5, 8, 10, 16, 20, 25] {
            let rational = expected(numerator, denominator);
            let rounded_float =
                Float::from_rational_round(&rational, prec, RoundingMode::NearestTiesToEven)
                    .unwrap();

            let decimal = decimal(numerator, 0);
            let decimal_ratio = decimal
                .to_rational(limits)
                .unwrap()
                .div(&expected(denominator, 1))
                .unwrap();
            assert_eq!(decimal_ratio, rational);
            let duplicate_float =
                Float::from_rational_round(&decimal_ratio, prec, RoundingMode::NearestTiesToEven)
                    .unwrap();
            assert_eq!(
                rounded_float.value().total_cmp(duplicate_float.value()),
                Ordering::Equal
            );
            assert_eq!(rounded_float.ordering(), duplicate_float.ordering());

            for mode in modes {
                let rounded = Float::from_rational_round(&rational, prec, mode).unwrap();
                let mirror = Float::from_rational_round(&decimal_ratio, prec, mode).unwrap();
                assert_eq!(rounded.value().total_cmp(mirror.value()), Ordering::Equal);
                assert_eq!(rounded.ordering(), mirror.ordering());
                cases += 1;
            }
        }
    }
    assert_eq!(cases, 3800);
}
