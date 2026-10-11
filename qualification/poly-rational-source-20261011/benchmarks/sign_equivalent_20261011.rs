//! Matched public-operation sign-equivalent GCD and self-division timing.
//! Run against both parent and candidate source; the fixture is identical.
use perfect_arithmetic::Integer;
use perfect_polynomial::{IntegerPolynomial as Z, RationalPolynomial as Q};
use perfect_rational::Rational;
use std::{hint::black_box, time::Instant};

fn z(degree: usize) -> Z {
    let mut coefficients = vec![Integer::zero(); degree + 1];
    coefficients[0] = Integer::from(12_i64);
    coefficients[1] = Integer::from(-6_i64);
    coefficients[degree / 2] = Integer::from(18_i64);
    coefficients[degree] = Integer::from(-3_i64);
    Z::new(coefficients)
}
fn r(n: i64, d: i64) -> Rational {
    Rational::new(Integer::from(n), Integer::from(d)).unwrap()
}
fn q(degree: usize) -> Q {
    let mut coefficients = vec![Rational::zero(); degree + 1];
    coefficients[0] = r(1, 3);
    coefficients[1] = r(-2, 5);
    coefficients[degree / 2] = r(3, 11);
    coefficients[degree] = r(-1, 7);
    Q::new(coefficients)
}
fn median(mut f: impl FnMut(), repetitions: usize) -> u128 {
    let mut runs = [0_u128; 3];
    for elapsed in &mut runs {
        let start = Instant::now();
        for _ in 0..repetitions {
            f();
        }
        *elapsed = start.elapsed().as_nanos() / repetitions as u128;
    }
    runs.sort_unstable();
    runs[1]
}
fn main() {
    for degree in [12_usize, 48] {
        let a = z(degree);
        let b = a.neg();
        assert_eq!(a.gcd(&b).unwrap(), b);
        println!(
            "z_gcd_neg_{degree}\t{}",
            median(
                || {
                    black_box(black_box(&a).gcd(black_box(&b)).unwrap());
                },
                3
            )
        );
        let a = q(degree);
        let b = a.neg();
        assert_eq!(a.gcd(&b).unwrap(), a.monic().unwrap());
        println!(
            "q_gcd_neg_{degree}\t{}",
            median(
                || {
                    black_box(black_box(&a).gcd(black_box(&b)).unwrap());
                },
                3
            )
        );
    }
    for degree in [32_usize, 128] {
        let a = z(degree);
        let b = a.neg();
        assert_eq!(a.div_exact(&b).unwrap(), Z::constant(Integer::from(-1_i64)));
        println!(
            "z_div_neg_{degree}\t{}",
            median(
                || {
                    black_box(black_box(&a).div_exact(black_box(&b)).unwrap());
                },
                8
            )
        );
        let a = q(degree);
        let b = a.neg();
        assert_eq!(
            a.div_rem(&b).unwrap(),
            (Q::constant(Rational::one().neg()), Q::zero())
        );
        println!(
            "q_div_neg_{degree}\t{}",
            median(
                || {
                    black_box(black_box(&a).div_rem(black_box(&b)).unwrap());
                },
                8
            )
        );
    }

    // Nonmatching same-degree controls expose extra comparison overhead.
    {
        let a = z(12);
        let mut terms = a.neg().coefficients().to_vec();
        terms[0] = terms[0].add(&Integer::one());
        let b = Z::new(terms);
        assert_ne!(a, b.neg());
        println!(
            "z_gcd_near_12\t{}",
            median(
                || {
                    black_box(black_box(&a).gcd(black_box(&b)).unwrap());
                },
                3
            )
        );
        let a = q(12);
        let mut terms = a.neg().coefficients().to_vec();
        terms[0] = terms[0].add(&r(1, 1));
        let b = Q::new(terms);
        assert_ne!(a, b.neg());
        println!(
            "q_gcd_near_12\t{}",
            median(
                || {
                    black_box(black_box(&a).gcd(black_box(&b)).unwrap());
                },
                3
            )
        );
    }
    {
        let a = z(32);
        let mut terms = a.neg().coefficients().to_vec();
        terms[0] = terms[0].add(&Integer::one());
        let b = Z::new(terms);
        assert_ne!(a, b.neg());
        println!(
            "z_div_near_32\t{}",
            median(
                || {
                    black_box(black_box(&a).div_exact(black_box(&b)).err());
                },
                8
            )
        );
        let a = q(32);
        let mut terms = a.neg().coefficients().to_vec();
        terms[0] = terms[0].add(&r(1, 1));
        let b = Q::new(terms);
        assert_ne!(a, b.neg());
        println!(
            "q_div_near_32\t{}",
            median(
                || {
                    black_box(black_box(&a).div_rem(black_box(&b)).unwrap());
                },
                8
            )
        );
    }
}
