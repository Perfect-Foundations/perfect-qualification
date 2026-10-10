use perfect_arithmetic::Integer;
use perfect_polynomial::IntegerPolynomial as Z;
use std::hash::{Hash, Hasher};
fn parse(s: &str) -> Z {
    if s == "_" {
        return Z::zero();
    }
    Z::new(
        s.split(',')
            .map(|c| Integer::from(c.parse::<i128>().unwrap()))
            .collect(),
    )
}
struct Fingerprint(u64);
impl Fingerprint {
    fn new() -> Self {
        Self(0xcbf29ce484222325)
    }
    fn byte(&mut self, v: u8) {
        self.0 = (self.0 ^ u64::from(v)).wrapping_mul(0x100000001b3);
    }
    fn text(&mut self, s: &str) {
        let len = s.len() as u64;
        for c in len.to_le_bytes() {
            self.byte(c);
        }
        for c in s.as_bytes() {
            self.byte(*c);
        }
    }
    fn poly(&mut self, p: &Z) {
        self.text(&p.coefficients().len().to_string());
        for c in p.coefficients() {
            c.hash(self);
        }
    }
}
impl Hasher for Fingerprint {
    fn finish(&self) -> u64 {
        self.0
    }
    fn write(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.byte(*byte);
        }
    }
}
fn main() {
    let mut fingerprint = Fingerprint::new();
    let mut count = 0_usize;
    for row in include_str!("../vectors/adversarial-gcd-v1.tsv")
        .lines()
        .skip(1)
    {
        let f: Vec<_> = row.split('\t').collect();
        let a = parse(f[1]);
        let b = parse(f[2]);
        fingerprint.text(f[0]);
        fingerprint.poly(&a.gcd(&b).unwrap());
        fingerprint.poly(&a.add(&b));
        fingerprint.poly(&a.derivative());
        count += 1;
    }
    for row in include_str!("../vectors/robustness-v1.tsv").lines().skip(1) {
        let f: Vec<_> = row.split('\t').collect();
        let a = parse(f[1]);
        let b = parse(f[2]);
        fingerprint.text(f[0]);
        fingerprint.poly(&a.gcd(&b).unwrap());
        fingerprint.poly(&a.mul(&b));
        fingerprint.poly(&a.derivative());
        count += 1;
    }
    assert_eq!(count, 579);
    println!(
        "source=d2365a0fc5af81877e64598301064ea2910e67f9 cases={count} fnv64={:016x}",
        fingerprint.0
    );
}
