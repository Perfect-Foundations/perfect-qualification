#![no_main]

use core::hash::{Hash, Hasher};

use libfuzzer_sys::fuzz_target;
use perfect_arithmetic::{Integer, Natural};

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

fn magnitude_from_bytes(bytes: &[u8]) -> Natural {
    let mut value = Natural::zero();
    for &byte in bytes.iter().take(64) {
        value = value.shl_bits(8).add(&Natural::from(byte));
    }
    value
}

fn integer_from_bytes(bytes: &[u8], negative: bool) -> Integer {
    let value = Integer::from(magnitude_from_bytes(bytes));
    if negative { value.neg() } else { value }
}

fn stable_hash<T: Hash>(value: &T) -> u64 {
    let mut hasher = StableHasher::new();
    value.hash(&mut hasher);
    hasher.finish()
}

fuzz_target!(|data: &[u8]| {
    let control = data.first().copied().unwrap_or(0);
    let payload = data.get(1..).unwrap_or_default();
    let split = payload.len() / 2;

    let left = integer_from_bytes(&payload[..split], control & 1 != 0);
    let right = integer_from_bytes(&payload[split..], control & 2 != 0);

    assert_eq!(stable_hash(&left), stable_hash(&left.clone()));
    assert_eq!(stable_hash(&right), stable_hash(&right.clone()));

    assert_eq!(left.add(&right).sub(&right), left);
    assert_eq!(left.sub(&right).add(&right), left);
    assert_eq!(left.neg().neg(), left);

    let product = left.mul(&right);
    if !left.is_zero() {
        let (q, r) = product.div_rem_trunc(&left).unwrap();
        assert_eq!(q, right);
        assert!(r.is_zero());
    }

    if !right.is_zero() {
        let (q, r) = left.div_rem_trunc(&right).unwrap();
        assert_eq!(q.mul(&right).add(&r), left);
        assert!(r.unsigned_abs() < right.unsigned_abs());
        if !r.is_zero() {
            assert_eq!(r.is_negative(), left.is_negative());
        }
    }

    assert_eq!(left.unsigned_abs().is_zero(), left.is_zero());
});
