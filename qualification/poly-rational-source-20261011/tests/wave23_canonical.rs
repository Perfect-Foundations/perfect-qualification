use perfect_arithmetic::Integer;
use perfect_polynomial::IntegerPolynomial;
#[test]
fn canonical_polynomial_values() {
    let mut s = 0x2300a177bc011_u64;
    for case in 0..48 {
        let mut rng = || {
            s ^= s >> 12;
            s ^= s << 25;
            s ^= s >> 27;
            (s.wrapping_mul(0x2545f4914f6cdd1d) % 17) as i64 - 8
        };
        let p = IntegerPolynomial::new((0..(1 + case % 9)).map(|_| Integer::from(rng())).collect());
        let q = IntegerPolynomial::new((0..(1 + case % 7)).map(|_| Integer::from(rng())).collect());
        let product = p.mul(&q);
        let derivative = product.derivative();
        let coeffs = |p: &IntegerPolynomial| {
            p.coefficients()
                .iter()
                .map(|v| v.try_to_i64().unwrap().into_value().to_string())
                .collect::<Vec<_>>()
                .join(",")
        };
        let value = product
            .evaluate(&Integer::from(2))
            .try_to_i64()
            .unwrap()
            .into_value();
        println!(
            "CANONICAL|POLY|{case:02}|{}|{}|{value}",
            coeffs(&product),
            coeffs(&derivative)
        );
    }
}
