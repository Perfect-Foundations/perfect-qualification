use perfect_arithmetic::Integer;
use perfect_polynomial::{IntegerPolynomial as Z, RationalPolynomial as Q};
use perfect_rational::Rational;
use std::{hint::black_box, time::Instant};

fn integer(n: usize) -> Z {
    let mut cs = vec![Integer::zero(); n + 1];
    cs[0] = Integer::from(-12_i64);
    cs[n / 2] = Integer::from(18_i64);
    cs[n] = Integer::from(6_i64);
    Z::new(cs)
}
fn rational(z: &Z) -> Q {
    Q::new(
        z.coefficients()
            .iter()
            .cloned()
            .map(Rational::from)
            .collect(),
    )
}
fn run_z(kind: &str, n: usize, a: &Z, b: &Z, reps: u128) {
    let start = Instant::now();
    let mut checksum = 0_usize;
    for _ in 0..reps {
        let g = black_box(a).gcd(black_box(b)).unwrap();
        checksum ^= black_box(g.coefficients().len());
    }
    println!(
        "CONSTGCD domain=Z n={n} kind={kind} ns={} check={checksum}",
        start.elapsed().as_nanos() / reps
    );
}
fn run_q(kind: &str, n: usize, a: &Q, b: &Q, reps: u128) {
    let start = Instant::now();
    let mut checksum = 0_usize;
    for _ in 0..reps {
        let g = black_box(a).gcd(black_box(b)).unwrap();
        checksum ^= black_box(g.coefficients().len());
    }
    println!(
        "CONSTGCD domain=Q n={n} kind={kind} ns={} check={checksum}",
        start.elapsed().as_nanos() / reps
    );
}
fn main() {
    for n in [16_usize, 64, 128, 256] {
        let z = integer(n);
        let qc = rational(&z);
        let cz = Z::constant(Integer::from(-18_i64));
        let cq = Q::constant(Rational::new(Integer::from(-7_i64), Integer::from(11_i64)).unwrap());
        let reps = if n < 32 {
            300_u128
        } else if n < 128 {
            100
        } else {
            40
        };
        run_z("const_right", n, &z, &cz, reps);
        run_z("const_left", n, &cz, &z, reps);
        run_q("const_right", n, &qc, &cq, reps);
        run_q("const_left", n, &cq, &qc, reps);
        let other = Z::new(vec![Integer::one(), Integer::one()]);
        let other_q = rational(&other);
        run_z("control", n, &z, &other, 100);
        run_q("control", n, &qc, &other_q, 100);
    }
}
