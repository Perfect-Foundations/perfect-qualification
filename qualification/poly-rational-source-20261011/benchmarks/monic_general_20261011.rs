//! Matched public-operation monic sparse/dense division and GCD benchmark.
//! Source-precise A/B/B/A execution is in verification/benchmark_monic_general_20261011.py.
use perfect_arithmetic::Integer;
use perfect_polynomial::{IntegerPolynomial as Z, RationalPolynomial as Q};
use perfect_rational::Rational;
use std::{hint::black_box, time::Instant};

fn r(n: i64) -> Rational {
    Rational::from(Integer::from(n))
}
fn bench(mut f: impl FnMut(), repetitions: usize) -> u128 {
    // Longer timed batches reduce Windows scheduling noise on short paths.
    let repetitions = repetitions * 16;
    let mut nanos = [0_u128; 3];
    for time in &mut nanos {
        let start = Instant::now();
        for _ in 0..repetitions {
            f();
        }
        *time = start.elapsed().as_nanos() / repetitions as u128;
    }
    nanos.sort_unstable();
    nanos[1]
}

fn division_case(degree: usize, qlen: usize, dense: bool, lead: i64) -> (Q, Q, Q, Q) {
    let mut coefficients = vec![Rational::zero(); degree + 1];
    coefficients[0] = r(2);
    coefficients[degree / 3] = r(3);
    coefficients[degree - 2] = r(-2);
    if dense {
        for (i, coefficient) in coefficients.iter_mut().enumerate().take(degree).skip(1) {
            *coefficient = r((i % 5) as i64 - 2);
        }
    }
    coefficients[degree] = r(lead);
    let b = Q::new(coefficients);
    let mut quot = vec![Rational::zero(); qlen];
    for (i, c) in quot.iter_mut().enumerate() {
        *c = r((i % 7) as i64 - 3);
    }
    quot[qlen - 1] = r(1);
    let q = Q::new(quot);
    let rem = Q::new(vec![r(1)]);
    let a = b.mul(&q).add(&rem);
    assert_eq!(a.div_rem(&b), Ok((q.clone(), rem.clone())));
    (a, b, q, rem)
}
fn gcd_case(degree: usize) -> (Z, Z, Z) {
    let g = Z::new(vec![
        Integer::from(1),
        Integer::from(2),
        Integer::zero(),
        Integer::one(),
    ]);
    let mut qa = vec![Integer::zero(); degree - 2];
    qa[0] = Integer::one();
    qa[degree - 3] = Integer::one();
    let mut qb = vec![Integer::zero(); degree - 3];
    qb[0] = Integer::one();
    qb[degree - 4] = Integer::one();
    let a = g.mul(&Z::new(qa));
    let b = g.mul(&Z::new(qb));
    assert_eq!(a.gcd(&b), Ok(g.clone()));
    (a, b, g)
}
fn main() {
    for (name, degree, qlen, dense, lead, reps) in [
        ("q_div_monic_sparse_64", 64, 48, false, 1, 8),
        ("q_div_monic_sparse_96", 96, 48, false, 1, 6),
        ("q_div_monic_dense_64", 64, 48, true, 1, 5),
        ("q_div_nonmonic_sparse_64", 64, 32, false, 2, 5),
        ("q_div_nonmonic_dense_64", 64, 32, true, 2, 5),
    ] {
        let (a, b, q, rem) = division_case(degree, qlen, dense, lead);
        println!(
            "{name}\t{}",
            bench(
                || {
                    let (actual_q, actual_r) = black_box(&a).div_rem(black_box(&b)).unwrap();
                    debug_assert_eq!(actual_q, q);
                    debug_assert_eq!(actual_r, rem);
                    black_box((actual_q, actual_r));
                },
                reps
            )
        );
    }
    for (name, degree, reps) in [
        ("z_gcd_monic_sparse_48", 48, 3),
        ("z_gcd_monic_sparse_80", 80, 3),
        ("z_gcd_monic_sparse_96", 96, 3),
    ] {
        let (a, b, g) = gcd_case(degree);
        println!(
            "{name}\t{}",
            bench(
                || {
                    let gcd = black_box(&a).gcd(black_box(&b)).unwrap();
                    debug_assert_eq!(gcd, g);
                    black_box(gcd);
                },
                reps
            )
        );
    }

    // Large sparse products avoid the O(n*m) zero-probe schoolbook loop.
    // Dense pairs are retained as crossover controls.
    for (name, na, nb, sparse_left, sparse_right, reps) in [
        (
            "z_mul_sparse_256x256",
            256_usize,
            256_usize,
            false,
            true,
            6_usize,
        ),
        ("z_mul_sparse_512x256", 512, 256, false, true, 5),
        ("z_mul_sparse_1024x128", 1024, 128, true, false, 4),
        ("z_mul_dense_256x256", 256, 256, false, false, 4),
        ("z_mul_dense_128x128", 128, 128, false, false, 5),
    ] {
        let operand = |n: usize, sparse: bool| {
            let mut coefficients = (0..n)
                .map(|i| {
                    if sparse {
                        Integer::zero()
                    } else {
                        Integer::from((i % 13) as i64 - 6)
                    }
                })
                .collect::<Vec<_>>();
            if sparse {
                coefficients[0] = Integer::from(3);
                coefficients[n / 2] = Integer::from(-7);
                coefficients[n - 1] = Integer::from(5);
            } else {
                coefficients[n - 1] = Integer::one();
            }
            Z::new(coefficients)
        };
        let a = operand(na, sparse_left);
        let b = operand(nb, sparse_right);
        let expected = a.mul(&b);
        println!(
            "{name}\t{}",
            bench(
                || {
                    let output = black_box(&a).mul(black_box(&b));
                    debug_assert_eq!(output, expected);
                    black_box(output);
                },
                reps
            )
        );
    }
    {
        let mut a = vec![Rational::zero(); 129];
        let mut b = vec![Rational::zero(); 257];
        a[0] = Rational::new(Integer::one(), Integer::from(2)).unwrap();
        a[64] = Rational::new(Integer::from(-7), Integer::from(3)).unwrap();
        a[128] = Rational::one();
        b[0] = Rational::new(Integer::from(2), Integer::from(5)).unwrap();
        b[85] = Rational::new(Integer::from(-11), Integer::from(7)).unwrap();
        b[256] = Rational::new(Integer::from(3), Integer::from(11)).unwrap();
        let left = Q::new(a);
        let right = Q::new(b);
        let expected = left.mul(&right);
        println!(
            "q_mul_sparse_mixed_129x257\t{}",
            bench(
                || {
                    let output = black_box(&left).mul(black_box(&right));
                    debug_assert_eq!(output, expected);
                    black_box(output);
                },
                5
            )
        );
    }
}
