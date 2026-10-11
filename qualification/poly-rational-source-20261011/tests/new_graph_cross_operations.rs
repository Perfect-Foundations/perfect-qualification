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

#[test]
fn sparse_integer_nonmonomial_division_matches_independent_exact_vectors() {
    use perfect_polynomial::{IntegerPolynomial as Z, PolynomialError};
    let from_terms = |length: usize, terms: &[(usize, i64)]| {
        let mut coefficients = vec![Integer::zero(); length];
        for &(degree, value) in terms {
            coefficients[degree] = Integer::from(value);
        }
        Z::new(coefficients)
    };
    // Independent integer convolution:
    // (x^256+3)*(2*x^200-5*x^13+7)
    // = 2*x^456-5*x^269+7*x^256+6*x^200-15*x^13+21.
    let divisor = from_terms(257, &[(0, 3), (256, 1)]);
    let expected = from_terms(201, &[(0, 7), (13, -5), (200, 2)]);
    let dividend = from_terms(
        457,
        &[(0, 21), (13, -15), (200, 6), (256, 7), (269, -5), (456, 2)],
    );
    assert_eq!(dividend.div_exact(&divisor), Ok(expected));

    // Exactly one-half in Q[x], but not in Z[x].
    let twice = from_terms(65, &[(0, 2), (64, 2)]);
    let half = from_terms(65, &[(0, 1), (64, 1)]);
    assert_eq!(
        half.div_exact(&twice),
        Err(PolynomialError::NonIntegralQuotient)
    );
    // Nonzero remainder must take precedence.
    let nonexact = from_terms(66, &[(0, 1), (65, 1)]);
    assert_eq!(
        nonexact.div_exact(&twice),
        Err(PolynomialError::NonExactDivision)
    );
}

#[test]
fn constant_polynomial_gcd_uses_exact_content_and_field_units() {
    use perfect_polynomial::IntegerPolynomial as Z;

    // Python/SymPy: gcd_Z(6*x^128 + 18*x^64 - 12, -18) = 6,
    // independent of sign, order, or sparse coefficient storage.
    let mut cs = vec![Integer::zero(); 129];
    cs[0] = Integer::from(-12_i64);
    cs[64] = Integer::from(18_i64);
    cs[128] = Integer::from(6_i64);
    let a = Z::new(cs);
    let c = Z::constant(Integer::from(-18_i64));
    let expected = Z::constant(Integer::from(6_i64));
    assert_eq!(a.gcd(&c), Ok(expected.clone()));
    assert_eq!(c.gcd(&a), Ok(expected));

    // Over Q[x] any nonzero rational constant is a unit; gcd is monic 1.
    let p = Q::new(vec![r(3, 7), r(-5, 11), r(1, 1)]);
    let unit = Q::constant(r(-7, 11));
    let one = Q::constant(Rational::one());
    assert_eq!(p.gcd(&unit), Ok(one.clone()));
    assert_eq!(unit.gcd(&p), Ok(one.clone()));
    assert_eq!(unit.gcd(&unit), Ok(one));
    assert_eq!(Q::zero().gcd(&Q::zero()), Ok(Q::zero()));
}

#[test]
fn identical_nonconstant_integer_and_rational_gcd_have_independent_canonical_coefficients() {
    use perfect_polynomial::IntegerPolynomial as Z;

    // In Z[x], gcd(p,p) retains full integer content, with positive leading sign.
    let negative = Z::new(vec![
        Integer::from(12),
        Integer::from(-18),
        Integer::from(-6),
    ]);
    let positive = Z::new(vec![
        Integer::from(-12),
        Integer::from(18),
        Integer::from(6),
    ]);
    assert_eq!(negative.gcd(&negative), Ok(positive));

    // In Q[x], gcd(p,p) is monic, exactly p / (-5/7).
    // Independently calculated with Python fractions.Fraction.
    let fraction = Q::new(vec![r(3, 10), r(-2, 3), r(-5, 7)]);
    let expected = Q::new(vec![r(-21, 50), r(14, 15), Rational::one()]);
    assert_eq!(fraction.gcd(&fraction), Ok(expected));

    // Nonidentical operands must not select the identical-input shortcut.
    let a = Z::new(vec![Integer::one(), Integer::from(2), Integer::one()]);
    let b = Z::new(vec![Integer::from(2), Integer::from(2), Integer::one()]);
    assert_eq!(a.gcd(&b), Ok(Z::constant(Integer::one())));

    // Equality above the bounded PRS degree cap remains exact.
    let mut high = vec![Integer::zero(); 65];
    high[0] = Integer::from(12);
    high[64] = Integer::from(-6);
    let high = Z::new(high);
    assert_eq!(high.gcd(&high), Ok(high.neg()));
}

#[test]
fn degree48_nonidentical_prs_matches_independent_sympy_gcd() {
    use perfect_polynomial::IntegerPolynomial as Z;
    fn polynomial(degree: usize, nonzero: &[(usize, i64)]) -> Z {
        let mut coefficients = vec![Integer::zero(); degree + 1];
        coefficients[degree] = Integer::one();
        for &(degree, value) in nonzero {
            coefficients[degree] = Integer::from(value);
        }
        Z::new(coefficients)
    }
    fn as_q(poly: &Z) -> Q {
        Q::new(
            poly.coefficients()
                .iter()
                .cloned()
                .map(Rational::from)
                .collect(),
        )
    }
    // Independent SymPy ZZ oracle for degree 36,40,48:
    // gcd((x^2+1)*(x^(n-2)+x+2), (x^2+1)*(x^(n-3)+3*x^2+1)) = x^2+1.
    let g = polynomial(2, &[(0, 1)]);
    for degree in [36_usize, 40, 48] {
        let a = g.mul(&polynomial(degree - 2, &[(0, 2), (1, 1)]));
        let b = g.mul(&polynomial(degree - 3, &[(0, 1), (2, 3)]));
        assert_ne!(a, b);
        assert_eq!(a.gcd(&b), Ok(g.clone()), "Z[x] degree {degree}");
        assert_eq!(b.gcd(&a), Ok(g.clone()), "reverse Z[x] {degree}");
        assert_eq!(as_q(&a).gcd(&as_q(&b)), Ok(as_q(&g)), "Q[x] {degree}");
    }
}

#[test]
fn degree65_nonidentical_exact_fallback_keeps_independent_gcd() {
    use perfect_polynomial::IntegerPolynomial as Z;
    // Independently verified by SymPy:
    // gcd((x²+1)(x^63+x+2), (x²+1)(x^62+3x²+1)) = x²+1.
    let g = Z::new(vec![Integer::one(), Integer::zero(), Integer::one()]);
    let mut first = vec![Integer::zero(); 64];
    first[0] = Integer::from(2_i64);
    first[1] = Integer::one();
    first[63] = Integer::one();
    let mut second = vec![Integer::zero(); 63];
    second[0] = Integer::one();
    second[2] = Integer::from(3_i64);
    second[62] = Integer::one();
    let a = g.mul(&Z::new(first));
    let b = g.mul(&Z::new(second));
    assert_eq!(a.degree(), Some(65));
    assert_eq!(a.gcd(&b), Ok(g.clone()));
    let rational = |p: &Z| {
        Q::new(
            p.coefficients()
                .iter()
                .cloned()
                .map(Rational::from)
                .collect(),
        )
    };
    assert_eq!(rational(&a).gcd(&rational(&b)), Ok(rational(&g)));
}

fn sympy_certified_degree64_products(
    degree: usize,
) -> (
    perfect_polynomial::IntegerPolynomial,
    perfect_polynomial::IntegerPolynomial,
) {
    use perfect_polynomial::IntegerPolynomial as Z;
    // Independent full sparse coefficient expansions for:
    // A=(x^3+2x+1)*(x^(degree-3)+2x^2+1)
    // B=(x^3+2x+1)*(x^(degree-4)+3x+1)
    // SymPy gcd_ZZ(A,B)=x^3+2x+1 for degrees 49,56,64,65.
    let mut left = vec![Integer::zero(); degree + 1];
    for (index, value) in [
        (0, 1),
        (1, 2),
        (2, 2),
        (3, 5),
        (5, 2),
        (degree - 3, 1),
        (degree - 2, 2),
        (degree, 1),
    ] {
        left[index] = Integer::from(value);
    }
    let mut right = vec![Integer::zero(); degree];
    for (index, value) in [
        (0, 1),
        (1, 5),
        (2, 6),
        (3, 1),
        (4, 3),
        (degree - 4, 1),
        (degree - 3, 2),
        (degree - 1, 1),
    ] {
        right[index] = Integer::from(value);
    }
    (Z::new(left), Z::new(right))
}

#[test]
fn independent_degree64_and_65_nonidentical_integer_gcd_and_content() {
    use perfect_polynomial::IntegerPolynomial as Z;
    let common = Z::new(vec![
        Integer::one(),
        Integer::from(2),
        Integer::zero(),
        Integer::one(),
    ]);
    let content = Z::new(vec![
        Integer::from(6),
        Integer::from(12),
        Integer::zero(),
        Integer::from(6),
    ]);
    for degree in [49_usize, 56, 64, 65] {
        let (a, b) = sympy_certified_degree64_products(degree);
        assert_ne!(a, b);
        assert_eq!(a.gcd(&b), Ok(common.clone()), "degree {degree}");
        assert_eq!(b.gcd(&a), Ok(common.clone()), "reverse degree {degree}");
        let left = Z::new(
            a.coefficients()
                .iter()
                .map(|c| c.mul(&Integer::from(-12)))
                .collect(),
        );
        let right = Z::new(
            b.coefficients()
                .iter()
                .map(|c| c.mul(&Integer::from(18)))
                .collect(),
        );
        assert_eq!(
            left.gcd(&right),
            Ok(content.clone()),
            "signed degree {degree}"
        );
        assert_eq!(
            right.gcd(&left),
            Ok(content.clone()),
            "reverse signed degree {degree}"
        );
    }
}

#[test]
fn independent_degree64_and_65_nonidentical_rational_monic_gcd() {
    let expected = Q::new(vec![
        Rational::one(),
        r(2, 1),
        Rational::zero(),
        Rational::one(),
    ]);
    for degree in [49_usize, 56, 64, 65] {
        let (a, b) = sympy_certified_degree64_products(degree);
        let left = Q::new(
            a.coefficients()
                .iter()
                .cloned()
                .map(Rational::from)
                .map(|v| v.mul(&r(-7, 11)))
                .collect(),
        );
        let right = Q::new(
            b.coefficients()
                .iter()
                .cloned()
                .map(Rational::from)
                .map(|v| v.mul(&r(13, 17)))
                .collect(),
        );
        assert_eq!(
            left.gcd(&right),
            Ok(expected.clone()),
            "rational degree {degree}"
        );
        assert_eq!(
            right.gcd(&left),
            Ok(expected.clone()),
            "reverse rational degree {degree}"
        );
    }
}

#[test]
fn degree96_independent_expansion_integer_gcd_and_content() {
    use perfect_polynomial::IntegerPolynomial as Z;
    // Independently confirmed with SymPy for n=72,80,96,97:
    // gcd(A_n,B_n)=x^3+2*x+1, for the distinct expanded A/B vectors.
    let common = Z::new(vec![
        Integer::one(),
        Integer::from(2),
        Integer::zero(),
        Integer::one(),
    ]);
    let scaled = Z::new(vec![
        Integer::from(6),
        Integer::from(12),
        Integer::zero(),
        Integer::from(6),
    ]);
    for degree in [72_usize, 80, 96, 97] {
        let (a, b) = sympy_certified_degree64_products(degree);
        assert_ne!(a, b);
        assert_eq!(a.gcd(&b), Ok(common.clone()), "Z degree {degree}");
        assert_eq!(b.gcd(&a), Ok(common.clone()), "reverse Z degree {degree}");
        let signed = |p: &Z, n: i64| {
            Z::new(
                p.coefficients()
                    .iter()
                    .map(|v| v.mul(&Integer::from(n)))
                    .collect(),
            )
        };
        assert_eq!(signed(&a, -12).gcd(&signed(&b, 18)), Ok(scaled.clone()));
    }
}
#[test]
fn degree96_independent_expansion_rational_monic_gcd() {
    // Same independently expanded integer coefficient vectors as above,
    // lifted using signed exact fractions without production multiplication.
    let common = Q::new(vec![r(1, 1), r(2, 1), r(0, 1), r(1, 1)]);
    for degree in [72_usize, 80, 96, 97] {
        let (a, b) = sympy_certified_degree64_products(degree);
        let aq = Q::new(
            a.coefficients()
                .iter()
                .cloned()
                .map(Rational::from)
                .map(|v| v.mul(&r(-7, 11)))
                .collect(),
        );
        let bq = Q::new(
            b.coefficients()
                .iter()
                .cloned()
                .map(Rational::from)
                .map(|v| v.mul(&r(13, 17)))
                .collect(),
        );
        assert_eq!(aq.gcd(&bq), Ok(common.clone()), "Q degree {degree}");
        assert_eq!(bq.gcd(&aq), Ok(common.clone()), "reverse Q degree {degree}");
    }
}

#[test]
fn oversized_rational_lift_inputs_keep_public_exact_gcd() {
    // Independent algebraic oracle: gcd_Q(x^3+c, x^3+x)=1 for every
    // nonzero real rational c. The second polynomial is x*(x^2+1),
    // whose roots 0, +/-i are never roots of x^3+c for real c != 0.
    let b = Q::new(vec![
        Rational::zero(),
        Rational::one(),
        Rational::zero(),
        Rational::one(),
    ]);
    let denominator_over_512 =
        Rational::new(Integer::one(), Integer::from(Natural::one().shl_bits(512))).unwrap();
    let numerator_over_4096 = Rational::new(
        Integer::from(Natural::one().shl_bits(4096)).add(&Integer::one()),
        Integer::from(3_i64),
    )
    .unwrap();
    let one = Q::constant(Rational::one());
    for constant in [denominator_over_512, numerator_over_4096] {
        let a = Q::new(vec![
            constant,
            Rational::zero(),
            Rational::zero(),
            Rational::one(),
        ]);
        assert_eq!(a.gcd(&b), Ok(one.clone()));
        assert_eq!(b.gcd(&a), Ok(one.clone()));
    }
}
