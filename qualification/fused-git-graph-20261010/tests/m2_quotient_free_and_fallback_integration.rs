//! Source-exact independent SymPy/Fraction cross-operation contracts.
use perfect_arithmetic::{Integer, Natural};
use perfect_polynomial::{IntegerPolynomial as Z, PolynomialError, RationalPolynomial as Q};
use perfect_rational::Rational;

fn z(v: &[i64]) -> Q {
    Q::new(
        v.iter()
            .map(|&n| Rational::from(Integer::from(n)))
            .collect(),
    )
}
fn f(n: i64, d: i64) -> Rational {
    Rational::new(Integer::from(n), Integer::from(d)).unwrap()
}

#[test]
fn independent_sympy_gcd_after_fused_and_field_quotient_free_remainders() {
    // SymPy: H=(x+1)^2, U=x^17+2x+1, U(-2)=-131075 != 0.
    // gcd(HU, H(x+2))=H, even when the dividend contains B*Q.
    let h = z(&[1, 2, 1]);
    let mut u = vec![Rational::zero(); 18];
    u[0] = Rational::one();
    u[1] = Rational::from(Integer::from(2));
    u[17] = Rational::one();
    let b = h.mul(&Q::new(u));
    let r = h.mul(&z(&[2, 1]));
    for steps in [40usize, 170usize] {
        let quotient = Q::new(
            (0..steps)
                .map(|i| f(if i % 3 == 0 { 2 } else { -1 }, 7))
                .collect(),
        );
        let a = b.mul(&quotient).add(&r);
        let (actual_q, actual_r) = a.div_rem(&b).unwrap();
        assert_eq!(actual_q, quotient);
        assert_eq!(actual_r, r);
        assert_eq!(b.mul(&actual_q).add(&actual_r), a);
        assert!(actual_r.degree().unwrap() < b.degree().unwrap());
        assert_eq!(a.gcd(&b).unwrap(), h);
        assert_eq!(b.gcd(&a).unwrap(), h);
    }
}

#[test]
fn reconstruct_signed_large_integer_zero_gcd_and_typed_fallback() {
    let leading = Integer::from(Natural::one().shl_bits(512)).add(&Integer::from(17));
    let primitive = Z::new(vec![
        Integer::one(),
        Integer::from(-3),
        Integer::zero(),
        Integer::from(5),
        leading.clone(),
    ]);
    let negative = primitive.mul(&Z::constant(Integer::from(-21)));
    let expected = primitive.mul(&Z::constant(Integer::from(21)));
    assert_eq!(negative.gcd(&Z::zero()).unwrap(), expected);
    assert_eq!(Z::zero().gcd(&negative).unwrap(), expected);

    let two_x = Z::new(vec![Integer::zero(), Integer::from(2)]);
    assert_eq!(
        Z::new(vec![Integer::zero(), leading.clone()]).div_exact(&two_x),
        Err(PolynomialError::NonIntegralQuotient)
    );
    assert_eq!(
        Z::new(vec![Integer::one(), leading]).div_exact(&two_x),
        Err(PolynomialError::NonExactDivision)
    );
}

#[test]
fn sparse_fraction_multiply_then_exact_divide_and_monic_gcd() {
    let a = Q::new(vec![
        f(1, 2),
        f(0, 1),
        f(0, 1),
        f(0, 1),
        f(3, 5),
        f(0, 1),
        f(0, 1),
        f(0, 1),
        f(1, 7),
    ]);
    let b = Q::new(vec![
        f(-2, 3),
        f(0, 1),
        f(0, 1),
        f(5, 11),
        f(0, 1),
        f(0, 1),
        f(0, 1),
        f(2, 13),
    ]);
    let product = a.mul(&b);
    let (quotient, remainder) = product.div_rem(&b).unwrap();
    assert_eq!(quotient, a);
    assert!(remainder.is_zero());
    assert_eq!(product.gcd(&b).unwrap(), b.monic().unwrap());
    assert_eq!(b.gcd(&product).unwrap(), b.monic().unwrap());
}
