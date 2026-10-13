use perfect_error_correction::{CodeError, PrimeField, ReedSolomon, RsDecoderWorkspace, RsField};
use perfect_number_theory::{PrimalityResult, PrimeModulus, classify_u64};

const REFERENCE: &str = include_str!("../vectors/rs_gf17_n16_v1.tsv");

#[test]
fn release_scratch_and_recover_from_exact_external_reference() {
    let PrimalityResult::ProvenPrime(proof) = classify_u64(17) else {
        panic!("17 must be prime");
    };
    let field = RsField::Prime(PrimeField::new(PrimeModulus::from_proven(proof).unwrap()));
    let code = ReedSolomon::new(field.clone(), 16, 12, 1000).unwrap();
    let mut scratch = RsDecoderWorkspace::new(&code).unwrap();
    let row = REFERENCE
        .lines()
        .find(|s| !s.is_empty() && !s.starts_with('#'))
        .unwrap();
    let data: Vec<u64> = row.split_whitespace().map(|v| v.parse().unwrap()).collect();
    assert_eq!(data.len(), 28);
    let message: Vec<_> = data[..12]
        .iter()
        .map(|&v| field.ordinal(v).unwrap())
        .collect();
    let expected: Vec<_> = data[12..]
        .iter()
        .map(|&v| field.ordinal(v).unwrap())
        .collect();
    let mut corrupted = expected.clone();
    for i in [0, 15] {
        corrupted[i] = corrupted[i].add(&field.one()).unwrap();
    }
    assert_eq!(
        code.decode_with_workspace(&corrupted, &[], &mut scratch)
            .unwrap()
            .message,
        Some(message.clone())
    );
    let before = scratch.retained_element_capacity();
    scratch.release_transient_storage();
    let after = scratch.retained_element_capacity();
    assert!(after < before);
    assert!(after > 0);
    assert!(matches!(
        code.decode_with_workspace(&expected, &[16], &mut scratch),
        Err(CodeError::InvalidErasures)
    ));
    assert_eq!(
        code.decode_with_workspace(&corrupted, &[], &mut scratch)
            .unwrap()
            .message,
        Some(message.clone())
    );
    scratch.release_transient_storage();
    assert_eq!(
        code.decode_with_workspace(&expected, &[], &mut scratch)
            .unwrap()
            .message,
        Some(message)
    );
}
