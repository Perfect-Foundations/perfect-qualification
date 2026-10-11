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

#[test]
fn signed_unit_multiplication_integral_products_and_horner_match_fraction() {
    let p = Q::new(vec![r(1, 2), r(2, 3), r(-3, 5)]);
    let minus_one = Q::constant(r(-1, 1));
    let negated = p.mul(&minus_one);
    // Python Fraction: all product coefficients are the exact negatives.
    assert_eq!(negated, Q::new(vec![r(-1, 2), r(-2, 3), r(3, 5)]));
    // Fraction polynomial at x=5/3: -1/18 before negation, +1/18 after.
    assert_eq!(p.evaluate(&r(5, 3)), r(-1, 18));
    assert_eq!(negated.evaluate(&r(5, 3)), r(1, 18));
    let (q, rem) = negated.div_rem(&minus_one).unwrap();
    assert_eq!(q, p);
    assert_eq!(rem, Q::zero());
    // Python integer convolution: [23,-31] * [-31] = [-713,961].
    let ints = Q::new(vec![r(23, 1), r(-31, 1)]);
    let result = ints.mul(&Q::constant(r(-31, 1)));
    assert_eq!(result, Q::new(vec![r(-713, 1), r(961, 1)]));
}

#[test]
fn subtraction_and_monomial_remainder_have_independent_fraction_values() {
    let a = r(5, 6);
    let b = r(7, 10);
    // Python Fraction(5,6) - Fraction(7,10) == Fraction(2,15).
    assert_eq!(a.sub(&b), r(2, 15));
    assert_eq!(a.sub(&a), Rational::zero());
    let mut coefficients = vec![Rational::zero(); 26];
    coefficients[0] = r(1, 3);
    coefficients[3] = r(-2, 5);
    coefficients[4] = r(4, 9);
    coefficients[14] = r(-5, 13);
    coefficients[25] = r(7, 11);
    let a = Q::new(coefficients);
    let b = Q::new(vec![
        Rational::zero(),
        Rational::zero(),
        Rational::zero(),
        Rational::zero(),
        r(-3, 7),
    ]);
    let (q, rem) = a.div_rem(&b).unwrap();
    let mut reference = vec![Rational::zero(); 22];
    reference[0] = r(-28, 27);
    reference[10] = r(35, 39);
    reference[21] = r(-49, 33);
    assert_eq!(q, Q::new(reference));
    assert_eq!(
        rem,
        Q::new(vec![r(1, 3), Rational::zero(), Rational::zero(), r(-2, 5)])
    );
}

#[test]
fn dense_integer_convolution_uses_independent_triangular_coefficients() {
    use perfect_polynomial::IntegerPolynomial as Z;
    let a = Z::new((1..=64).map(Integer::from).collect());
    let b = Z::new(vec![Integer::one(); 64]);
    let product = a.mul(&b);
    assert_eq!(product.coefficients().len(), 127);
    // CPython arbitrary integer convolution: coefficient[k] is the sum
    // of 1-based indices in the valid finite length-64 window.
    for k in 0..127usize {
        let lower = k.saturating_sub(63);
        let upper = k.min(63);
        let sum = ((upper + 1) * (upper + 2) - lower * (lower + 1)) / 2;
        assert_eq!(product.coefficients()[k], Integer::from(sum), "degree={k}");
    }
}

#[test]
fn high_degree_sparse_field_divisor_matches_independent_fraction_vectors() {
    // Independent CPython Fraction expansion for:
    // (x^256 + 1/3) * (2*x^200 - 5*x^13/2 + 1/7) + (-x^80 + 1/11).
    let mut divisor = vec![Rational::zero(); 257];
    divisor[0] = r(1, 3);
    divisor[256] = Rational::one();
    let divisor = Q::new(divisor);

    let mut expected_q = vec![Rational::zero(); 201];
    expected_q[0] = r(1, 7);
    expected_q[13] = r(-5, 2);
    expected_q[200] = r(2, 1);
    let expected_q = Q::new(expected_q);

    let mut expected_r = vec![Rational::zero(); 81];
    expected_r[0] = r(1, 11);
    expected_r[80] = r(-1, 1);
    let expected_r = Q::new(expected_r);

    let mut dividend = vec![Rational::zero(); 457];
    for (i, n, d) in [
        (0, 32, 231),
        (13, -5, 6),
        (80, -1, 1),
        (200, 2, 3),
        (256, 1, 7),
        (269, -5, 2),
        (456, 2, 1),
    ] {
        dividend[i] = r(n, d);
    }
    let actual = Q::new(dividend).div_rem(&divisor).unwrap();
    assert_eq!(actual, (expected_q, expected_r));
}

#[test]
fn integral_fraction_cancellation_matches_python_fraction_at_wide_width() {
    let denominator = Integer::from(Natural::one().shl_bits(4096).add(&Natural::from(19_u8)));
    let fractional = Rational::new(Integer::from(29_i64), denominator.clone()).unwrap();
    let integral = Rational::from(denominator.clone());
    // Python Fraction(D,1) * Fraction(29,D) = Fraction(29,1).
    assert_eq!(
        integral.mul(&fractional),
        Rational::from(Integer::from(29_i64))
    );
    assert_eq!(
        fractional.mul(&integral.neg()),
        Rational::from(Integer::from(-29_i64))
    );
    let numerator = Integer::from(Natural::one().shl_bits(4096).add(&Natural::from(17_u8)));
    let n = Rational::from(numerator.clone());
    // Independently specified Fraction(N*29,D) with canonical sign and reduction.
    let reference = Rational::new(numerator.mul(&Integer::from(29_i64)), denominator).unwrap();
    assert_eq!(n.mul(&fractional), reference);
    assert_eq!(fractional.mul(&n), reference);
}

#[test]
fn integer_monomial_coefficients_match_shifted_integer_reference() {
    use perfect_polynomial::IntegerPolynomial as Z;
    let high = Integer::from(Natural::one().shl_bits(256));
    let a = Z::new(
        (0..96usize)
            .map(|i| {
                let value = high.add(&Integer::from((i + 5) as i64));
                if i.is_multiple_of(3) {
                    value.neg()
                } else {
                    value
                }
            })
            .collect(),
    );
    let mut monomial = vec![Integer::zero(); 37];
    monomial.push(Integer::from(-17_i64));
    let b = Z::new(monomial);
    let mut reference = vec![Integer::zero(); 37];
    reference.extend(
        a.coefficients()
            .iter()
            .map(|c| c.mul(&Integer::from(-17_i64))),
    );
    let expected = Z::new(reference);
    assert_eq!(a.mul(&b), expected);
    assert_eq!(b.mul(&a), expected);
}
