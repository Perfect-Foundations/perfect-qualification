use perfect_error_correction::{
    CodeError, DecodeStatus, PrimeField, ReedSolomon, ReedSolomonCodeProfile, RsDecoderWorkspace,
    RsField,
};
use perfect_number_theory::{PrimalityResult, PrimeModulus, classify_u64};
const SOURCE: &str = include_str!("../vectors/rs_gf17_n16_v1.tsv");
fn prime(p: u64) -> RsField {
    let PrimalityResult::ProvenPrime(proof) = classify_u64(p) else {
        panic!("prime proof")
    };
    RsField::Prime(PrimeField::new(PrimeModulus::from_proven(proof).unwrap()))
}
#[test]
fn reusable_bw_workspace_matches_independent_gf17_codewords_and_recovers_after_failures() {
    let rs = ReedSolomon::new(prime(17), 16, 12, 1000).unwrap();
    let profile = ReedSolomonCodeProfile::from_code(&rs);
    let mut workspace = RsDecoderWorkspace::new(&rs).unwrap();
    let capacity = workspace.retained_element_capacity();
    let mut cases = 0;
    for row in SOURCE
        .lines()
        .filter(|s| !s.is_empty() && !s.starts_with('#'))
    {
        let data: Vec<u64> = row.split_whitespace().map(|s| s.parse().unwrap()).collect();
        assert_eq!(data.len(), 28);
        let message: Vec<_> = data[..12]
            .iter()
            .map(|&v| rs.field().ordinal(v).unwrap())
            .collect();
        let expected: Vec<_> = data[12..]
            .iter()
            .map(|&v| rs.field().ordinal(v).unwrap())
            .collect();
        assert_eq!(profile.encode_with(&rs, &message).unwrap(), expected);
        let mut corrupted = expected.clone();
        for i in [0, 15] {
            corrupted[i] = corrupted[i].add(&rs.field().one()).unwrap();
        }
        let corrected = profile.decode_with(&rs, &corrupted, &[]).unwrap();
        let reused = rs
            .decode_with_workspace(&corrupted, &[], &mut workspace)
            .unwrap();
        assert_eq!(reused, corrected);
        assert_eq!(reused.message, Some(message.clone()));
        assert_eq!(reused.codeword, Some(expected.clone()));
        assert!(matches!(reused.status, DecodeStatus::Corrected { .. }));
        let mut erased = expected.clone();
        for i in [0, 3, 11, 15] {
            erased[i] = rs.field().zero();
        }
        let result = rs
            .decode_with_workspace(&erased, &[0, 3, 11, 15], &mut workspace)
            .unwrap();
        assert_eq!(result.message, Some(message.clone()));
        assert_eq!(result.codeword, Some(expected.clone()));
        assert!(matches!(
            rs.decode_with_workspace(&corrupted, &[16], &mut workspace),
            Err(CodeError::InvalidErasures)
        ));
        assert_eq!(
            rs.decode_with_workspace(&expected, &[], &mut workspace)
                .unwrap()
                .message,
            Some(message)
        );
        assert_eq!(workspace.retained_element_capacity(), capacity);
        cases += 1;
    }
    assert_eq!(cases, 8);
    let mismatch = ReedSolomon::new(prime(19), 16, 12, 1000).unwrap();
    assert!(matches!(
        mismatch.decode_with_workspace(&[], &[], &mut workspace),
        Err(CodeError::IncompatibleProfile)
    ));
    assert!(matches!(
        profile.decode_with(&mismatch, &[], &[]),
        Err(CodeError::IncompatibleProfile)
    ));
}
