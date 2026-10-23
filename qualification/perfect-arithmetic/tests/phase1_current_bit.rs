//! Qualification-owned independent bit reference for Arithmetic current main.
//! This covers the additive Natural::bit API without deriving expected bits
//! from Arithmetic bit methods.

use perfect_arithmetic::Natural;

#[test]
fn natural_bit_matches_primitive_oracle_across_boundaries() {
    let mut checked = 0_u64;
    for raw in 0_u16..=4095 {
        let number = Natural::from(raw);
        for index in 0_u64..=128 {
            let expected = if index < u64::from(u16::BITS) {
                (raw & (1_u16 << u32::try_from(index).unwrap())) != 0
            } else {
                false
            };
            assert_eq!(number.bit(index), expected, "raw={raw} bit={index}");
            checked += 1;
        }
    }
    assert_eq!(checked, 4096 * 129);
}

#[test]
fn natural_bit_handles_sparse_multilimb_and_huge_indices() {
    let set_bits = [0_u64, 63, 64, 127, 128, 255, 256, 4095];
    let mut value = Natural::from(0_u8);
    for &index in &set_bits {
        value = value.add(&Natural::one().shl_bits(index));
    }
    for index in [
        0,
        1,
        62,
        63,
        64,
        65,
        126,
        127,
        128,
        129,
        254,
        255,
        256,
        257,
        4094,
        4095,
        4096,
        65535,
        u64::MAX,
    ] {
        assert_eq!(
            value.bit(index),
            set_bits.contains(&index),
            "multilimb bit={index}"
        );
    }
    let zero = Natural::from(0_u8);
    for index in [0, 1, 63, 64, 4095, u64::MAX] {
        assert!(!zero.bit(index));
    }
}
