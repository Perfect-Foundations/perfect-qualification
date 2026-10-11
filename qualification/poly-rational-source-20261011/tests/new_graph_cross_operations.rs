//! Independent exact Fraction vectors for the new four-crate source graph.
//! The product coefficients were computed using CPython fractions.Fraction.
use perfect_arithmetic::{Integer, Natural};
use perfect_polynomial::RationalPolynomial as Q;
use perfect_rational::Rational;

fn r(n: i64, d: i64) -> Rational {
    Rational::new(Integer::from(n), Integer::from(d)).expect("nonzero reference denominator")
}

#[test]
fn wide_zero_rational_flows_through_product_division_derivative_and_gcd() {
    let denominator = Integer::from(Natural::one().shl_bits(8192).add(&Natural::from(17_u8)));
    let zero = Rational::new(Integer::zero(), denominator.neg())
        .expect("zero over nonzero negative denominator");
    assert_eq!(zero, Rational::zero());
    assert_eq!(zero.denominator(), &Natural::one());

    let a = Q::new(vec![zero, r(1, 3), r(-2, 5), r(1, 1)]);
    let b = Q::new(vec![r(1, 7), r(-1, 3), r(1, 1)]);
    // Independent full polynomial convolution using Python Fraction.
    let expected = Q::new(vec![
        r(0, 1),
        r(1, 21),
        r(-53, 315),
        r(64, 105),
        r(-11, 15),
        r(1, 1),
    ]);
    let product = a.mul(&b);
    assert_eq!(product, expected);
    let (quotient, rem) = product.div_rem(&a).expect("nonzero divisor");
    assert_eq!(quotient, b);
    assert_eq!(rem, Q::zero());
    assert_eq!(a.gcd(&product).unwrap(), a.monic().unwrap());
    assert_eq!(a.derivative(), Q::new(vec![r(1, 3), r(-4, 5), r(3, 1)]));
}

#[test]
fn fused_integer_coefficient_cancellation_is_exact_at_wide_magnitude() {
    let wide = Integer::from(Natural::one().shl_bits(4096));
    let left = wide.add(&Integer::from(17_i64));
    let mut accumulator = Integer::from(3_i64);
    // 3*2 - (2^4096+17)*(-1) = 2^4096 + 23.
    accumulator.scale_sub_mul_assign(&Integer::from(2_i64), &left, &Integer::from(-1_i64));
    assert_eq!(accumulator, wide.add(&Integer::from(23_i64)));
    accumulator.sub_mul_assign(&left, &Integer::one());
    assert_eq!(accumulator, Integer::from(6_i64));
    assert_eq!(left.unsigned_bit_length(), 4097);
    assert_eq!(left.neg().unsigned_bit_length(), 4097);
}
