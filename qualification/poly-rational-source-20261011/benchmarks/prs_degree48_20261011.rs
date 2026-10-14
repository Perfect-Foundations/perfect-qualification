use perfect_arithmetic::Integer;
use perfect_polynomial::{IntegerPolynomial as Z, RationalPolynomial as Q};
use perfect_rational::Rational;
use std::{hint::black_box, time::Instant};

fn z(degree: usize, nonzero: &[(usize, i64)]) -> Z {
    let mut coeffs = vec![Integer::zero(); degree + 1];
    coeffs[degree] = Integer::one();
    for &(i, n) in nonzero {
        coeffs[i] = Integer::from(n);
    }
    Z::new(coeffs)
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
fn main() {
    let g = z(3, &[(0, 1), (1, 2)]);
    for degree in [12_usize, 16, 24, 28, 36, 40, 48] {
        let cases = [
            (
                "sparse",
                z(degree - 3, &[(0, 1), (2, 1)]),
                z(degree - 4, &[(0, 1), (1, -1)]),
            ),
            (
                "semisparse",
                z(degree - 3, &[(0, 2), (2, -3), (4, 2), (5, 1)]),
                z(degree - 4, &[(0, 1), (1, 2), (3, -2), (5, 1)]),
            ),
            (
                "dense",
                z(
                    degree - 3,
                    &[(0, 1), (1, 1), (2, 1), (3, 1), (4, 1), (5, 1)],
                ),
                z(
                    degree - 4,
                    &[(0, 1), (1, -1), (2, 1), (3, -1), (4, 1), (5, -1)],
                ),
            ),
        ];
        for (name, x, y) in cases {
            let a = g.mul(&x);
            let b = g.mul(&y);
            let aq = q(&a);
            let bq = q(&b);
            let reps = if degree < 16 {
                50_u128
            } else if degree <= 24 {
                16
            } else if degree < 32 {
                6
            } else {
                3
            };
            let now = Instant::now();
            let mut check = 0usize;
            for _ in 0..reps {
                let v = black_box(&a).gcd(black_box(&b)).unwrap();
                check ^= black_box(v.coefficients().len());
            }
            println!(
                "PRS domain=Z degree={degree} kind={name} ns={} check={check}",
                now.elapsed().as_nanos() / reps
            );
            let now = Instant::now();
            for _ in 0..reps {
                let v = black_box(&aq).gcd(black_box(&bq)).unwrap();
                check ^= black_box(v.coefficients().len());
            }
            println!(
                "PRS domain=Q degree={degree} kind={name} ns={} check={check}",
                now.elapsed().as_nanos() / reps
            );
        }
    }
}
