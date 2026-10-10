#![no_main]
use libfuzzer_sys::fuzz_target;
use perfect_arithmetic::Integer;
use perfect_polynomial::{IntegerPolynomial as Z, RationalPolynomial as Q};
use perfect_rational::Rational;

fn build_poly(data: &[u8], offset: usize, len: usize) -> Z {
    let mut coeffs = Vec::with_capacity(len);
    for i in 0..len {
        let v = *data.get(offset + i).unwrap_or(&0);
        coeffs.push(Integer::from(i64::from(v % 23) - 11));
    }
    Z::new(coeffs)
}
fn rational(poly: &Z, den: i64) -> Q {
    Q::new(
        poly.coefficients()
            .iter()
            .cloned()
            .map(|c| Rational::new(c, Integer::from(den)).unwrap())
            .collect(),
    )
}
fuzz_target!(|data: &[u8]| {
    if data.len() < 2 {
        return;
    }
    let alen = usize::from(data[0] % 9);
    let blen = usize::from(data[1] % 9);
    let a = build_poly(data, 2, alen);
    let b = build_poly(data, 2 + alen, blen);
    let sum = a.add(&b);
    let product = a.mul(&b);
    assert_eq!(sum.sub(&b), a);
    assert_eq!(a.mul(&b), b.mul(&a));
    assert_eq!(
        a.mul(&b.add(&Z::new(vec![Integer::one()]))),
        product.add(&a)
    );
    assert_eq!(
        product.derivative(),
        a.derivative().mul(&b).add(&a.mul(&b.derivative()))
    );
    let gcd = a.gcd(&b).unwrap();
    assert_eq!(gcd, b.gcd(&a).unwrap());
    if !gcd.is_zero() {
        assert_eq!(a.div_exact(&gcd).unwrap().mul(&gcd), a);
        assert_eq!(b.div_exact(&gcd).unwrap().mul(&gcd), b);
    }
    let den1 = 2 + i64::from(data.get(30).copied().unwrap_or(0) % 7);
    let den2 = 2 + i64::from(data.get(31).copied().unwrap_or(0) % 7);
    let qa = rational(&a, den1);
    let qb = rational(&b, den2);
    let gq = qa.gcd(&qb).unwrap();
    assert_eq!(gq, qb.gcd(&qa).unwrap());
    if !gq.is_zero() {
        assert_eq!(gq, gq.monic().unwrap());
    }
});
