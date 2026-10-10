//! Deterministic public-API differential tests with an independent i128 oracle.
//! Inputs are deliberately bounded so the oracle cannot overflow i128.
use perfect_arithmetic::Integer;
use perfect_polynomial::{IntegerPolynomial as Z, PolynomialError};

struct Generator(u64);

impl Generator {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1);
        self.0
    }

    fn coefficients(&mut self, max_len: usize) -> Vec<i128> {
        let len = (self.next() as usize) % (max_len + 1);
        (0..len)
            .map(|_| i128::from((self.next() % 23) as i16) - 11)
            .collect()
    }
}

fn polynomial(a: &[i128]) -> Z {
    Z::new(a.iter().copied().map(Integer::from).collect())
}

fn normalize(mut a: Vec<i128>) -> Vec<i128> {
    while a.last() == Some(&0) {
        a.pop();
    }
    a
}

fn oracle_add(a: &[i128], b: &[i128], subtract: bool) -> Vec<i128> {
    let mut result = vec![0; a.len().max(b.len())];
    for (index, &value) in a.iter().enumerate() {
        result[index] += value;
    }
    for (index, &value) in b.iter().enumerate() {
        if subtract {
            result[index] -= value;
        } else {
            result[index] += value;
        }
    }
    normalize(result)
}

fn oracle_mul(a: &[i128], b: &[i128]) -> Vec<i128> {
    if a.is_empty() || b.is_empty() {
        return Vec::new();
    }
    let mut result = vec![0; a.len() + b.len() - 1];
    for (i, &left) in a.iter().enumerate() {
        for (j, &right) in b.iter().enumerate() {
            result[i + j] += left * right;
        }
    }
    normalize(result)
}

fn oracle_derivative(a: &[i128]) -> Vec<i128> {
    normalize(
        a.iter()
            .enumerate()
            .skip(1)
            .map(|(index, &value)| (index as i128) * value)
            .collect(),
    )
}

fn oracle_evaluate(a: &[i128], x: i128) -> i128 {
    a.iter()
        .rev()
        .fold(0i128, |sum, &coefficient| sum * x + coefficient)
}

#[test]
fn seeded_public_integer_operations_match_independent_i128_oracle() {
    const SEED: u64 = 0xB1A2_1C3D_4E5F_6789;
    let mut generator = Generator(SEED);
    for case in 0..1536 {
        let a = generator.coefficients(8);
        let b = generator.coefficients(8);
        let left = polynomial(&a);
        let right = polynomial(&b);
        let canonical_a = normalize(a.clone());

        assert_eq!(left, polynomial(&canonical_a), "case {case}: canonical");
        assert_eq!(
            left.degree(),
            canonical_a.len().checked_sub(1),
            "case {case}: degree"
        );
        assert_eq!(
            left.add(&right),
            polynomial(&oracle_add(&a, &b, false)),
            "case {case}: add"
        );
        assert_eq!(
            left.sub(&right),
            polynomial(&oracle_add(&a, &b, true)),
            "case {case}: sub"
        );
        assert_eq!(
            left.mul(&right),
            polynomial(&oracle_mul(&a, &b)),
            "case {case}: mul"
        );
        assert_eq!(
            left.derivative(),
            polynomial(&oracle_derivative(&a)),
            "case {case}: derivative"
        );

        let x = i128::from((generator.next() % 9) as i8) - 4;
        assert_eq!(
            left.evaluate(&Integer::from(x)),
            Integer::from(oracle_evaluate(&a, x)),
            "case {case}: evaluation"
        );
    }
}

#[test]
fn seeded_public_exact_division_and_gcd_preserve_factor_invariants() -> Result<(), PolynomialError>
{
    const SEED: u64 = 0xDA7A_43FD_1234_BEEF;
    let mut generator = Generator(SEED);
    for case in 0..192 {
        let mut factor_coeffs = generator.coefficients(3);
        if normalize(factor_coeffs.clone()).is_empty() {
            factor_coeffs = vec![1];
        }
        let factor = polynomial(&factor_coeffs);
        let a = polynomial(&generator.coefficients(3));
        let b = polynomial(&generator.coefficients(3));
        let left = factor.mul(&a);
        let right = factor.mul(&b);

        assert_eq!(left.div_exact(&factor)?, a, "case {case}: exact left");
        assert_eq!(right.div_exact(&factor)?, b, "case {case}: exact right");
        let gcd = left.gcd(&right)?;
        assert_eq!(gcd, right.gcd(&left)?, "case {case}: gcd symmetry");
        if !gcd.is_zero() {
            let quotient_left = left.div_exact(&gcd)?;
            let quotient_right = right.div_exact(&gcd)?;
            assert_eq!(quotient_left.mul(&gcd), left, "case {case}: left divisible");
            assert_eq!(
                quotient_right.mul(&gcd),
                right,
                "case {case}: right divisible"
            );
        } else {
            assert!(left.is_zero() && right.is_zero(), "case {case}: zero gcd");
        }
    }
    Ok(())
}
