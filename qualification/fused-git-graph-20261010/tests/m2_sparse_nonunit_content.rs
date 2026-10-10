//! Independent SymPy ZZ[x] sparse cofactors and >2048-bit signed content.
//! H=(x²+1)²; U=x⁹+x²+2; V=x⁸+3x+1. SymPy gcd(U,V)=1.
use perfect_arithmetic::{Integer, Natural};
use perfect_polynomial::{IntegerPolynomial as Z, RationalPolynomial as Q};
use perfect_rational::Rational;
fn z(v: &[i64]) -> Z {
    Z::new(v.iter().copied().map(Integer::from).collect())
}
fn as_q(p: &Z) -> Q {
    Q::new(
        p.coefficients()
            .iter()
            .cloned()
            .map(Rational::from)
            .collect(),
    )
}
#[test]
fn sparse_signed_nonunit_content_restored_after_exact_gcd() {
    let h = z(&[1, 0, 2, 0, 1]);
    let a = z(&[2, 0, 5, 0, 4, 0, 1, 0, 0, 1, 0, 2, 0, 1]);
    let b = z(&[1, 3, 2, 6, 1, 3, 0, 0, 1, 0, 2, 0, 1]);
    assert_eq!(a.gcd(&b).unwrap(), h);
    let s = Integer::from(Natural::one().shl_bits(2048)).add(&Integer::from(13));
    let aa = a.mul(&Z::constant(s.mul(&Integer::from(-21))));
    let bb = b.mul(&Z::constant(s.mul(&Integer::from(35))));
    let expected = h.mul(&Z::constant(s.mul(&Integer::from(7))));
    assert_eq!(aa.gcd(&bb).unwrap(), expected);
    assert_eq!(bb.gcd(&aa).unwrap(), expected);
    assert_eq!(as_q(&aa).gcd(&as_q(&bb)).unwrap(), as_q(&h));
    assert_eq!(expected.content(), aa.content().gcd(&bb.content()));
    assert_eq!(expected.mul(&aa.div_exact(&expected).unwrap()), aa);
    assert_eq!(expected.mul(&bb.div_exact(&expected).unwrap()), bb);
    assert!(!expected.coefficients().last().unwrap().is_negative());
}
