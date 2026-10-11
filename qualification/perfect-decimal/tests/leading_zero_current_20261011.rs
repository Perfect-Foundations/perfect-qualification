//! Current-source Decimal parser independent leading-zero representation vectors.
use perfect_arithmetic::Natural;
use perfect_decimal::{Decimal, DecimalError, DecimalExponent, DecimalLimits};
fn limit() -> DecimalLimits {
    DecimalLimits::new(1).unwrap()
}
#[test]
fn long_zero_prefix_preserves_quantum_and_negative_zero() {
    for width in [31_usize, 1024, 8192, 65536] {
        let zeros = "0".repeat(width);
        let integer = Decimal::parse(&format!("{zeros}7"), limit()).unwrap();
        assert!(integer.same_representation(&Decimal::from_parts(
            false,
            Natural::from(7_u8),
            DecimalExponent::new(0)
        )));
        let fractional = Decimal::parse(&format!("-{zeros}.{zeros}7"), limit()).unwrap();
        assert!(fractional.same_representation(&Decimal::from_parts(
            true,
            Natural::from(7_u8),
            DecimalExponent::new(-((width + 1) as i64))
        )));
        let signed_zero = Decimal::parse(&format!("-{zeros}.{zeros}"), limit()).unwrap();
        assert!(signed_zero.same_representation(&Decimal::from_parts(
            true,
            Natural::zero(),
            DecimalExponent::new(-(width as i64))
        )));
    }
}
#[test]
fn padding_does_not_hide_coefficient_limit_or_syntax_error() {
    let zeros = "0".repeat(8192);
    assert_eq!(
        Decimal::parse(&format!("{zeros}12"), limit()),
        Err(DecimalError::CoefficientLimitExceeded {
            required: 2,
            limit: 1
        })
    );
    assert_eq!(
        Decimal::parse(&format!("{zeros}1.2.3"), limit()),
        Err(DecimalError::InvalidSyntax)
    );
}
