#![no_main]

use core::hash::{Hash, Hasher};

use libfuzzer_sys::fuzz_target;
use perfect_arithmetic::Natural;

struct StableHasher(u64);

impl StableHasher {
    fn new() -> Self {
        Self(0xcbf2_9ce4_8422_2325)
    }
}

impl Hasher for StableHasher {
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.0 ^= u64::from(byte);
            self.0 = self.0.wrapping_mul(0x0000_0100_0000_01b3);
        }
    }
}

fn natural_from_bytes(bytes: &[u8]) -> Natural {
    let mut value = Natural::zero();
    for &byte in bytes.iter().take(64) {
        value = value.shl_bits(8).add(&Natural::from(byte));
    }
    value
}

fn stable_hash<T: Hash>(value: &T) -> u64 {
    let mut hasher = StableHasher::new();
    value.hash(&mut hasher);
    hasher.finish()
}

fuzz_target!(|data: &[u8]| {
    let split = data.len() / 2;
    let left = natural_from_bytes(&data[..split]);
    let right = natural_from_bytes(&data[split..]);

    assert_eq!(stable_hash(&left), stable_hash(&left.clone()));
    assert_eq!(stable_hash(&right), stable_hash(&right.clone()));

    let sum = left.add(&right);
    assert_eq!(sum.checked_sub(&left).unwrap(), right);
    assert_eq!(sum.checked_sub(&right).unwrap(), left);

    let product = left.mul(&right);
    if !left.is_zero() {
        let (q, r) = product.div_rem(&left).unwrap();
        assert_eq!(q, right);
        assert!(r.is_zero());
    }
    if !right.is_zero() {
        let (q, r) = product.div_rem(&right).unwrap();
        assert_eq!(q, left);
        assert!(r.is_zero());

        let (q, r) = left.div_rem(&right).unwrap();
        assert_eq!(q.mul(&right).add(&r), left);
        assert!(r < right);
    }

    let gcd = left.gcd(&right);
    if !gcd.is_zero() {
        assert!(left.div_rem(&gcd).unwrap().1.is_zero());
        assert!(right.div_rem(&gcd).unwrap().1.is_zero());
    }

    let shift = u64::from(data.first().copied().unwrap_or(0) % 64);
    let shifted = left.shl_bits(shift);
    assert_eq!(shifted.shr_bits(shift), left);
    if !left.is_zero() {
        assert_eq!(shifted.bit_length(), left.bit_length() + shift);
    }
});
