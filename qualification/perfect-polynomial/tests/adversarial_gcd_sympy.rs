//! SymPy-created, source-independent structured GCD oracle; signed/dense/sparse/bounds.
use perfect_arithmetic::Integer;
use perfect_polynomial::{IntegerPolynomial as Z, RationalPolynomial as Q};
use perfect_rational::Rational;

fn parse(s: &str) -> Z {
    if s == "_" {
        return Z::zero();
    }
    Z::new(
        s.split(',')
            .map(|c| Integer::from(c.parse::<i128>().unwrap()))
            .collect(),
    )
}
fn rational(a: &Z, den: i64) -> Q {
    Q::new(
        a.coefficients()
            .iter()
            .cloned()
            .map(|c| Rational::new(c, Integer::from(den)).unwrap())
            .collect(),
    )
}
#[test]
fn adversarial_integer_and_rational_gcd_against_independent_sympy() {
    let corpus = include_str!("../vectors/adversarial-gcd-v1.tsv");
    let mut count = 0;
    for line in corpus.lines().skip(1) {
        let v: Vec<_> = line.split('\t').collect();
        assert_eq!(v.len(), 7, "{line}");
        let a = parse(v[1]);
        let b = parse(v[2]);
        let g = parse(v[3]);
        assert_eq!(a.gcd(&b).unwrap(), g, "{} Z[x] gcd", v[0]);
        assert_eq!(b.gcd(&a).unwrap(), g, "{} symmetric Z[x] gcd", v[0]);
        if !g.is_zero() {
            assert_eq!(
                a.div_exact(&g).unwrap().mul(&g),
                a,
                "{} Z[x] divides a",
                v[0]
            );
            assert_eq!(
                b.div_exact(&g).unwrap().mul(&g),
                b,
                "{} Z[x] divides b",
                v[0]
            );
        }
        let qa = rational(&a, 17);
        let qb = rational(&b, 19);
        let expected = rational(&g, 1).monic().unwrap();
        assert_eq!(qa.gcd(&qb).unwrap(), expected, "{} Q[x] gcd", v[0]);
        assert_eq!(
            qb.gcd(&qa).unwrap(),
            expected,
            "{} symmetric Q[x] gcd",
            v[0]
        );
        count += 1;
    }
    assert_eq!(count, 79);
}
