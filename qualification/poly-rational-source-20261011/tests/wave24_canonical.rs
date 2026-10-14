//! Canonical exact numerical transcript; do not hash Rust Debug or std::hash.
//! External runner hashes only W24| lines in original case order.
use perfect_arithmetic::Integer;
use perfect_polynomial::{IntegerPolynomial as Z, RationalPolynomial as Q};
use perfect_rational::Rational;
fn serial_z(p:&Z)->String {
    p.coefficients().iter()
        .map(|v|v.try_to_i128().unwrap().into_value().to_string())
        .collect::<Vec<_>>().join(",")
}
fn serial_r(v:&Rational)->String {
    let m=v.numerator_magnitude().try_to_u128().unwrap().into_value();
    let d=v.denominator().try_to_u128().unwrap().into_value();
    format!("{}{m}/{d}",if v.is_negative(){"-"}else{""})
}
fn serial_q(p:&Q)->String {p.coefficients().iter().map(serial_r).collect::<Vec<_>>().join(",")}
fn number(n:i64,d:i64)->Rational {
    Rational::new(Integer::from(n),Integer::from(d)).unwrap()
}
#[test]
fn canonical_wave24_poly_highdegree_and_rational_division(){
    let mut seed=0x24e1_3917_aa7c_b3e1_u64;
    const DEGREES:[usize;8]=[63,64,95,96,97,127,128,256];
    for case in 0..128 {
        let degree=DEGREES[case%DEGREES.len()];
        let sparse=case%3==0;
        let mut coefficients=Vec::with_capacity(degree+1);
        for i in 0..=degree {
            seed^=seed>>12;seed^=seed<<25;seed^=seed>>27;
            let v=(seed.wrapping_mul(0x2545f4914f6cdd1d)%17) as i64-8;
            coefficients.push(if i==degree {Integer::from(1)}
                else if sparse && i%29!=0 {Integer::zero()}
                else if i==degree/2 {Integer::from(1_i64<<53)}
                else {Integer::from(v)});
        }
        let a=Z::new(coefficients);
        let b=Z::new(vec![Integer::from((case%7) as i64-3),
            Integer::from(1),Integer::from(-1)]);
        let product=a.mul(&b);
        assert_eq!(product.div_exact(&b).unwrap(),a);
        let derivative=product.derivative();
        let evaluation=derivative.evaluate(&Integer::one())
            .try_to_i128().unwrap().into_value();
        let q=Q::new(vec![
            number(case as i64%11-5,3),number(2-(case%5) as i64,7),
            number((case%13) as i64+1,11),
        ]);
        let divisor=Q::new(vec![number(1,2),number(-3,5)]);
        let (quotient,remainder)=q.div_rem(&divisor).unwrap();
        assert_eq!(divisor.mul(&quotient).add(&remainder),q);
        assert!(remainder.is_zero() || remainder.degree()<divisor.degree());
        println!("W24|POLY|{case:03}|{degree}|{}|{evaluation}|{}|{}",
            serial_z(&product),serial_q(&quotient),serial_q(&remainder));
    }
}
