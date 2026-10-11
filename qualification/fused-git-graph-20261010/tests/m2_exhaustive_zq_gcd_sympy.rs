//! Independent exhaustive small-degree Z[x]/Q[x] GCD coefficients.
//! The 1369 exact expected pairs are produced independently by SymPy 1.14.0,
//! not by production Polynomial. See references/gen_m2_degree2_gcd_sympy.py.
use perfect_arithmetic::Integer;
use perfect_polynomial::{IntegerPolynomial as Z, RationalPolynomial as Q};
use perfect_rational::Rational;

const ORACLE: &str = include_str!("../references/m2_degree2_zq_gcd_sympy_v1.tsv");

fn signed(value: &str) -> Integer {
    Integer::from(value.parse::<i64>().expect("oracle integer"))
}
fn zpoly(encoded: &str) -> Z {
    if encoded == "_" {
        return Z::zero();
    }
    Z::new(encoded.split(',').map(signed).collect())
}
fn qpoly(encoded: &str) -> Q {
    if encoded == "_" {
        return Q::zero();
    }
    Q::new(
        encoded
            .split(',')
            .map(|v| {
                let (n, d) = v.split_once('/').expect("oracle rational");
                Rational::new(signed(n), signed(d)).expect("nonzero denominator")
            })
            .collect(),
    )
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
fn exhaustive_degree_two_signed_zero_content_and_field_gcd_match_sympy() {
    let mut checked = 0;
    for line in ORACLE.lines() {
        let row: Vec<_> = line.split('\t').collect();
        assert_eq!(row.len(), 4, "oracle schema at index {checked}");
        let a = zpoly(row[0]);
        let b = zpoly(row[1]);
        let expected_integer = zpoly(row[2]);
        let expected_rational = qpoly(row[3]);

        let actual_integer = a.gcd(&b).expect("integer exact gcd");
        assert_eq!(actual_integer, expected_integer, "Z gcd at {checked}");
        let actual_rational = lift(&a).gcd(&lift(&b)).expect("rational exact gcd");
        assert_eq!(actual_rational, expected_rational, "Q gcd at {checked}");

        if !actual_integer.is_zero() {
            assert_eq!(
                a.div_exact(&actual_integer)
                    .expect("A divisible by gcd")
                    .mul(&actual_integer),
                a,
                "Z divisor A at {checked}"
            );
            assert_eq!(
                b.div_exact(&actual_integer)
                    .expect("B divisible by gcd")
                    .mul(&actual_integer),
                b,
                "Z divisor B at {checked}"
            );
        } else {
            assert!(a.is_zero() && b.is_zero(), "zero gcd only for two zeros");
        }
        checked += 1;
    }
    assert_eq!(checked, 1369, "all independent reference rows");
}
