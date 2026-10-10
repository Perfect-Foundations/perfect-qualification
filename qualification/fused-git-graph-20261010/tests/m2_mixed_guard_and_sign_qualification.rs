//! Independently SymPy-established H=(x+1)^2, U=x^5+2,
//! V=x^4+x+1, gcd(U,V)=1, gcd(HU,HV)=H over Q[x].
//! Exact Fraction denominator magnitudes exercise guarded PRS/fallback.
use perfect_arithmetic::{Integer, Natural};
use perfect_polynomial::{PolynomialError, RationalPolynomial as Q};
use perfect_rational::Rational;
fn z(n: i64) -> Rational {
    Rational::from(Integer::from(n))
}
fn q(n: i64, d: i64) -> Rational {
    Rational::new(Integer::from(n), Integer::from(d)).unwrap()
}
fn scaled(coeffs: &[i64], den: &Integer) -> Q {
    Q::new(
        coeffs
            .iter()
            .map(|&x| Rational::new(Integer::from(x), den.clone()).unwrap())
            .collect(),
    )
}
#[test]
fn sympy_coprime_cofactors_across_512_and_513_bit_denominators() {
    let h = Q::new(vec![z(1), z(2), z(1)]);
    let a_coeffs = [2, 4, 2, 0, 0, 1, 2, 1];
    let b_coeffs = [1, 3, 3, 1, 1, 2, 1];
    let d512 = Integer::from(Natural::one().shl_bits(511));
    let d513 = Integer::from(Natural::one().shl_bits(512));
    // 512-bit denominator remains private-PRS eligible; the distinct
    // 513-bit denominator requires exact public Euclidean fallback.
    let a = scaled(&a_coeffs, &d512);
    let b = scaled(&b_coeffs, &d513);
    assert_eq!(a.gcd(&b).unwrap(), h);
    assert_eq!(b.gcd(&a).unwrap(), h);
    let (quotient, remainder) = a.div_rem(&b).unwrap();
    assert_eq!(b.mul(&quotient).add(&remainder), a);
    assert!(remainder.degree().is_none_or(|d| d < b.degree().unwrap()));
    assert_eq!(a.div_rem(&Q::zero()), Err(PolynomialError::DivisionByZero));
    assert_eq!(h.monic().unwrap(), h);
}
#[test]
fn independent_signed_fraction_negative_monic_and_zero_gcd() {
    let negative = Q::new(vec![q(-3, 7), z(0), q(2, 11), z(-1)]);
    let canonical = Q::new(vec![q(3, 7), z(0), q(-2, 11), z(1)]);
    assert_eq!(negative.monic().unwrap(), canonical);
    assert_eq!(negative.gcd(&Q::zero()).unwrap(), canonical);
    assert_eq!(Q::zero().gcd(&negative).unwrap(), canonical);
    assert_eq!(Q::zero().monic().unwrap(), Q::zero());
}
