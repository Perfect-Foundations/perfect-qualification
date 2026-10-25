use perfect_constants::math::{
    ALL, APERY_ZETA3_F32, APERY_ZETA3_F64, CATALAN_F32, CATALAN_F64, ConstantId, E_F32, E_F64,
    EULER_MASCHERONI_F32, EULER_MASCHERONI_F64, GOLDEN_RATIO_F32, GOLDEN_RATIO_F64, LN2_F32,
    LN2_F64, SQRT2_F32, SQRT2_F64,
};
use qualify_perfect_constants::{first_const_value, semantic_fingerprint};

const REFERENCE: &str = include_str!("../vectors/native.tsv");
const EXPECTED_FINGERPRINT: &str = include_str!("../semantic-fingerprint.txt");
const CONST_E: (f32, f64) = first_const_value();

fn entries() -> impl Iterator<Item = (&'static str, u32, u64)> {
    REFERENCE
        .lines()
        .filter(|s| !s.is_empty() && !s.starts_with('#'))
        .map(|line| {
            let mut parts = line.split_ascii_whitespace();
            let name = parts.next().unwrap();
            let binary32 = u32::from_str_radix(parts.next().unwrap(), 16).unwrap();
            let binary64 = u64::from_str_radix(parts.next().unwrap(), 16).unwrap();
            assert!(parts.next().is_none());
            (name, binary32, binary64)
        })
}

#[test]
fn public_ids_and_reference_vectors_are_complete_and_unique() {
    let cases: Vec<_> = entries().collect();
    assert_eq!(cases.len(), 7);
    assert_eq!(ALL.len(), cases.len());
    for (idx, (name, f32bits, f64bits)) in cases.into_iter().enumerate() {
        let id = ALL[idx];
        assert_eq!(id.name(), name);
        assert_eq!(id.as_f32().to_bits(), f32bits, "{name}");
        assert_eq!(id.as_f64().to_bits(), f64bits, "{name}");
        for prior in &ALL[..idx] {
            assert_ne!(*prior, id);
            assert_ne!(prior.name(), id.name());
        }
    }
}

#[test]
fn all_seven_documented_public_variants_have_stable_names() {
    assert_eq!(
        ALL,
        [
            ConstantId::E,
            ConstantId::GoldenRatio,
            ConstantId::Sqrt2,
            ConstantId::Ln2,
            ConstantId::EulerMascheroni,
            ConstantId::Catalan,
            ConstantId::AperyZeta3,
        ]
    );
}

#[test]
fn public_named_values_agree_with_public_id_retrieval() {
    let expected = [
        (E_F32, E_F64),
        (GOLDEN_RATIO_F32, GOLDEN_RATIO_F64),
        (SQRT2_F32, SQRT2_F64),
        (LN2_F32, LN2_F64),
        (EULER_MASCHERONI_F32, EULER_MASCHERONI_F64),
        (CATALAN_F32, CATALAN_F64),
        (APERY_ZETA3_F32, APERY_ZETA3_F64),
    ];
    for (id, (f32value, f64value)) in ALL.into_iter().zip(expected) {
        assert_eq!(id.as_f32().to_bits(), f32value.to_bits());
        assert_eq!(id.as_f64().to_bits(), f64value.to_bits());
    }
}

#[test]
fn positive_finite_values_obey_independent_order_and_ranges() {
    for id in ALL {
        assert!(id.as_f32().is_finite() && id.as_f32().is_sign_positive());
        assert!(id.as_f64().is_finite() && id.as_f64().is_sign_positive());
    }
    let observed: Vec<f64> = ALL.iter().map(|id| id.as_f64()).collect();
    assert!(observed[0] > 2.0 && observed[0] < 3.0);
    assert!(observed[1] > 1.0 && observed[1] < 2.0);
    assert!(observed[2] > 1.0 && observed[2] < observed[1]);
    assert!(observed[3] > 0.5 && observed[3] < 1.0);
    assert!(observed[4] > 0.5 && observed[4] < observed[3]);
    assert!(observed[5] > observed[3] && observed[5] < 1.0);
    assert!(observed[6] > 1.0 && observed[6] < observed[2]);
}

#[test]
fn semantic_fingerprint_matches_independent_reference_not_production_output() {
    let expected = u64::from_str_radix(EXPECTED_FINGERPRINT.trim(), 16).unwrap();
    assert_eq!(semantic_fingerprint(), expected);
}

#[test]
fn const_evaluation_and_explicit_namespace_boundary() {
    assert_eq!(CONST_E.0.to_bits(), 0x402df854);
    assert_eq!(CONST_E.1.to_bits(), 0x4005bf0a8b145769);
    // No pi or CODATA path is imported. Absence of an identifier cannot
    // be *proved* by these runtime tests; the public source/API audit is separate.
}
