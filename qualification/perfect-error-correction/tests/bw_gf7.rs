use perfect_error_correction::{CodeError, DecodeStatus, PrimeField, ReedSolomon, RsField};
use perfect_number_theory::{PrimalityResult, PrimeModulus, classify_u64};

const SOURCE: &str = include_str!("../vectors/rs_bw_gf7_v1.tsv");

fn gf7() -> RsField {
    let PrimalityResult::ProvenPrime(proof) = classify_u64(7) else {
        panic!("GF7 requires verified prime proof")
    };
    RsField::Prime(PrimeField::new(PrimeModulus::from_proven(proof).unwrap()))
}

#[test]
fn independently_calculated_gf7_bw_error_erasure_boundary() {
    let field = gf7();
    let code = ReedSolomon::new(field.clone(), 6, 3, 400).unwrap();
    let mut rows = 0;
    for line in SOURCE
        .lines()
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
    {
        let values: Vec<u64> = line
            .split_whitespace()
            .map(|text| text.parse::<u64>().unwrap())
            .collect();
        assert_eq!(values.len(), 27);
        let message: Vec<_> = values[..3]
            .iter()
            .map(|&v| field.ordinal(v).unwrap())
            .collect();
        let codeword: Vec<_> = values[3..9]
            .iter()
            .map(|&v| field.ordinal(v).unwrap())
            .collect();
        let received: Vec<_> = values[9..15]
            .iter()
            .map(|&v| field.ordinal(v).unwrap())
            .collect();
        let erasures: Vec<usize> = values[15..21]
            .iter()
            .enumerate()
            .filter_map(|(i, &flag)| (flag == 1).then_some(i))
            .collect();
        let corrected: Vec<usize> = values[21..27]
            .iter()
            .enumerate()
            .filter_map(|(i, &flag)| (flag == 1).then_some(i))
            .collect();
        assert!(values[15..].iter().all(|&v| v <= 1));
        assert_eq!(
            code.encode(&message).unwrap(),
            codeword,
            "independent row {rows}"
        );
        let decoded = code.decode(&received, &erasures).unwrap();
        assert_eq!(decoded.message, Some(message.clone()), "row {rows}");
        assert_eq!(decoded.codeword, Some(codeword), "row {rows}");
        assert_eq!(
            decoded.status,
            if corrected.is_empty() {
                DecodeStatus::Clean
            } else {
                DecodeStatus::Corrected {
                    positions: corrected,
                }
            },
            "row {rows}"
        );
        if erasures.is_empty() && received != code.encode(&message).unwrap() {
            let limited = ReedSolomon::new(field.clone(), 6, 3, 1).unwrap();
            assert!(matches!(
                limited.decode(&received, &[]),
                Err(CodeError::ResourceLimit { max_trials: 1 })
            ));
        }
        rows += 1;
    }
    assert_eq!(rows, 16);
}
