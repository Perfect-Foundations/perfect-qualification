//! Wave 24 mutation-derived false-positive shortcut regression.
//! Opposite-coefficient fast paths may *only* accept mathematically opposite
//! polynomials. This detects dangerous weakened predicates even where the
//! generic exact Euclidean fallback remains correct.
use perfect_arithmetic::Integer;
use perfect_polynomial::RationalPolynomial as Q;
use perfect_rational::Rational;
fn r(n: i64, d: i64) -> Rational {
    Rational::new(Integer::from(n), Integer::from(d)).unwrap()
}
fn q(coeffs: &[(i64, i64)]) -> Q {
    Q::new(coeffs.iter().map(|&(n, d)| r(n, d)).collect())
}
#[test]
fn near_opposite_mismatched_degree_and_all_coefficient_variants_never_shortcut() {
    let a = q(&[(1, 3), (-3, 7), (7, 11)]);
    let cases = [
        // An erroneous length predicate combined with truncating zip must
        // not mistake a shorter prefix-opposite polynomial for -a.
        ("shorter_opposite_prefix", q(&[(-1, 3), (3, 7)])),
        // A wrong numerator-magnitude predicate must not accept every
        // opposite-sign coefficient of different absolute magnitude.
        ("all_different_numerators", q(&[(-2, 3), (4, 7), (-8, 11)])),
        // A wrong denominator predicate must not accept equal magnitudes
        // paired with different rational denominators.
        (
            "all_different_denominators",
            q(&[(-1, 5), (3, 11), (-7, 13)]),
        ),
        // Wrong AND -> OR of first/rest accepts a first-coefficient match
        // even when the remaining coefficients contradict negation.
        (
            "first_opposite_remainder_not",
            q(&[(-1, 3), (-3, 7), (8, 11)]),
        ),
    ];
    for (label, b) in cases {
        assert_ne!(a, b.neg(), "bad oracle fixture: {label}");
        let (quotient, remainder) = a.div_rem(&b).unwrap();
        assert_eq!(
            b.mul(&quotient).add(&remainder),
            a,
            "{label}: exact Euclidean identity"
        );
        assert!(
            remainder.is_zero() || remainder.degree() < b.degree(),
            "{label}: remainder degree bound"
        );
        // In each fixture, constant -1 and zero remainder is incorrect.
        assert_ne!(
            quotient,
            Q::constant(r(-1, 1)),
            "{label}: false-positive negative shortcut"
        );
        assert!(!remainder.is_zero(), "{label}: invalid zero remainder");
    }
}
