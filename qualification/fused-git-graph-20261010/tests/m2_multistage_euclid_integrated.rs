//! Independent Fraction/SymPy-computable Euclidean chain with shared factor.
//! H=(x+1)^2, U=x^3+x+1, V=x^2+2x+3; gcd(U,V)=1.
use perfect_arithmetic::Integer;
use perfect_polynomial::{IntegerPolynomial as Z, RationalPolynomial as Q};
use perfect_rational::Rational;

fn z(v: &[i64]) -> Z {
    Z::new(v.iter().copied().map(Integer::from).collect())
}
fn r(n: i64, d: i64) -> Rational {
    Rational::new(n.into(), d.into()).unwrap()
}
fn q(v: &[(i64, i64)]) -> Q {
    Q::new(v.iter().map(|&(n, d)| r(n, d)).collect())
}
fn lift(p: &Z) -> Q {
    Q::new(
        p.coefficients()
            .iter()
            .cloned()
            .map(Rational::from)
            .collect(),
    )
}

#[test]
fn independently_specified_three_euclid_quotients_and_content_gcd() {
    // A=HU=(1,3,3,2,2,1); B=HV=(3,8,8,4,1).
    let a = z(&[1, 3, 3, 2, 2, 1]);
    let b = z(&[3, 8, 8, 4, 1]);
    let h = z(&[1, 2, 1]);
    let qa = lift(&a);
    let qb = lift(&b);
    let expected1 = q(&[(7, 1), (16, 1), (11, 1), (2, 1)]);
    let expected2 = q(&[(33, 4), (33, 2), (33, 4)]);
    let (q0, rem1) = qa.div_rem(&qb).unwrap();
    assert_eq!(q0, q(&[(-2, 1), (1, 1)]));
    assert_eq!(rem1, expected1);
    assert_eq!(qb.mul(&q0).add(&rem1), qa);

    let (q1, rem2) = qb.div_rem(&rem1).unwrap();
    assert_eq!(q1, q(&[(-3, 4), (1, 2)]));
    assert_eq!(rem2, expected2);
    assert_eq!(rem1.mul(&q1).add(&rem2), qb);

    let (q2, rem3) = rem1.div_rem(&rem2).unwrap();
    assert_eq!(q2, q(&[(28, 33), (8, 33)]));
    assert!(rem3.is_zero());
    assert_eq!(rem2.mul(&q2), rem1);
    assert_eq!(qa.gcd(&qb).unwrap(), lift(&h));
    assert_eq!(qb.gcd(&qa).unwrap(), lift(&h));

    // gcd_Z(-6*A, 15*B) = 3*H, positive leading, greatest content.
    let za = a.mul(&z(&[-6]));
    let zb = b.mul(&z(&[15]));
    let expected = h.mul(&z(&[3]));
    assert_eq!(za.gcd(&zb).unwrap(), expected);
    assert_eq!(zb.gcd(&za).unwrap(), expected);
    assert_eq!(expected.mul(&za.div_exact(&expected).unwrap()), za);
    assert_eq!(expected.mul(&zb.div_exact(&expected).unwrap()), zb);
}
