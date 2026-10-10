use perfect_arithmetic::Integer;
use perfect_polynomial::IntegerPolynomial as Z;
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
#[test]
fn five_hundred_seeded_independent_sympy_reference_cases() {
    let data = include_str!("../vectors/robustness-v1.tsv");
    let mut count = 0;
    for row in data.lines().skip(1) {
        let c: Vec<_> = row.split('\t').collect();
        assert_eq!(c.len(), 7);
        let a = parse(c[1]);
        let b = parse(c[2]);
        let g = parse(c[3]);
        assert_eq!(a.gcd(&b).unwrap(), g, "{} gcd", c[0]);
        assert_eq!(b.gcd(&a).unwrap(), g, "{} reverse gcd", c[0]);
        assert_eq!(a.add(&b), parse(c[4]), "{} add", c[0]);
        assert_eq!(a.mul(&b), parse(c[5]), "{} mul", c[0]);
        assert_eq!(a.derivative(), parse(c[6]), "{} derivative", c[0]);
        if !g.is_zero() {
            assert_eq!(a.div_exact(&g).unwrap().mul(&g), a, "{} divides a", c[0]);
            assert_eq!(b.div_exact(&g).unwrap().mul(&g), b, "{} divides b", c[0]);
        }
        count += 1;
    }
    assert_eq!(count, 500);
}
