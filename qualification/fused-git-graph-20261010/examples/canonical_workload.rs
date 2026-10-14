//! Canonical exact public-operation byte stream for cross-host qualification.
//! No std Hash, Debug formatting, process metadata, platform paths or time.
use perfect_arithmetic::{Integer, Natural};
use perfect_polynomial::{IntegerPolynomial as Z, PolynomialError, RationalPolynomial as Q};
use perfect_rational::Rational;

fn dec(v: &Integer) -> String {
    if v.is_zero() {
        return "0".into();
    }
    let negative = v.is_negative();
    let mut n = Integer::from(v.unsigned_abs());
    let ten = Integer::from(10_u8);
    let mut digits = Vec::new();
    while !n.is_zero() {
        let (q, r) = n.div_rem_trunc(&ten).expect("ten is nonzero");
        let digit = r.try_to_u8().expect("remainder in 0..9").into_value();
        digits.push(char::from(b'0' + digit));
        n = q;
    }
    if negative {
        digits.push('-');
    }
    digits.iter().rev().collect()
}
fn frac(v: &Rational) -> String {
    let sign = if v.is_negative() { "-" } else { "" };
    format!(
        "{}{}/{}",
        sign,
        dec(&Integer::from(v.numerator_magnitude().clone())),
        dec(&Integer::from(v.denominator().clone()))
    )
}
fn error_code(e: PolynomialError) -> &'static str {
    match e {
        PolynomialError::DivisionByZero => "DivisionByZero",
        PolynomialError::NonExactDivision => "NonExactDivision",
        PolynomialError::NonIntegralQuotient => "NonIntegralQuotient",
        PolynomialError::ArithmeticFailure => "ArithmeticFailure",
    }
}
fn emit_z(out: &mut String, name: &str, p: &Z) {
    let vals: Vec<String> = p.coefficients().iter().map(dec).collect();
    out.push_str(&format!("{name}|Z|{}|{}\n", vals.len(), vals.join(",")));
}
fn emit_q(out: &mut String, name: &str, p: &Q) {
    let vals: Vec<String> = p.coefficients().iter().map(frac).collect();
    out.push_str(&format!("{name}|Q|{}|{}\n", vals.len(), vals.join(",")));
}
fn z(values: &[i64]) -> Z {
    Z::new(values.iter().copied().map(Integer::from).collect())
}
fn q(values: &[(i64, i64)]) -> Q {
    Q::new(
        values
            .iter()
            .map(|&(n, d)| Rational::new(Integer::from(n), Integer::from(d)).unwrap())
            .collect(),
    )
}
fn to_q(p: &Z) -> Q {
    Q::new(
        p.coefficients()
            .iter()
            .cloned()
            .map(Rational::from)
            .collect(),
    )
}
fn stream() -> Vec<u8> {
    let mut out = String::from("PF-CANONICAL-POLYNOMIAL-V1\n");
    let zero = Z::new(vec![Integer::zero(), Integer::zero()]);
    let a = z(&[-7, 0, 5, -9, 1]);
    let b = z(&[3, -2, 0, 1]);
    emit_z(&mut out, "z.zero", &zero);
    emit_z(&mut out, "z.add", &a.add(&b));
    emit_z(&mut out, "z.sub", &a.sub(&b));
    emit_z(&mut out, "z.mul", &a.mul(&b));
    emit_z(&mut out, "z.derivative", &a.derivative());
    emit_z(
        &mut out,
        "z.eval_negative",
        &Z::constant(a.evaluate(&Integer::from(-11))),
    );
    let product = a.mul(&b);
    emit_z(
        &mut out,
        "z.exact_quotient",
        &product.div_exact(&b).unwrap(),
    );
    let mut bad = product.coefficients().to_vec();
    bad[0] = bad[0].add(&Integer::one());
    out.push_str(&format!(
        "z.nonexact_error|ERR|{}\n",
        error_code(Z::new(bad).div_exact(&b).unwrap_err())
    ));
    out.push_str(&format!(
        "z.zero_divisor|ERR|{}\n",
        error_code(a.div_exact(&zero).unwrap_err())
    ));
    emit_z(
        &mut out,
        "z.content",
        &Z::constant(Integer::from(a.mul(&z(&[42])).content())),
    );
    emit_z(
        &mut out,
        "z.primitive",
        &a.mul(&z(&[-42])).primitive_part().unwrap(),
    );
    let h = z(&[5, -2, 1]);
    let u = z(&[2, 3, 0, 0, 0, 0, 1]);
    let v = z(&[1, -4, 0, 0, 0, 1]);
    let az = h.mul(&u).mul(&z(&[12]));
    let bz = h.mul(&v).mul(&z(&[-18]));
    emit_z(&mut out, "z.multistage_gcd", &az.gcd(&bz).unwrap());
    emit_q(
        &mut out,
        "q.multistage_gcd",
        &to_q(&az).gcd(&to_q(&bz)).unwrap(),
    );
    let qa = q(&[(3, 7), (-2, 5), (0, 1), (11, 13), (-7, 2)]);
    let qb = q(&[(5, 11), (1, 3), (-9, 7)]);
    emit_q(&mut out, "q.add", &qa.add(&qb));
    emit_q(&mut out, "q.sub", &qa.sub(&qb));
    emit_q(&mut out, "q.mul", &qa.mul(&qb));
    emit_q(&mut out, "q.derivative", &qa.derivative());
    emit_q(
        &mut out,
        "q.eval_negative",
        &Q::constant(qa.evaluate(&Rational::from(-3))),
    );
    let (quo, rem) = qa.div_rem(&qb).unwrap();
    emit_q(&mut out, "q.quotient", &quo);
    emit_q(&mut out, "q.remainder", &rem);
    emit_q(&mut out, "q.monic", &qa.monic().unwrap());
    emit_q(
        &mut out,
        "q.zero",
        &Q::new(vec![Rational::zero(), Rational::zero()]),
    );
    let wide = Integer::from(Natural::one().shl_bits(521))
        .neg()
        .add(&Integer::from(17));
    let w = Z::new(vec![
        wide.clone(),
        Integer::zero(),
        wide.neg(),
        Integer::one(),
    ]);
    emit_z(&mut out, "z.wide_input", &w);
    emit_z(&mut out, "z.wide_mul", &w.mul(&z(&[3, -5, 7])));
    emit_z(&mut out, "z.wide_derivative", &w.derivative());
    let denominator = Integer::from(Natural::one().shl_bits(513));
    let rwide = Rational::new(Integer::one(), denominator).unwrap();
    let rq = Q::new(vec![rwide, Rational::from(-4), Rational::from(1)]);
    emit_q(&mut out, "q.large_denominator_mul", &rq.mul(&qa));
    let (quotient, remainder) = rq.mul(&qb).add(&q(&[(1, 17)])).div_rem(&qb).unwrap();
    emit_q(&mut out, "q.large_denominator_quotient", &quotient);
    emit_q(&mut out, "q.large_denominator_remainder", &remainder);
    let dense = |length: usize, salt: u64| {
        let mut seed = 20261010_u64 ^ salt;
        let mut coeffs = Vec::new();
        for _ in 0..length {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            coeffs.push(Integer::from((seed >> 33) as i64 % 41 - 20));
        }
        if let Some(last) = coeffs.last_mut() {
            *last = Integer::one();
        }
        Z::new(coeffs)
    };
    let d1 = dense(97, 1);
    let d2 = dense(95, 2);
    emit_z(&mut out, "z.dense_kara_97x95", &d1.mul(&d2));
    let mut sparse = vec![Integer::zero(); 129];
    sparse[0] = Integer::from(-31);
    sparse[67] = Integer::from(11);
    sparse[128] = Integer::one();
    emit_z(&mut out, "z.sparse_129x97", &Z::new(sparse).mul(&d1));
    emit_z(&mut out, "z.dense_derivative", &d1.derivative());
    emit_z(
        &mut out,
        "z.dense_eval",
        &Z::constant(d1.evaluate(&Integer::from(-2))),
    );
    let late_a: [i128; 10] = [
        29878539771007619802695925865,
        38337129496435065462512069061,
        26176432220010364463952772943,
        36455492137305517204742188554,
        -31583172706667982753055670592,
        24146646319036245156938719383,
        -33042054106061461237049169535,
        27587738088759190405566614755,
        39124099768788597505792466446,
        37241909747421405995232538844,
    ];
    let late_b: [i128; 9] = [
        -32510778393112134553770803457,
        22708147360007579159236093779,
        20811411071155743672883378584,
        29928754231223780915951098300,
        -34504213327522140517185037813,
        -36255739560194229515569692825,
        -32845031363358562485482133337,
        31138875340773139053657760992,
        -29878475348932783589686397830,
    ];
    let la = Z::new(late_a.into_iter().map(Integer::from).collect());
    let lb = Z::new(late_b.into_iter().map(Integer::from).collect());
    emit_z(&mut out, "z.late_prs_gcd", &la.gcd(&lb).unwrap());
    emit_q(
        &mut out,
        "q.late_prs_gcd",
        &to_q(&la).gcd(&to_q(&lb)).unwrap(),
    );
    let guard = Z::new(vec![
        Integer::from(Natural::one().shl_bits(4097)),
        Integer::one(),
    ]);
    emit_z(
        &mut out,
        "z.width_guard_fallback",
        &guard.gcd(&guard).unwrap(),
    );
    let mut degree = vec![Integer::zero(); 34];
    degree[0] = Integer::from(2);
    degree[33] = Integer::from(-6);
    let degree = Z::new(degree);
    emit_z(
        &mut out,
        "z.degree_guard_fallback",
        &degree.gcd(&degree).unwrap(),
    );
    out.push_str(&format!(
        "z.nonintegral_error|ERR|{}\n",
        error_code(z(&[1, 1]).div_exact(&z(&[2, 2])).unwrap_err())
    ));
    out.into_bytes()
}
fn main() {
    let bytes = stream();
    let path = std::env::args()
        .nth(1)
        .expect("pass exact output file path");
    std::fs::write(path, bytes).expect("write canonical byte stream");
}
