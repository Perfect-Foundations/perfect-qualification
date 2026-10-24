use core::cmp::Ordering;
use perfect_arithmetic::Natural;
use perfect_decimal::{Decimal, DecimalContext, DecimalExponent, DecimalLimits, DecimalPrecision};
use perfect_numeric::RoundingMode;
#[test]
fn quantize_matches_independent_i64_division() {
    let modes = [
        RoundingMode::NearestTiesToEven,
        RoundingMode::NearestTiesAway,
        RoundingMode::TowardNegative,
        RoundingMode::TowardPositive,
        RoundingMode::TowardZero,
    ];
    let mut cases = 0;
    for signed in -64_i64..=64 {
        let negative = signed < 0;
        let magnitude = signed.unsigned_abs();
        for exponent in -3_i64..=3 {
            for target in -3_i64..=3 {
                for mode in modes {
                    let decimal = Decimal::from_parts(
                        negative,
                        Natural::from(magnitude),
                        DecimalExponent::new(exponent),
                    );
                    let context = DecimalContext::new(
                        DecimalPrecision::new(24).unwrap(),
                        mode,
                        DecimalLimits::new(40).unwrap(),
                    )
                    .unwrap();
                    let got = decimal
                        .quantize(DecimalExponent::new(target), context)
                        .unwrap();
                    let (expected, order) = if target <= exponent {
                        (
                            magnitude * 10_u64.pow((exponent - target) as u32),
                            Ordering::Equal,
                        )
                    } else {
                        let divisor = 10_u64.pow((target - exponent) as u32);
                        let rem = magnitude % divisor;
                        let q = magnitude / divisor;
                        let up = if rem == 0 {
                            false
                        } else {
                            match mode {
                                RoundingMode::TowardZero => false,
                                RoundingMode::TowardPositive => !negative,
                                RoundingMode::TowardNegative => negative,
                                RoundingMode::NearestTiesAway => 2 * rem >= divisor,
                                RoundingMode::NearestTiesToEven => {
                                    2 * rem > divisor || (2 * rem == divisor && q % 2 == 1)
                                }
                            }
                        };
                        let ord = if rem == 0 {
                            Ordering::Equal
                        } else if up != negative {
                            Ordering::Greater
                        } else {
                            Ordering::Less
                        };
                        (q + u64::from(up), ord)
                    };
                    assert_eq!(
                        got.value().coefficient(),
                        &Natural::from(expected),
                        "value={signed} exp={exponent} target={target} mode={mode:?}"
                    );
                    assert_eq!(got.value().exponent().get(), target);
                    assert_eq!(got.value().is_negative(), negative);
                    assert_eq!(
                        got.ordering(),
                        order,
                        "value={signed} exp={exponent} target={target} mode={mode:?}"
                    );
                    cases += 1;
                }
            }
        }
    }
    assert_eq!(cases, 31605);
}

#[test]
fn quantize_preserves_signed_zero_at_all_requested_quanta() {
    let context = DecimalContext::new(
        DecimalPrecision::new(24).unwrap(),
        RoundingMode::NearestTiesToEven,
        DecimalLimits::new(40).unwrap(),
    )
    .unwrap();
    for negative in [false, true] {
        for source_exp in [-3_i64, 0, 3] {
            for target_exp in [-3_i64, 0, 3] {
                let source = Decimal::zero(DecimalExponent::new(source_exp), negative);
                let result = source
                    .quantize(DecimalExponent::new(target_exp), context)
                    .unwrap();
                assert!(result.value().is_zero());
                assert_eq!(result.value().is_negative(), negative);
                assert_eq!(result.value().exponent().get(), target_exp);
                assert_eq!(result.ordering(), Ordering::Equal);
            }
        }
    }
}
