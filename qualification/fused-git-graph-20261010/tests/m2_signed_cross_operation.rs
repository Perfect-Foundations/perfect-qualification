//! Independent closed-form signed wide-coefficient cross-operation qualification.
//! Python integer algebra expands (x+W)^2(x+2) and (x+W)(x-3).
use perfect_arithmetic::{Integer, Natural};
use perfect_polynomial::{IntegerPolynomial as Z, PolynomialError};

fn z(coeffs: Vec<Integer>) -> Z {
    Z::new(coeffs)
}

#[test]
fn high_width_repeated_factor_gcd_and_exact_division() {
    let w = Integer::from(Natural::one().shl_bits(192)).add(&Integer::from(5));
    let h = z(vec![w.clone(), 1.into()]);
    let u = z(vec![2.into(), 1.into()]);
    let v = z(vec![(-3).into(), 1.into()]);
    let w2 = w.mul(&w);
    // A=2W² + (W²+4W)x + (2W+2)x² + x³
    let a = z(vec![
        w2.mul(&Integer::from(2)),
        w2.add(&w.mul(&Integer::from(4))),
        w.mul(&Integer::from(2)).add(&Integer::from(2)),
        1.into(),
    ]);
    // B=-3W + (W-3)x + x²
    let b = z(vec![
        w.mul(&Integer::from(-3)),
        w.sub(&Integer::from(3)),
        1.into(),
    ]);
    assert_eq!(h.mul(&h).mul(&u), a);
    assert_eq!(h.mul(&v), b);
    assert_eq!(a.gcd(&b).unwrap(), h);
    assert_eq!(b.gcd(&a).unwrap(), h);
    assert_eq!(a.div_exact(&h).unwrap(), h.mul(&u));
    assert_eq!(b.div_exact(&h).unwrap(), v);
    assert_eq!(a.div_exact(&h.mul(&u)).unwrap(), h);
    assert_eq!(a.div_exact(&h.mul(&h)).unwrap(), u);
    assert_eq!(
        a.div_exact(&Z::zero()),
        Err(PolynomialError::DivisionByZero)
    );
}

#[test]
fn later_fractional_quotient_with_zero_remainder_is_not_nonexact() {
    // (1+3x+2x²)/(2+2x) = 1/2+x, exactly over Q[x],
    // but not exactly divisible over Z[x].
    let a = z(vec![1.into(), 3.into(), 2.into()]);
    let b = z(vec![2.into(), 2.into()]);
    assert_eq!(a.div_exact(&b), Err(PolynomialError::NonIntegralQuotient));
    // A one-unit change creates a genuinely nonzero field remainder.
    assert_eq!(
        z(vec![2.into(), 3.into(), 2.into()]).div_exact(&b),
        Err(PolynomialError::NonExactDivision)
    );
}
