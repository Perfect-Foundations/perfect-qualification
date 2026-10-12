use perfect_algebra::{ExtensionField, ExtensionLimits, PrimeField};
use perfect_arithmetic::Natural;
use perfect_number_theory::{PrimalityResult, PrimeModulus, classify_u64};

fn binary_field(modulus: &[u64]) -> ExtensionField {
    let PrimalityResult::ProvenPrime(proof) = classify_u64(2) else {
        panic!("proven prime")
    };
    let base = PrimeField::new(PrimeModulus::from_proven(proof).unwrap());
    let f = modulus
        .iter()
        .map(|&v| base.element(Natural::from(v)).unwrap())
        .collect();
    ExtensionField::new(base, f, ExtensionLimits::default()).unwrap()
}
fn symbol(field: &ExtensionField, bits: u64) -> perfect_algebra::ExtensionFieldElement {
    field
        .element(
            (0..field.degree())
                .map(|i| {
                    field
                        .base()
                        .element(Natural::from((bits >> i) & 1))
                        .unwrap()
                })
                .collect(),
        )
        .unwrap()
}
fn independent_bit_poly_mul(mut a: u64, mut b: u64, degree: u32, modulus: u64) -> u64 {
    let mut result = 0u64;
    while b != 0 {
        if b & 1 != 0 {
            result ^= a;
        }
        b >>= 1;
        a <<= 1;
        if a & (1u64 << degree) != 0 {
            a ^= modulus;
        }
    }
    result
}
#[test]
fn canonical_monic_multiplication_matches_independent_bit_polynomials() {
    for (poly, mask, degree, seeds) in [
        (
            &[1u64, 1, 0, 0, 1][..],
            0x13u64,
            4u32,
            &[0u64, 1, 2, 3, 7, 15][..],
        ),
        (
            &[1u64, 1, 0, 1, 1, 0, 0, 0, 1][..],
            0x11bu64,
            8u32,
            &[0u64, 1, 2, 3, 0x53, 0xca, 0xff][..],
        ),
    ] {
        let f = binary_field(poly);
        let identical = binary_field(poly);
        let order = 1u64 << degree;
        for &a in seeds {
            for b in 0..order {
                let lhs = symbol(&f, a);
                let rhs = symbol(&identical, b);
                let result = lhs.mul(&rhs).unwrap();
                let expected = independent_bit_poly_mul(a, b, degree, mask);
                assert_eq!(result, symbol(&f, expected), "degree={degree} a={a} b={b}");
                assert_eq!(result.parent(), &f);
            }
        }
        let high = f
            .element(
                (0..=degree)
                    .map(|i| {
                        f.base()
                            .element(Natural::from(u64::from(i == degree)))
                            .unwrap()
                    })
                    .collect(),
            )
            .unwrap();
        assert_eq!(high, symbol(&f, mask ^ (1u64 << degree)));
    }
}

#[test]
fn extension_inverse_matches_independent_brute_force_binary_field_oracle() {
    for (poly, mask, degree) in [
        (&[1_u64, 1, 0, 0, 1][..], 0x13u64, 4u32),
        (&[1_u64, 1, 0, 1, 1, 0, 0, 0, 1][..], 0x11bu64, 8u32),
    ] {
        let field = binary_field(poly);
        let order = 1u64 << degree;
        for a in 1..order {
            let expected = (1..order)
                .find(|&candidate| independent_bit_poly_mul(a, candidate, degree, mask) == 1)
                .expect("finite-field element has unique inverse");
            let actual = symbol(&field, a).inverse().unwrap();
            assert_eq!(
                actual,
                symbol(&field, expected),
                "GF({order}) inverse input={a}"
            );
            assert_eq!(actual.inverse().unwrap(), symbol(&field, a));
        }
        assert!(field.zero().inverse().is_err());
    }
}

#[test]
fn inplace_subtract_matches_independent_xor_and_preserves_parent_identity() {
    for (poly, degree, seeds) in [
        (&[1u64, 1, 0, 0, 1][..], 4u32, &[0u64, 1, 2, 3, 7, 15][..]),
        (
            &[1u64, 1, 0, 1, 1, 0, 0, 0, 1][..],
            8u32,
            &[0u64, 1, 2, 3, 0x53, 0xca, 0xff][..],
        ),
    ] {
        let field = binary_field(poly);
        for &a in seeds {
            for b in 0..(1u64 << degree) {
                let mut modified = symbol(&field, a);
                modified.sub_assign(&symbol(&field, b)).unwrap();
                assert_eq!(
                    modified,
                    symbol(&field, a ^ b),
                    "GF(2^{degree}) a={a} b={b}"
                );
                modified.sub_assign(&field.zero()).unwrap();
                assert_eq!(modified, symbol(&field, a ^ b));
            }
        }
    }
    let a = binary_field(&[1, 1, 0, 0, 1]);
    let b = binary_field(&[1, 0, 0, 1, 1]);
    let mut candidate = a.one();
    let before = candidate.clone();
    assert!(candidate.sub_assign(&b.one()).is_err());
    assert_eq!(candidate, before);
}
