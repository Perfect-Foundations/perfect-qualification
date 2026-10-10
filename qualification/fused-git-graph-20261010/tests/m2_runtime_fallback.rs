//! Independent closed-form M2 reference: runtime fraction-free guard
//! falls back to exact Q[x] Euclidean division without losing coefficients.
use perfect_arithmetic::{Integer, Natural};
use perfect_polynomial::{IntegerPolynomial as Z, PolynomialError, RationalPolynomial as Q};
use perfect_rational::Rational;

#[test]
fn high_growth_geometric_division_has_independent_complete_quotient() {
    // A=x^60, B=x+C, C=2^96+3. Q[k]=(-C)^(59-k), R=(-C)^60.
    // This case passes the initial fraction-free estimate and exceeds
    // its 4096-bit runtime coefficient limit. Public exactness is unchanged.
    let c = Integer::from(Natural::one().shl_bits(96)).add(&Integer::from(3));
    let neg = c.neg();
    let b = Q::new(vec![Rational::from(c), Rational::one()]);
    let mut monomial = vec![Rational::zero(); 61];
    monomial[60] = Rational::one();
    let a = Q::new(monomial);
    let (q, r) = a.div_rem(&b).unwrap();
    let mut expected = vec![Rational::zero(); 60];
    let mut v = Integer::one();
    for k in (0..60).rev() {
        expected[k] = Rational::from(v.clone());
        v = v.mul(&neg);
    }
    assert_eq!(q, Q::new(expected));
    assert_eq!(r, Q::constant(Rational::from(v)));
    assert_eq!(b.mul(&q).add(&r), a);
    assert_eq!(r.degree(), Some(0));
    assert_eq!(a.div_rem(&Q::zero()), Err(PolynomialError::DivisionByZero));
}

#[test]
fn zero_and_unit_domain_gcd_and_division_stay_canonical() {
    let zz = Z::zero();
    let z = Z::new(vec![(-6).into(), 12.into(), (-18).into()]);
    let pos = Z::new(vec![6.into(), (-12).into(), 18.into()]);
    assert_eq!(zz.gcd(&zz).unwrap(), zz);
    assert_eq!(zz.gcd(&z).unwrap(), pos);
    assert_eq!(z.gcd(&zz).unwrap(), pos);
    let qq = Q::zero();
    let p = Q::new(vec![
        Rational::new((-3).into(), 2.into()).unwrap(),
        Rational::new(9.into(), 4.into()).unwrap(),
        Rational::new((-15).into(), 2.into()).unwrap(),
    ]);
    let expected = Q::new(vec![
        Rational::new(1.into(), 5.into()).unwrap(),
        Rational::new((-3).into(), 10.into()).unwrap(),
        Rational::one(),
    ]);
    assert_eq!(qq.gcd(&qq).unwrap(), qq);
    assert_eq!(qq.gcd(&p).unwrap(), expected);
    assert_eq!(p.gcd(&qq).unwrap(), expected);
    for unit in [Rational::one(), Rational::from(-1)] {
        let (q, r) = p.div_rem(&Q::constant(unit.clone())).unwrap();
        assert!(r.is_zero());
        assert_eq!(
            q,
            if unit == Rational::one() {
                p.clone()
            } else {
                p.neg()
            }
        );
    }
    let constant = Rational::new((-7).into(), 11.into()).unwrap();
    let (q, r) = p.div_rem(&Q::constant(constant.clone())).unwrap();
    assert!(r.is_zero());
    let expected_q = Q::new(
        p.coefficients()
            .iter()
            .map(|c| c.div(&constant).unwrap())
            .collect(),
    );
    assert_eq!(q, expected_q);
}
