//! Qualification-owned SymPy/Python fixed mathematical boundary vectors.
use perfect_arithmetic::{Integer, Natural};
use perfect_polynomial::{IntegerPolynomial as Z, PolynomialError, RationalPolynomial as Q};
use perfect_rational::Rational;
fn z(values: Vec<Integer>) -> Z {
    Z::new(values)
}
fn to_q(p: &Z) -> Q {
    Q::new(
        p.coefficients()
            .iter()
            .cloned()
            .map(Rational::from)
            .collect(),
    )
}
#[test]
fn independent_4096_accepted_and_4097_guard_public_gcd() {
    let p = Integer::from(Natural::one().shl_bits(2048));
    let a0 = p.sub(&Integer::one());
    let b3 = Integer::from(Natural::one().shl_bits(2049)).sub(&Integer::one());
    assert_eq!(a0.unsigned_bit_length(), 2048);
    assert_eq!(b3.unsigned_bit_length(), 2049);
    assert_eq!(a0.mul(&b3).unsigned_bit_length(), 4097);
    let a = z(vec![
        a0,
        Integer::zero(),
        Integer::zero(),
        Integer::zero(),
        Integer::one(),
    ]);
    let b = z(vec![Integer::zero(), Integer::zero(), Integer::one(), b3]);
    let one = z(vec![Integer::one()]);
    assert_eq!(a.gcd(&b).unwrap(), one);
    assert_eq!(b.gcd(&a).unwrap(), one);
    assert_eq!(
        to_q(&a).gcd(&to_q(&b)).unwrap(),
        Q::new(vec![Rational::one()])
    );
    let fits = z(vec![
        p,
        Integer::zero(),
        Integer::zero(),
        Integer::zero(),
        Integer::one(),
    ]);
    let fits_b = z(vec![
        Integer::zero(),
        Integer::zero(),
        Integer::one(),
        Integer::from(Natural::one().shl_bits(2047)),
    ]);
    assert_eq!(
        Integer::from(Natural::one().shl_bits(2048))
            .mul(&Integer::from(Natural::one().shl_bits(2047)))
            .unsigned_bit_length(),
        4096
    );
    assert_eq!(fits.gcd(&fits_b).unwrap(), z(vec![Integer::one()]));
}
#[test]
fn independently_expanded_wide_signed_integer_division_and_errors() {
    // (-3+2x)*(5-Wx+Wx²) = -15+(10+3W)x-5Wx²+2Wx³
    // independently expanded in Python, W=2^257+3.
    let w = Integer::from(Natural::one().shl_bits(257)).add(&Integer::from(3));
    let q = z(vec![Integer::from(5), w.neg(), w.clone()]);
    let divisor = z(vec![Integer::from(-3), Integer::from(2)]);
    let a = z(vec![
        Integer::from(-15),
        Integer::from(10).add(&w.mul(&Integer::from(3))),
        w.mul(&Integer::from(-5)),
        w.mul(&Integer::from(2)),
    ]);
    assert_eq!(a.div_exact(&divisor).unwrap(), q);
    assert_eq!(a.div_exact(&divisor.neg()).unwrap(), q.neg());
    assert_eq!(
        a.add(&z(vec![Integer::one()])).div_exact(&divisor),
        Err(PolynomialError::NonExactDivision)
    );
    assert_eq!(
        z(vec![1.into(), 1.into()]).div_exact(&z(vec![2.into(), 2.into()])),
        Err(PolynomialError::NonIntegralQuotient)
    );
    assert_eq!(
        a.div_exact(&Z::zero()),
        Err(PolynomialError::DivisionByZero)
    );
}
