//! Independent large integer division oracle for current Arithmetic main.
//! Reference uses num-bigint 0.4.8; Arithmetic uses its own pinned backend.
use num_bigint::BigUint;
use perfect_arithmetic::Natural;

fn compare_bits(value: &Natural, expected: &BigUint) {
    assert_eq!(value.bit_length(), expected.bits());
    let bytes = expected.to_bytes_le();
    for bit in 0..=expected.bits() + 2 {
        let expected_set = bytes
            .get((bit / 8) as usize)
            .is_some_and(|byte| byte & (1_u8 << (bit % 8)) != 0);
        assert_eq!(
            value.bit(bit),
            expected_set,
            "bit disagreement at {bit}; expected length={}",
            expected.bits()
        );
    }
}

#[test]
fn large_division_and_remainder_match_independent_bigint() {
    let mut cases = 0;
    for bits in [64_u64, 256, 1024, 4096, 8192, 16384] {
        for shift in [0_u64, 1, 7, bits / 2, bits - 1] {
            let divisor_bits = bits - shift;
            for salt in 0_u64..8 {
                let a = Natural::one()
                    .shl_bits(bits)
                    .add(&Natural::one().shl_bits(bits / 2))
                    .add(&Natural::from(salt + 31));
                let b = Natural::one()
                    .shl_bits(divisor_bits)
                    .add(&Natural::one().shl_bits(divisor_bits / 2))
                    .add(&Natural::from(salt * 7 + 3));
                let ra = (BigUint::from(1_u8) << bits)
                    + (BigUint::from(1_u8) << (bits / 2))
                    + BigUint::from(salt + 31);
                let rb = (BigUint::from(1_u8) << divisor_bits)
                    + (BigUint::from(1_u8) << (divisor_bits / 2))
                    + BigUint::from(salt * 7 + 3);
                let (q, r) = a.div_rem(&b).unwrap();
                compare_bits(&q, &(&ra / &rb));
                compare_bits(&r, &(&ra % &rb));
                assert_eq!(
                    q.mul(&b).add(&r),
                    a,
                    "reconstruction: bits={bits}, shift={shift}, salt={salt}"
                );
                assert!(r < b);
                cases += 1;
            }
        }
    }
    assert_eq!(cases, 240);
}

#[test]
fn division_boundary_contracts_are_exact() {
    let a = Natural::one().shl_bits(4096).add(&Natural::from(7_u8));
    let bigger = Natural::one().shl_bits(4097);
    let (q, r) = a.div_rem(&bigger).unwrap();
    assert!(q.is_zero());
    assert_eq!(r, a);
    let (q, r) = a.div_rem(&Natural::one()).unwrap();
    assert_eq!(q, a);
    assert!(r.is_zero());
    assert!(a.div_rem(&Natural::zero()).is_err());
}
