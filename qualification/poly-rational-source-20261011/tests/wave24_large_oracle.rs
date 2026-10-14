//! Independent unbounded Integer schoolbook oracle over optimized dispatch thresholds.
use perfect_arithmetic::{Integer, Natural};
use perfect_polynomial::{IntegerPolynomial as Z, PolynomialError};
fn polynomial(c: Vec<Integer>) -> Z {
    Z::new(c)
}
fn schoolbook(a: &[Integer], b: &[Integer]) -> Vec<Integer> {
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let mut result = vec![Integer::zero(); a.len() + b.len() - 1];
    for (i, x) in a.iter().enumerate() {
        for (j, y) in b.iter().enumerate() {
            result[i + j] = result[i + j].add(&x.mul(y));
        }
    }
    while result.last().is_some_and(Integer::is_zero) {
        result.pop();
    }
    result
}
fn wide(degree: usize, sparse: bool) -> Vec<Integer> {
    let large = Integer::from(Natural::one().shl_bits(513)).add(&Integer::from(17));
    (0..=degree)
        .map(|i| {
            if i == degree {
                return large.neg();
            }
            if i == 0 {
                return large.clone();
            }
            if sparse && i % 37 != 0 {
                return Integer::zero();
            }
            if i == degree / 2 {
                return large.mul(&Integer::from(3));
            }
            Integer::from((i as i64 * 17 + degree as i64 * 11) % 29 - 14)
        })
        .collect()
}
#[test]
fn degree_dispatch_and_signed_513bit_schoolbook_reference() {
    let mut cases = 0;
    for degree in [63, 64, 95, 96, 97, 127, 128, 256] {
        for sparse in [false, true] {
            let a = wide(degree, sparse);
            let b = wide(if sparse { degree } else { degree / 2 + 1 }, !sparse);
            let (pa, pb) = (polynomial(a.clone()), polynomial(b.clone()));
            let expected = polynomial(schoolbook(&a, &b));
            assert_eq!(
                pa.mul(&pb),
                expected,
                "degree={degree},sparse={sparse}: exact coefficient vector"
            );
            assert_eq!(pb.mul(&pa), expected);
            let derivative = polynomial(
                a.iter()
                    .enumerate()
                    .skip(1)
                    .map(|(i, c)| c.mul(&Integer::from(i)))
                    .collect(),
            );
            assert_eq!(pa.derivative(), derivative);
            assert_eq!(pa.coefficients().last(), Some(&a[degree]));
            cases += 1;
        }
    }
    assert_eq!(cases, 16);
}
#[test]
fn signed_monic_and_nonmonic_exact_division_oracle() {
    let factors = [
        polynomial(vec![Integer::from(-7), Integer::one()]),
        polynomial(vec![Integer::from(3), Integer::from(-5)]),
        polynomial(vec![Integer::from(-11), Integer::zero(), Integer::from(4)]),
    ];
    for degree in [63, 64, 95, 96, 97, 127] {
        let dividend_source = polynomial(wide(degree, true));
        for f in &factors {
            let exact = polynomial(schoolbook(dividend_source.coefficients(), f.coefficients()));
            assert_eq!(
                exact.div_exact(f).unwrap(),
                dividend_source,
                "degree={degree},divisor_degree={:?}",
                f.degree()
            );
            assert_eq!(exact.neg().div_exact(f).unwrap(), dividend_source.neg());
            assert_eq!(exact.div_exact(&f.neg()).unwrap(), dividend_source.neg());
        }
    }
    let two = polynomial(vec![Integer::from(2)]);
    assert_eq!(
        polynomial(vec![Integer::from(3)]).div_exact(&two),
        Err(PolynomialError::NonIntegralQuotient)
    );
    assert_eq!(
        polynomial(vec![Integer::one()]).div_exact(&Z::zero()),
        Err(PolynomialError::DivisionByZero)
    );
    assert_eq!(
        polynomial(vec![Integer::one()])
            .div_exact(&polynomial(vec![Integer::one(), Integer::one()])),
        Err(PolynomialError::NonExactDivision)
    );
}
