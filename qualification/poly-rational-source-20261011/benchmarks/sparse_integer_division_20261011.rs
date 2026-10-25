use perfect_arithmetic::Integer;
use perfect_polynomial::IntegerPolynomial as Z;
use std::{hint::black_box,time::Instant};
fn poly(size:usize, terms:&[(usize,i64)])->Z{
    let mut cs=vec![Integer::zero();size];
    for &(i,n) in terms{cs[i]=Integer::from(n);}
    Z::new(cs)
}
fn main(){
    for &(d, qlen, sparse) in &[
      (64_usize,200_usize,true),(128,200,true),(256,200,true),(512,200,true),
      (128,20,false),(256,20,false),
    ]{
        let divisor=if sparse{
            poly(d+1,&[(0,3),(d,1)])
        }else{
            Z::new((0..=d).map(|i|Integer::from((i%7+1) as i64)).collect())
        };
        let quotient=Z::new((0..=qlen).map(|i|Integer::from((i%9+1) as i64)).collect());
        let dividend=divisor.mul(&quotient);
        let rounds=if !sparse {4_u128} else if d>=256 {24} else {48};
        let now=Instant::now();let mut checksum=0usize;
        for _ in 0..rounds{
          let out=black_box(&dividend).div_exact(black_box(&divisor)).unwrap();
          checksum^=black_box(out.coefficients().len());
        }
        println!("INTDIV d={d} q={qlen} sparse={sparse} ns={} checksum={checksum}",now.elapsed().as_nanos()/rounds);
    }
}
