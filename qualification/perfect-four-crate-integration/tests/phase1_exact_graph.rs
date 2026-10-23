use core::cmp::Ordering;
use perfect_arithmetic::Integer;
use perfect_decimal::{Decimal, DecimalLimits};
use perfect_float::{Float, Precision};
use perfect_numeric::RoundingMode;
use perfect_rational::Rational;

#[test]
fn all_five_crates_share_exact_finite_semantics() {
    let a = Decimal::parse("1.25").unwrap();
    let b = Decimal::parse("2.5").unwrap();
    let limits = DecimalLimits::new(40).unwrap();
    let product = a.mul_exact(&b, limits).unwrap();
    assert_eq!(product, Decimal::parse("3.125").unwrap());

    let reference = Rational::new(Integer::from(25), Integer::from(8)).unwrap();
    assert_eq!(product.to_rational(limits).unwrap(), reference);

    for rounding in [
        RoundingMode::NearestTiesToEven,
        RoundingMode::NearestTiesAway,
        RoundingMode::TowardNegative,
        RoundingMode::TowardPositive,
        RoundingMode::TowardZero,
    ] {
        let p = Precision::new(128).unwrap();
        let from_decimal =
            Float::from_rational_round(&product.to_rational(limits).unwrap(), p, rounding).unwrap();
        let from_reference = Float::from_rational_round(&reference, p, rounding).unwrap();
        assert_eq!(
            from_decimal.value().total_cmp(from_reference.value()),
            Ordering::Equal
        );
        assert_eq!(from_decimal.ordering(), from_reference.ordering());
    }
}
