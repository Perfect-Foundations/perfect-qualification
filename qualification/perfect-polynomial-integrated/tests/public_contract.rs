use perfect_arithmetic::{Integer, Natural};
use perfect_polynomial::{IntegerPolynomial as Z, PolynomialError, RationalPolynomial as Q};
use perfect_rational::Rational;

fn z(v: &[i64]) -> Z {
    Z::new(v.iter().copied().map(Integer::from).collect())
}
fn q(v: &[(i64, i64)]) -> Q {
    Q::new(
        v.iter()
            .map(|&(a, b)| Rational::new(Integer::from(a), Integer::from(b)).unwrap())
            .collect(),
    )
}

#[test]
fn external_integer_cross_operation_identities() {
    let a = z(&[3, -2, 0, 5]);
    let b = z(&[-4, 1, 1]);
    let c = z(&[1, 0, -3, 7]);
    assert_eq!(z(&[0, 0]), Z::zero());
    assert_eq!(z(&[3, 0, 0]), z(&[3]));
    assert_eq!(a.degree(), Some(3));
    assert_eq!(a.add(&b), z(&[-1, -1, 1, 5]));
    assert_eq!(a.sub(&b), z(&[7, -3, -1, 5]));
    assert_eq!(a.derivative(), z(&[-2, 0, 15]));
    assert_eq!(a.evaluate(&Integer::from(2)), Integer::from(39));
    assert_eq!(a.mul(&b).mul(&c), a.mul(&b.mul(&c)));
    assert_eq!(a.mul(&b.add(&c)), a.mul(&b).add(&a.mul(&c)));
    let factor = z(&[2, 2]);
    let left = factor.mul(&z(&[-3, 1]));
    let right = factor.mul(&z(&[1, 2]));
    assert_eq!(left.gcd(&right).unwrap(), factor);
    assert_eq!(left.div_exact(&factor).unwrap(), z(&[-3, 1]));
    assert_eq!(z(&[-6, 0, -12]).primitive_part().unwrap(), z(&[1, 0, 2]));
    assert_eq!(z(&[4, 0, 2]).content(), Natural::from(2_u8));
}

#[test]
fn external_rational_field_monic_and_division() {
    let a = q(&[(1, 2), (-3, 2), (1, 1)]);
    let b = q(&[(-1, 2), (1, 1)]);
    let (quot, rem) = a.div_rem(&b).unwrap();
    assert_eq!(rem, Q::zero());
    assert_eq!(b.mul(&quot).add(&rem), a);
    assert_eq!(quot, q(&[(-1, 1), (1, 1)]));
    assert_eq!(a.gcd(&b).unwrap(), b.monic().unwrap());
    let c = q(&[(3, 4), (-1, 2), (4, 7)]);
    let d = q(&[(8, 13), (3, 5)]);
    assert_eq!(
        c.mul(&d).derivative(),
        c.derivative().mul(&d).add(&c.mul(&d.derivative()))
    );
}

#[test]
fn external_typed_failures_and_zeros() {
    assert_eq!(
        z(&[1]).div_exact(&Z::zero()),
        Err(PolynomialError::DivisionByZero)
    );
    assert_eq!(
        z(&[1, 2]).div_exact(&z(&[2])),
        Err(PolynomialError::NonIntegralQuotient)
    );
    assert_eq!(
        z(&[1, 1]).div_exact(&z(&[1, 0, 1])),
        Err(PolynomialError::NonExactDivision)
    );
    assert_eq!(
        q(&[(1, 1)]).div_rem(&Q::zero()),
        Err(PolynomialError::DivisionByZero)
    );
    assert_eq!(Z::zero().gcd(&Z::zero()).unwrap(), Z::zero());
    assert_eq!(Q::zero().gcd(&Q::zero()).unwrap(), Q::zero());
}

#[test]
fn external_wide_signed_coefficients() {
    let huge = Integer::from(Natural::one().shl_bits(4095));
    let a = Z::new(vec![huge.clone(), Integer::zero(), Integer::one()]);
    let b = Z::new(vec![huge.clone(), Integer::zero(), Integer::from(2)]);
    // Wide inputs exceed the bounded PRS path but the exact public
    // fallback must still return a mathematically correct constant GCD.
    let g = a.gcd(&b).unwrap();
    assert_eq!(g, Z::new(vec![Integer::one()]));
    assert_eq!(a.div_exact(&g).unwrap(), a);
    assert_eq!(b.div_exact(&g).unwrap(), b);
}
