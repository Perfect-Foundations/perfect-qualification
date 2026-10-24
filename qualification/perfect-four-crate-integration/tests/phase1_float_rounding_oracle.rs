//! Qualification-owned integer proof oracle for bounded binary Float rounding.
//! Reference rounding decisions use u128 integer division, not Float internals.
use core::cmp::Ordering;
use perfect_arithmetic::Integer;
use perfect_float::{Float, Precision};
use perfect_numeric::RoundingMode;
use perfect_rational::Rational;

fn fraction(numerator: i128, denominator: i128) -> Rational {
    Rational::new(Integer::from(numerator), Integer::from(denominator)).unwrap()
}

#[test]
fn binary_rounding_matches_independent_integer_quotient() {
    let modes = [
        RoundingMode::NearestTiesToEven,
        RoundingMode::NearestTiesAway,
        RoundingMode::TowardNegative,
        RoundingMode::TowardPositive,
        RoundingMode::TowardZero,
    ];
    let mut cases = 0_u64;
    for numerator in -47_i128..=47 {
        if numerator == 0 {
            continue;
        }
        for denominator in [1_u128, 2, 3, 5, 7, 9, 13, 17, 23, 31] {
            let negative = numerator < 0;
            let n = numerator.unsigned_abs();
            let d = denominator;
            let mut exponent = if n >= d { 0_i32 } else { -1 };
            if n >= d {
                while n >= (d << (exponent + 1)) {
                    exponent += 1;
                }
            } else {
                while (n << (-exponent)) < d {
                    exponent -= 1;
                }
            }
            for bits in [3_u32, 5, 8, 16] {
                let precision = Precision::new(bits).unwrap();
                let shift = bits as i32 - 1 - exponent;
                let (scaled_n, scaled_d) = if shift >= 0 {
                    (n << shift, d)
                } else {
                    (n, d << (-shift))
                };
                let floor = scaled_n / scaled_d;
                let rem = scaled_n % scaled_d;
                assert!(floor >= (1_u128 << (bits - 1)));
                assert!(floor < (1_u128 << bits));
                for mode in modes {
                    let increment = rem != 0
                        && match mode {
                            RoundingMode::TowardZero => false,
                            RoundingMode::TowardPositive => !negative,
                            RoundingMode::TowardNegative => negative,
                            RoundingMode::NearestTiesAway => rem * 2 >= scaled_d,
                            RoundingMode::NearestTiesToEven => {
                                rem * 2 > scaled_d || (rem * 2 == scaled_d && floor % 2 == 1)
                            }
                        };
                    let q = (floor + u128::from(increment)) as i128;
                    let signed = if negative { -q } else { q };
                    let exact_dyadic = if shift >= 0 {
                        fraction(signed, 1_i128 << shift)
                    } else {
                        fraction(signed << (-shift), 1)
                    };
                    let representable = Float::from_rational_round(
                        &exact_dyadic,
                        precision,
                        RoundingMode::NearestTiesToEven,
                    )
                    .unwrap();
                    assert_eq!(representable.ordering(), Ordering::Equal);
                    let input = fraction(numerator, denominator as i128);
                    let result = Float::from_rational_round(&input, precision, mode).unwrap();
                    let direction = if rem == 0 {
                        Ordering::Equal
                    } else if increment != negative {
                        Ordering::Greater
                    } else {
                        Ordering::Less
                    };
                    assert_eq!(
                        result.ordering(),
                        direction,
                        "n={numerator}, d={denominator}, bits={bits}, mode={mode:?}"
                    );
                    assert_eq!(
                        result.value().total_cmp(representable.value()),
                        Ordering::Equal,
                        "value n={numerator}, d={denominator}, bits={bits}, mode={mode:?}"
                    );
                    cases += 1;
                }
            }
        }
    }
    assert_eq!(cases, 18_800);
}
