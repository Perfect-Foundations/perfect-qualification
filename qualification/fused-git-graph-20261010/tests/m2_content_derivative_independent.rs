//! Qualification-owned complete SymPy ZZ[x] vectors and derivative values.
//! H=(x^2-3x+5)^2; U=x^12-2x+7; V=x^11+3x^2+2.
//! SymPy checks gcd(U,V)=1, gcd(HU,HV)=H, including signed content.
use perfect_arithmetic::{Integer, Natural};
use perfect_polynomial::{IntegerPolynomial as Z, RationalPolynomial as Q};
use perfect_rational::Rational;

fn z(v: &[i64]) -> Z {
    Z::new(v.iter().copied().map(Integer::from).collect())
}
fn to_q(v: &Z) -> Q {
    Q::new(
        v.coefficients()
            .iter()
            .cloned()
            .map(Rational::from)
            .collect(),
    )
}

#[test]
fn sympy_content_large_signed_repeated_gcd_and_derivative_evaluation() {
    let h = z(&[25, -30, 19, -6, 1]);
    let a = z(&[
        175, -260, 193, -80, 19, -2, 0, 0, 0, 0, 0, 0, 25, -30, 19, -6, 1,
    ]);
    let b = z(&[
        50, -60, 113, -102, 59, -18, 3, 0, 0, 0, 0, 25, -30, 19, -6, 1,
    ]);
    assert_eq!(a.gcd(&b).unwrap(), h);
    assert_eq!(to_q(&a).gcd(&to_q(&b)).unwrap(), to_q(&h));
    assert_eq!(
        a.derivative().evaluate(&Integer::from(-2)),
        Integer::from(-6392520)
    );
    assert_eq!(
        b.derivative().evaluate(&Integer::from(-2)),
        Integer::from(2958840)
    );

    let shared = Integer::from(Natural::one().shl_bits(512)).add(&Integer::from(19));
    let aa = a.mul(&Z::constant(shared.mul(&Integer::from(-6))));
    let bb = b.mul(&Z::constant(shared.mul(&Integer::from(15))));
    let expected = h.mul(&Z::constant(shared.mul(&Integer::from(3))));
    assert_eq!(aa.gcd(&bb).unwrap(), expected);
    assert_eq!(bb.gcd(&aa).unwrap(), expected);
    assert_eq!(to_q(&aa).gcd(&to_q(&bb)).unwrap(), to_q(&h));
    assert_eq!(expected.mul(&aa.div_exact(&expected).unwrap()), aa);
    assert_eq!(expected.mul(&bb.div_exact(&expected).unwrap()), bb);
    assert_eq!(
        aa.derivative().evaluate(&Integer::from(-2)),
        shared.mul(&Integer::from(38355120))
    );
    assert_eq!(
        bb.derivative().evaluate(&Integer::from(-2)),
        shared.mul(&Integer::from(44382600))
    );
    assert_eq!(expected.content(), aa.content().gcd(&bb.content()));
}
