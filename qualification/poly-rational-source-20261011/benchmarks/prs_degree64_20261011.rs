use perfect_arithmetic::Integer;
use perfect_polynomial::{IntegerPolynomial as Z, RationalPolynomial as Q};
use perfect_rational::Rational;
use std::{hint::black_box, time::Instant};
fn z(degree: usize, nonzero: &[(usize, i64)]) -> Z {
    let mut out = vec![Integer::zero(); degree + 1];
    out[degree] = Integer::one();
    for &(i, c) in nonzero {
        out[i] = Integer::from(c);
    }
    Z::new(out)
}
fn q(p: &Z) -> Q {
    Q::new(
        p.coefficients()
            .iter()
            .cloned()
            .map(Rational::from)
            .collect(),
    )
}
fn measure<T>(mut f: impl FnMut() -> T, reps: usize) -> u128 {
    let mut runs = [0u128; 3];
    for sample in &mut runs {
        let start = Instant::now();
        for _ in 0..reps {
            black_box(f());
        }
        *sample = start.elapsed().as_nanos() / reps as u128;
    }
    runs.sort_unstable();
    runs[1]
}
fn main() {
    let g = z(3, &[(0, 1), (1, 2)]);
    for degree in [24_usize, 40, 48, 49, 56, 64, 65, 80] {
        for kind in ["sparse", "semisparse", "dense"] {
            let (l, r) = match kind {
                "sparse" => (vec![(0, 1), (2, 1)], vec![(0, 1), (1, -1)]),
                "semisparse" => (
                    vec![(0, 2), (2, -3), (4, 2), (5, 1)],
                    vec![(0, 1), (1, 2), (3, -2), (5, 1)],
                ),
                _ => (
                    vec![(0, 1), (1, 1), (2, 1), (3, 1), (4, 1), (5, 1)],
                    vec![(0, 1), (1, -1), (2, 1), (3, -1), (4, 1), (5, -1)],
                ),
            };
            let a = g.mul(&z(degree - 3, &l));
            let b = g.mul(&z(degree - 4, &r));
            let aq = q(&a);
            let bq = q(&b);
            let reps = if degree <= 48 { 6 } else { 4 };
            println!(
                "degree={degree} kind={kind} z_ns={} q_ns={}",
                measure(|| black_box(&a).gcd(black_box(&b)).unwrap(), reps),
                measure(|| black_box(&aq).gcd(black_box(&bq)).unwrap(), reps)
            );
        }
    }
}
