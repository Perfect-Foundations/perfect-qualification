use perfect_error_correction::{DecodeStatus, PrimeField, ReedSolomon, RsField};
use perfect_number_theory::{PrimalityResult, PrimeModulus, classify_u64};
const SOURCE: &str = include_str!("../vectors/rs_gf17_n16_v1.tsv");
fn gf17() -> RsField {
    let PrimalityResult::ProvenPrime(proof) = classify_u64(17) else {
        panic!("prime proof")
    };
    RsField::Prime(PrimeField::new(PrimeModulus::from_proven(proof).unwrap()))
}
#[test]
fn independent_gf17_high_degree_n16_exact_errors_and_erasures() {
    let rs = ReedSolomon::new(gf17(), 16, 12, 400).unwrap();
    let mut cases = 0;
    for (index, line) in SOURCE
        .lines()
        .filter(|s| !s.is_empty() && !s.starts_with('#'))
        .enumerate()
    {
        let entries: Vec<u64> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        assert_eq!(entries.len(), 28);
        let input: Vec<_> = entries[..12]
            .iter()
            .map(|&x| rs.field().ordinal(x).unwrap())
            .collect();
        let expected: Vec<_> = entries[12..]
            .iter()
            .map(|&x| rs.field().ordinal(x).unwrap())
            .collect();
        assert_eq!(rs.encode(&input).unwrap(), expected, "oracle row {index}");
        for (a, b) in [(0, 15), (6, 12)] {
            let mut received = expected.clone();
            received[a] = received[a].add(&rs.field().one()).unwrap();
            received[b] = received[b].add(&rs.field().one()).unwrap();
            let output = rs.decode(&received, &[]).unwrap();
            assert_eq!(
                output.message,
                Some(input.clone()),
                "case {index} errors {a},{b}"
            );
            assert_eq!(output.codeword, Some(expected.clone()));
            assert!(matches!(output.status, DecodeStatus::Corrected { .. }));
        }
        for erasures in [[0, 3, 11, 15], [15, 11, 3, 0]] {
            let mut received = expected.clone();
            for &loc in &erasures {
                received[loc] = rs.field().zero();
            }
            let output = rs.decode(&received, &erasures).unwrap();
            assert_eq!(output.message, Some(input.clone()));
            assert_eq!(output.codeword, Some(expected.clone()));
        }
        cases += 1;
    }
    assert_eq!(cases, 8);
}
