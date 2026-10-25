//! Distinct V2 exact-resource and dispatch-boundary canonical byte corpus.
//! V1 stream and fingerprint remain unchanged and retained separately.
use perfect_arithmetic::{Integer, Natural};
use perfect_polynomial::{IntegerPolynomial as Z, PolynomialError, RationalPolynomial as Q};
use perfect_rational::Rational;

fn decimal(v: &Integer) -> String {
    if v.is_zero() {
        return String::from("0");
    }
    let mut n = Integer::from(v.unsigned_abs());
    let mut digits = Vec::new();
    while !n.is_zero() {
        let (q, r) = n.div_rem_trunc(&Integer::from(10)).unwrap();
        digits.push(char::from(b'0' + r.try_to_u8().unwrap().into_value()));
        n = q;
    }
    if v.is_negative() {
        digits.push('-');
    }
    digits.iter().rev().collect()
}
fn rational_decimal(r: &Rational) -> String {
    let neg = if r.is_negative() { "-" } else { "" };
    format!(
        "{neg}{}/{}",
        decimal(&Integer::from(r.numerator_magnitude().clone())),
        decimal(&Integer::from(r.denominator().clone()))
    )
}
fn emit_z(out: &mut String, id: &str, p: &Z) {
    let cs: Vec<String> = p.coefficients().iter().map(decimal).collect();
    out.push_str(&format!("{id}|Z|{}|{}\n", cs.len(), cs.join(",")));
}
fn emit_q(out: &mut String, id: &str, p: &Q) {
    let cs: Vec<String> = p.coefficients().iter().map(rational_decimal).collect();
    out.push_str(&format!("{id}|Q|{}|{}\n", cs.len(), cs.join(",")));
}
fn error_code(e: PolynomialError) -> &'static str {
    match e {
        PolynomialError::DivisionByZero => "DivisionByZero",
        PolynomialError::NonExactDivision => "NonExactDivision",
        PolynomialError::NonIntegralQuotient => "NonIntegralQuotient",
        PolynomialError::ArithmeticFailure => "ArithmeticFailure",
    }
}
fn from_z(p: &Z) -> Q {
    Q::new(
        p.coefficients()
            .iter()
            .cloned()
            .map(Rational::from)
            .collect(),
    )
}
fn z(values: &[i64]) -> Z {
    Z::new(values.iter().copied().map(Integer::from).collect())
}
fn q(values: &[(i64, i64)]) -> Q {
    Q::new(
        values
            .iter()
            .map(|&(n, d)| Rational::new(n.into(), d.into()).unwrap())
            .collect(),
    )
}
fn monomial(degree: usize) -> Q {
    let mut c = vec![Rational::zero(); degree + 1];
    c[degree] = Rational::one();
    Q::new(c)
}
fn corpus() -> Vec<u8> {
    let mut out = String::from("PF-CANONICAL-POLYNOMIAL-V2\n");
    let p = Integer::from(Natural::one().shl_bits(2048));
    let top = Integer::from(Natural::one().shl_bits(2049));
    let a0 = p.sub(&Integer::one());
    let b3 = top.sub(&Integer::one());
    let a_over = Z::new(vec![
        a0,
        Integer::zero(),
        Integer::zero(),
        Integer::zero(),
        Integer::one(),
    ]);
    let b_over = Z::new(vec![Integer::zero(), Integer::zero(), Integer::one(), b3]);
    emit_z(
        &mut out,
        "z.prs_uncancellable_4097_gcd",
        &a_over.gcd(&b_over).unwrap(),
    );
    emit_q(
        &mut out,
        "q.prs_uncancellable_4097_gcd",
        &from_z(&a_over).gcd(&from_z(&b_over)).unwrap(),
    );
    let a_fit = Z::new(vec![
        p,
        Integer::zero(),
        Integer::zero(),
        Integer::zero(),
        Integer::one(),
    ]);
    let b_fit = Z::new(vec![
        Integer::zero(),
        Integer::zero(),
        Integer::one(),
        Integer::from(Natural::one().shl_bits(2047)),
    ]);
    emit_z(&mut out, "z.prs_4096_gcd", &a_fit.gcd(&b_fit).unwrap());
    let w = Integer::from(Natural::one().shl_bits(257)).add(&Integer::from(3));
    let b = z(&[-3, 2]);
    let quotient = Z::new(vec![Integer::from(5), w.neg(), w.clone()]);
    let a = b.mul(&quotient);
    emit_z(&mut out, "z.wide_signed_exact_q", &a.div_exact(&b).unwrap());
    emit_z(
        &mut out,
        "z.wide_signed_exact_negative_divisor",
        &a.div_exact(&b.neg()).unwrap(),
    );
    out.push_str(&format!(
        "z.wide_nonexact|ERR|{}\n",
        error_code(a.add(&z(&[1])).div_exact(&b).unwrap_err())
    ));
    out.push_str(&format!(
        "z.nonintegral|ERR|{}\n",
        error_code(z(&[1, 1]).div_exact(&z(&[2, 2])).unwrap_err())
    ));
    out.push_str(&format!(
        "z.zero_divisor|ERR|{}\n",
        error_code(a.div_exact(&Z::zero()).unwrap_err())
    ));
    for bits in [512_u64, 513] {
        let denominator = Integer::from(Natural::one().shl_bits(bits - 1));
        let divisor = Q::new(vec![
            Rational::one(),
            Rational::new(Integer::from(-1), denominator).unwrap(),
        ]);
        let quo = monomial(11).mul(&q(&[(2, 1)])).add(&q(&[(-3, 1)]));
        let remainder = q(&[(1, 17)]);
        let a = divisor.mul(&quo).add(&remainder);
        let (actual_q, actual_r) = a.div_rem(&divisor).unwrap();
        emit_q(&mut out, &format!("q.lcm{bits}.quotient"), &actual_q);
        emit_q(&mut out, &format!("q.lcm{bits}.remainder"), &actual_r);
    }
    let unit_divisor = q(&[(1, 1), (1, 1)]);
    for steps in [11_usize, 12, 160, 161] {
        let quo = monomial(steps - 1);
        let a = unit_divisor.mul(&quo);
        let (result, remainder) = a.div_rem(&unit_divisor).unwrap();
        emit_q(&mut out, &format!("q.steps{steps}.quotient"), &result);
        emit_q(&mut out, &format!("q.steps{steps}.remainder"), &remainder);
    }
    let h = z(&[1, 2, 1]);
    let h_squared = h.mul(&h);
    let left = h_squared.mul(&z(&[1, 0, 1])).mul(&z(&[-42]));
    let right = h_squared.mul(&z(&[2, 1])).mul(&z(&[30]));
    emit_z(
        &mut out,
        "z.repeated_factor_gcd",
        &left.gcd(&right).unwrap(),
    );
    emit_q(
        &mut out,
        "q.repeated_factor_gcd",
        &from_z(&left).gcd(&from_z(&right)).unwrap(),
    );
    emit_z(&mut out, "z.zero_after_cancellation", &left.sub(&left));
    emit_q(
        &mut out,
        "q.zero_after_cancellation",
        &from_z(&right).sub(&from_z(&right)),
    );
    for n in [63_usize, 64, 65] {
        let mut a: Vec<Integer> = (0..n)
            .map(|i| Integer::from(((i * 17) % 23) as i64 - 11))
            .collect();
        let mut b: Vec<Integer> = (0..n)
            .map(|i| Integer::from(((i * 19) % 29) as i64 - 14))
            .collect();
        a[n - 1] = Integer::one();
        b[n - 1] = Integer::one();
        emit_z(
            &mut out,
            &format!("z.mul_dense_{n}x{n}"),
            &Z::new(a).mul(&Z::new(b)),
        );
    }
    out.into_bytes()
}
fn main() {
    let dest = std::env::args().nth(1).expect("exact byte output path");
    std::fs::write(dest, corpus()).expect("write canonical V2 bytes");
}
