//! Qualification-owned independent Python Fraction / SymPy vectors.
//! Not copied from the source crate's test modules.
use perfect_arithmetic::{Integer, Natural};
use perfect_polynomial::{IntegerPolynomial as Z, PolynomialError, RationalPolynomial as Q};
use perfect_rational::Rational;

fn z(cs: &[i64]) -> Z {
    Z::new(cs.iter().copied().map(Integer::from).collect())
}
fn r(n: i64, d: i64) -> Rational {
    Rational::new(Integer::from(n), Integer::from(d)).unwrap()
}

#[test]
fn independently_calculated_integer_exact_quotient_and_refusals() {
    // Python convolution independently computed: (3-5x+2x²)
    // * (7-11x+13x²-17x³+19x⁴).
    let a = z(&[21, -68, 108, -138, 168, -129, 38]);
    let b = z(&[3, -5, 2]);
    let q = z(&[7, -11, 13, -17, 19]);
    assert_eq!(a.div_exact(&b).unwrap(), q);
    assert_eq!(a.div_exact(&b.neg()).unwrap(), q.neg());
    assert_eq!(
        z(&[1]).div_exact(&Z::zero()),
        Err(PolynomialError::DivisionByZero)
    );
    assert_eq!(
        z(&[1, 1]).div_exact(&z(&[2, 2])),
        Err(PolynomialError::NonIntegralQuotient)
    );
    assert_eq!(
        z(&[1, 0, 1]).div_exact(&z(&[1, 2])),
        Err(PolynomialError::NonExactDivision)
    );
}

#[test]
fn independently_calculated_fraction_quotient_and_remainder() {
    // Python fractions.Fraction convolution, ascending x coefficients:
    // B=[2/3,-5/7,11/4], Q=[-3/5,7/11,13/9,-2/3],
    // R=[17/19,-4/23]
    // A=[47/95,3607/5313,-6781/5940,23/84,1121/252,-11/6].
    let a = Q::new(vec![
        r(47, 95),
        r(3607, 5313),
        r(-6781, 5940),
        r(23, 84),
        r(1121, 252),
        r(-11, 6),
    ]);
    let b = Q::new(vec![r(2, 3), r(-5, 7), r(11, 4)]);
    let expected_q = Q::new(vec![r(-3, 5), r(7, 11), r(13, 9), r(-2, 3)]);
    let expected_r = Q::new(vec![r(17, 19), r(-4, 23)]);
    assert_eq!(
        a.div_rem(&b).unwrap(),
        (expected_q.clone(), expected_r.clone())
    );
    assert_eq!(b.mul(&expected_q).add(&expected_r), a);
    assert!(expected_r.degree() < b.degree());
}

#[test]
fn sympy_multistage_gcd_and_content() {
    // Independently verified in SymPy: gcd(U,V)=1 where
    // U=x^39+3x^20+2, V=x^20+4x^10+1, H=x²-2x+5.
    let mut u = vec![Integer::zero(); 40];
    u[0] = 2.into();
    u[20] = 3.into();
    u[39] = 1.into();
    let mut v = vec![Integer::zero(); 21];
    v[0] = 1.into();
    v[10] = 4.into();
    v[20] = 1.into();
    let h = z(&[5, -2, 1]);
    let a = h.mul(&Z::new(u));
    let b = h.mul(&Z::new(v));
    assert_eq!(a.gcd(&b).unwrap(), h);
    let az = a.mul(&z(&[12]));
    let bz = b.mul(&z(&[-18]));
    assert_eq!(az.gcd(&bz).unwrap(), z(&[30, -12, 6]));
    assert_eq!(bz.gcd(&az).unwrap(), z(&[30, -12, 6]));
}

#[test]
fn mutable_arithmetic_large_exact_carry_and_cancellation() {
    for bits in [128_u64, 512, 2048, 8192] {
        let power = Integer::from(Natural::one().shl_bits(bits));
        let mut borrow = power.sub(&Integer::one());
        borrow.sub_mul_assign(&Integer::from(-1), &Integer::one());
        assert_eq!(borrow, power);
        let mut zero = power.clone();
        zero.scale_sub_mul_assign(&Integer::from(-1), &power.neg(), &Integer::one());
        assert!(zero.is_zero());
        let mut scale = Integer::from(-3);
        scale.scale_sub_mul_assign(&Integer::from(-1), &Integer::from(9), &Integer::from(-1));
        assert_eq!(scale, Integer::from(12));
    }
}
#[test]
fn independent_sympy_bounded_primitive_gcd() {
    // SymPy: H=x²-2x+5; gcd(H*(x^6+3*x+2),H*(x^5-4*x+1))=H.
    let h = z(&[5, -2, 1]);
    let a = z(&[10, 11, -4, 3, 0, 0, 5, -2, 1]);
    let b = z(&[5, -22, 9, -4, 0, 5, -2, 1]);
    assert_eq!(a.gcd(&b).unwrap(), h);
    assert_eq!(a.neg().gcd(&b).unwrap(), h);
    assert_eq!(a.div_exact(&h).unwrap(), z(&[2, 3, 0, 0, 0, 0, 1]));
    assert_eq!(b.div_exact(&h).unwrap(), z(&[1, -4, 0, 0, 0, 1]));
    assert_eq!(
        a.mul(&z(&[12])).gcd(&b.mul(&z(&[-18]))).unwrap(),
        z(&[30, -12, 6])
    );
    let qa = Q::new(
        a.coefficients()
            .iter()
            .cloned()
            .map(Rational::from)
            .collect(),
    );
    let qb = Q::new(
        b.coefficients()
            .iter()
            .cloned()
            .map(Rational::from)
            .collect(),
    );
    let qh = Q::new(
        h.coefficients()
            .iter()
            .cloned()
            .map(Rational::from)
            .collect(),
    );
    assert_eq!(qa.gcd(&qb).unwrap(), qh);
}

#[test]
fn direct_unsigned_width_exact_power_boundaries_and_fused_operators() {
    assert_eq!(Integer::zero().unsigned_bit_length(), 0);
    for bits in [
        1_u64, 31, 32, 63, 64, 127, 128, 129, 512, 2048, 4096, 8192, 16384,
    ] {
        let power = Integer::from(Natural::one().shl_bits(bits));
        let below = power.sub(&Integer::one());
        let above = power.add(&Integer::one());
        assert_eq!(power.unsigned_bit_length(), bits + 1);
        assert_eq!(power.neg().unsigned_bit_length(), bits + 1);
        assert_eq!(below.unsigned_bit_length(), bits);
        assert_eq!(below.neg().unsigned_bit_length(), bits);
        assert_eq!(above.unsigned_bit_length(), bits + 1);
        let mut accumulator = power.clone();
        accumulator.scale_sub_mul_assign(&Integer::from(-1), &power.neg(), &Integer::one());
        assert!(accumulator.is_zero());
        assert_eq!(accumulator.unsigned_bit_length(), 0);
    }
}

#[test]
fn sympy_repeated_quadratic_factor_with_signed_integer_content() {
    // SymPy 1.14 QQ/ZZ independent oracle:
    // H=x^2+3x+5, U=x^7-3x^5+2x+4, V=x^6+5x^2-x+1.
    // A=-42*H^2*U, B=30*H^2*V, gcd_ZZ(A,B)=6H^2,
    // gcd_QQ(A,B)=H^2 (monic). No output reconstructed by this crate.
    let a = z(&[
        -4200, -7140, -5712, -2604, -672, 3066, 3780, 1344, -504, -672, -252, -42,
    ]);
    let b = z(&[750, 150, 3420, 4110, 2700, 870, 900, 900, 570, 180, 30]);
    let expected_z = z(&[150, 180, 114, 36, 6]);
    let expected_q = Q::new(
        [25, 30, 19, 6, 1]
            .into_iter()
            .map(|n| Rational::from(Integer::from(n)))
            .collect(),
    );
    assert_eq!(a.gcd(&b).unwrap(), expected_z);
    assert_eq!(b.gcd(&a).unwrap(), expected_z);
    assert_eq!(a.neg().gcd(&b).unwrap(), expected_z);
    let qa = Q::new(
        a.coefficients()
            .iter()
            .cloned()
            .map(Rational::from)
            .collect(),
    );
    let qb = Q::new(
        b.coefficients()
            .iter()
            .cloned()
            .map(Rational::from)
            .collect(),
    );
    assert_eq!(qa.gcd(&qb).unwrap(), expected_q);
    assert_eq!(qb.gcd(&qa).unwrap(), expected_q);
    assert_eq!(
        a.div_exact(&z(&[25, 30, 19, 6, 1])).unwrap(),
        z(&[-168, -84, 0, 0, 0, 126, 0, -42])
    );
}
