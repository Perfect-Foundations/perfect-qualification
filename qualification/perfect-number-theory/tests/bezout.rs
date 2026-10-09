//! Independent primitive Euclid and exact Bézout verification of public API.
use perfect_arithmetic::{Integer, Natural};
use perfect_number_theory::extended_gcd;

fn primitive_gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

#[test]
fn bounded_and_high_bit_bezout_identities_are_exact() {
    for a in 0_u64..=64 {
        for b in 0_u64..=64 {
            let value = extended_gcd(&Natural::from(a), &Natural::from(b)).unwrap();
            assert_eq!(value.gcd, Natural::from(primitive_gcd(a, b)));
            assert_eq!(
                value
                    .x
                    .mul(&Integer::from(a))
                    .add(&value.y.mul(&Integer::from(b))),
                Integer::from(value.gcd)
            );
        }
    }
    let zero = Natural::zero();
    let wide = Natural::one().shl_bits(512);
    let narrow = Natural::one().shl_bits(256);
    for (a, b, expected) in [
        (zero.clone(), zero.clone(), zero.clone()),
        (wide.clone(), zero.clone(), wide.clone()),
        (zero.clone(), wide.clone(), wide.clone()),
        (wide.clone(), narrow.clone(), narrow.clone()),
        (narrow.clone(), wide, narrow),
    ] {
        let value = extended_gcd(&a, &b).unwrap();
        assert_eq!(value.gcd, expected);
        assert_eq!(
            value
                .x
                .mul(&Integer::from(a))
                .add(&value.y.mul(&Integer::from(b))),
            Integer::from(expected)
        );
    }
}
