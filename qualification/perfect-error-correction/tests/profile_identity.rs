use perfect_error_correction::{
    Hamming, HammingCodeProfile, OverallParityPlacement, PackedBitOrder, PrimeField, ReedSolomon,
    ReedSolomonCodeProfile, RsCoefficientOrder, RsEvaluationConvention, RsField, RsSymbolMapping,
};
use perfect_number_theory::{PrimalityResult, PrimeModulus, classify_u64};
fn prime(p: u64) -> RsField {
    let PrimalityResult::ProvenPrime(proof) = classify_u64(p) else {
        panic!("prime proof missing")
    };
    RsField::Prime(PrimeField::new(PrimeModulus::from_proven(proof).unwrap()))
}
#[test]
fn exact_profile_identity_and_incompatible_codes_refuse() {
    for extended in [false, true] {
        for order in [
            PackedBitOrder::MostSignificantFirst,
            PackedBitOrder::LeastSignificantFirst,
        ] {
            let code = Hamming::new(3, extended).unwrap();
            let profile = HammingCodeProfile::new(code, order);
            assert_eq!(profile.code(), code);
            assert_eq!(profile.packing(), order);
            assert_eq!(
                profile.overall_parity_placement(),
                if extended {
                    OverallParityPlacement::FinalLogicalBit
                } else {
                    OverallParityPlacement::Absent
                }
            );
        }
    }
    let original = ReedSolomon::new(prime(7), 6, 3, 20).unwrap();
    let identity = ReedSolomonCodeProfile::from_code(&original);
    assert!(identity.matches(&original));
    assert_eq!(identity.minimum_distance(), 4);
    assert_eq!(identity.correction_capacity(1), Some(1));
    assert_eq!(
        identity.evaluation_convention(),
        RsEvaluationConvention::CanonicalFieldOrdinals
    );
    assert_eq!(
        identity.coefficient_order(),
        RsCoefficientOrder::AscendingDegree
    );
    assert_eq!(
        identity.symbol_mapping(),
        RsSymbolMapping::PerfectAlgebraCanonicalOrdinal
    );
    assert!(!identity.matches(&ReedSolomon::new(prime(7), 6, 2, 20).unwrap()));
    assert!(!identity.matches(&ReedSolomon::new(prime(7), 5, 3, 20).unwrap()));
    assert!(!identity.matches(&ReedSolomon::new(prime(11), 6, 3, 20).unwrap()));
    assert!(ReedSolomonCodeProfile::new(prime(5), 6, 3).is_err());
}
