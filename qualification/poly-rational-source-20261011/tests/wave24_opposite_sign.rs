//! Exact independent coefficient oracle for near-opposite rational division.
//! A same-degree quotient is the ratio of leading coefficients; the
//! remainder is computed coefficient-wise, not via production div_rem.
use perfect_arithmetic::Integer;
use perfect_polynomial::{PolynomialError, RationalPolynomial as Q};
use perfect_rational::Rational;
fn r(n: i64, d: i64) -> Rational {
    Rational::new(Integer::from(n), Integer::from(d)).unwrap()
}
fn p(v: &[(i64, i64)]) -> Q {
    Q::new(v.iter().map(|&(n, d)| r(n, d)).collect())
}
#[test]
fn all_opposite_shortcut_predicates_have_independent_near_misses() {
    let original = p(&[(0, 1), (1, 2), (-3, 5), (0, 1), (7, 11)]);
    let exact_opposite = original.neg();
    let (q, rem) = original.div_rem(&exact_opposite).unwrap();
    assert_eq!(q, Q::constant(r(-1, 1)));
    assert_eq!(rem, Q::zero());
    for (name, other) in [
        (
            "nonzero_from_zero",
            p(&[(1, 7), (-1, 2), (3, 5), (0, 1), (-7, 11)]),
        ),
        (
            "equal_sign_middle",
            p(&[(0, 1), (-1, 2), (-3, 5), (0, 1), (-7, 11)]),
        ),
        (
            "unequal_magnitude",
            p(&[(0, 1), (-1, 2), (4, 5), (0, 1), (-7, 11)]),
        ),
        (
            "unequal_denominator",
            p(&[(0, 1), (-1, 2), (3, 7), (0, 1), (-7, 11)]),
        ),
        (
            "negative_lead_denominator",
            p(&[(0, 1), (-1, 2), (3, 5), (0, 1), (-7, 13)]),
        ),
        (
            "not_opposite_on_last",
            p(&[(0, 1), (-1, 2), (3, 5), (0, 1), (7, 11)]),
        ),
        (
            "constant_multiple_not_minus_one",
            p(&[(0, 1), (-1, 1), (6, 5), (0, 1), (-14, 11)]),
        ),
    ] {
        let (got_q, got_r) = original.div_rem(&other).unwrap();
        let expected_scale = original
            .coefficients()
            .last()
            .unwrap()
            .div(other.coefficients().last().unwrap())
            .unwrap();
        let expected_q = Q::constant(expected_scale.clone());
        assert_eq!(got_q, expected_q, "case={name}: quotient");
        let expected_remainder = Q::new(
            original
                .coefficients()
                .iter()
                .enumerate()
                .map(|(i, c)| {
                    let rhs = other.coefficient(i);
                    c.sub(&expected_scale.mul(&rhs))
                })
                .collect(),
        );
        assert_eq!(got_r, expected_remainder, "case={name}: remainder");
        assert_eq!(
            other.mul(&got_q).add(&got_r),
            original,
            "case={name}: identity"
        );
    }
    let shorter = p(&[(0, 1), (-1, 2), (3, 5), (-7, 11)]);
    let (sq, sr) = original.div_rem(&shorter).unwrap();
    assert_eq!(shorter.mul(&sq).add(&sr), original);
    assert!(sr.is_zero() || sr.degree() < shorter.degree());
    assert_eq!(
        original.div_rem(&Q::zero()),
        Err(PolynomialError::DivisionByZero)
    );
}
