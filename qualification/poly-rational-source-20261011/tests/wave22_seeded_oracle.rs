//! Seeded external-coefficient oracle for the optimized Z[x] engine.
//! The i128 reference is schoolbook coefficient arithmetic, not a polynomial
//! implementation or a call into production kernels.
use perfect_arithmetic::Integer;
use perfect_polynomial::{IntegerPolynomial as Z, RationalPolynomial as Q};
use perfect_rational::Rational;
const SEED: u64 = 0x22a19d037bce4411;
fn rand(s: &mut u64) -> u64 {
    *s ^= *s >> 12;
    *s ^= *s << 25;
    *s ^= *s >> 27;
    s.wrapping_mul(0x2545f4914f6cdd1d)
}
fn norm(mut c: Vec<i128>) -> Vec<i128> {
    while c.last() == Some(&0) {
        c.pop();
    }
    c
}
fn z(a: &[i128]) -> Z {
    Z::new(a.iter().map(|&v| Integer::from(v as i64)).collect())
}
fn coefficient_oracle(a: &[i128], b: &[i128]) -> Vec<i128> {
    if a.is_empty() || b.is_empty() {
        return Vec::new();
    }
    let mut r = vec![0; a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        for (j, &y) in b.iter().enumerate() {
            r[i + j] += x * y;
        }
    }
    norm(r)
}
#[test]
fn seeded_512_exact_integer_products_and_derivatives() {
    let mut state = SEED;
    const SIZES: &[usize] = &[0, 1, 2, 3, 7, 16, 47, 48, 64, 127, 128, 129, 256];
    for case in 0..512 {
        let n = SIZES[(rand(&mut state) as usize) % SIZES.len()];
        let m = SIZES[(rand(&mut state) as usize) % SIZES.len()];
        let a = norm(
            (0..n)
                .map(|i| {
                    let x = (rand(&mut state) % 17) as i128 - 8;
                    if case % 3 == 0 && i % 11 != 0 && i + 1 != n {
                        0
                    } else {
                        x
                    }
                })
                .collect(),
        );
        let b = norm(
            (0..m)
                .map(|i| {
                    let x = (rand(&mut state) % 13) as i128 - 6;
                    if case % 4 == 0 && i % 13 != 0 && i + 1 != m {
                        0
                    } else {
                        x
                    }
                })
                .collect(),
        );
        let (p, q) = (z(&a), z(&b));
        let reference = coefficient_oracle(&a, &b);
        assert_eq!(p.mul(&q), z(&reference), "seed={SEED:x},case={case}");
        let derivative = norm(
            a.iter()
                .enumerate()
                .skip(1)
                .map(|(i, &v)| i as i128 * v)
                .collect(),
        );
        assert_eq!(p.derivative(), z(&derivative), "derivative case={case}");
        if !q.is_zero() {
            assert_eq!(p.mul(&q).div_exact(&q).unwrap(), p, "division case={case}");
        }
        for x in [-1_i64, 0, 1] {
            let mut expected = 0_i128;
            let mut power = 1_i128;
            for &v in &a {
                expected += v * power;
                power *= x as i128;
            }
            assert_eq!(
                p.evaluate(&Integer::from(x)),
                Integer::from(expected as i64)
            );
        }
    }
}
#[test]
fn seeded_128_rational_gcd_and_field_division_properties() {
    let mut state = SEED ^ 0x71aabb32;
    for case in 0..128 {
        let mut make = |n: usize| {
            Q::new(
                (0..n)
                    .map(|_| {
                        let numerator = (rand(&mut state) % 11) as i64 - 5;
                        let denominator = (rand(&mut state) % 5 + 1) as i64;
                        Rational::new(Integer::from(numerator), Integer::from(denominator)).unwrap()
                    })
                    .collect(),
            )
        };
        let factor = make(2 + case % 3);
        let a = make(1 + case % 5);
        let b = make(1 + (case + 1) % 5);
        let dividend = factor.mul(&a);
        let (quotient, remainder) = dividend.div_rem(&factor).unwrap();
        assert_eq!(quotient, a, "rational quotient case={case}");
        assert!(remainder.is_zero(), "rational remainder case={case}");
        let g = dividend.gcd(&factor.mul(&b)).unwrap();
        assert_eq!(g.coefficients().last(), Some(&Rational::one()));
        assert!(dividend.div_rem(&g).unwrap().1.is_zero());
        assert!(factor.mul(&b).div_rem(&g).unwrap().1.is_zero());
        assert_eq!(
            a.mul(&b).derivative(),
            a.derivative().mul(&b).add(&a.mul(&b.derivative()))
        );
    }
}
