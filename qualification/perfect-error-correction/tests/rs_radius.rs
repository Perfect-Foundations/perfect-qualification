//! Independent GF(5) polynomial-evaluation vectors, full declared RS
//! correction-radius boundaries, erasures and deterministic statuses.
use perfect_algebra::PrimeField;
use perfect_error_correction::{CodeError, DecodeStatus, ReedSolomon, RsField, RsSymbol};
use perfect_number_theory::{PrimalityResult, PrimeModulus, classify_u64};
const GF5: &str = include_str!("../vectors/rs_gf5_v1.tsv");
fn field() -> RsField {
    let PrimalityResult::ProvenPrime(proof) = classify_u64(5) else {
        panic!("proof")
    };
    RsField::Prime(PrimeField::new(PrimeModulus::from_proven(proof).unwrap()))
}
fn symbol(rs: &ReedSolomon, n: u64) -> RsSymbol {
    rs.field().ordinal(n).unwrap()
}
#[test]
fn all_gf5_single_error_single_erasure_patterns_within_two_e_plus_v_bound() {
    let rs = ReedSolomon::new(field(), 5, 2, 1000).unwrap();
    let mut actual_patterns = 0;
    let mut messages = 0;
    for row in GF5.lines().filter(|line| !line.starts_with('#')) {
        let nums: Vec<u64> = row.split_whitespace().map(|s| s.parse().unwrap()).collect();
        let msg = [symbol(&rs, nums[0]), symbol(&rs, nums[1])];
        let original = rs.encode(&msg).unwrap();
        // The independent frozen corpus contains GF(5) evaluations at
        // x=0..3; the fifth point x=4 follows from the *same independent*
        // linear-polynomial input coefficients, without using Rust as oracle.
        for i in 0..4 {
            assert_eq!(original[i], symbol(&rs, nums[2 + i]));
        }
        assert_eq!(original[4], symbol(&rs, (nums[0] + 4 * nums[1]) % 5));
        for error_position in 0..5 {
            for erasure_position in 0..5 {
                if error_position == erasure_position {
                    continue;
                }
                for delta in 1..5 {
                    let mut received = original.clone();
                    let updated = (nums[0] + nums[1] * (error_position as u64) + delta) % 5;
                    received[error_position] = symbol(&rs, updated);
                    received[erasure_position] = rs.field().zero();
                    let result = rs.decode(&received, &[erasure_position]).unwrap();
                    assert_eq!(result.message, Some(msg.to_vec()));
                    assert_eq!(result.codeword, Some(original.clone()));
                    assert!(matches!(result.status, DecodeStatus::Corrected { .. }));
                    actual_patterns += 1;
                }
            }
        }
        for a in 0..5 {
            for b in a + 1..5 {
                for c in b + 1..5 {
                    let mut received = original.clone();
                    for index in [a, b, c] {
                        received[index] = rs.field().zero();
                    }
                    // 0 hard errors, 3 supplied erasures meets parity n-k=3.
                    let first = rs.decode(&received, &[a, b, c]).unwrap();
                    let reverse = rs.decode(&received, &[c, b, a]).unwrap();
                    assert_eq!(first, reverse);
                    assert_eq!(first.message, Some(msg.to_vec()));
                    assert_eq!(first.codeword, Some(original.clone()));
                    actual_patterns += 1;
                }
            }
        }
        messages += 1;
    }
    assert_eq!(messages, 25);
    assert_eq!(actual_patterns, 2250); // 2000 mixed error/erasure + 250 erasure-only
}
#[test]
fn invalid_erasure_locations_and_explicit_candidate_limits() {
    let rs = ReedSolomon::new(field(), 5, 2, 1).unwrap();
    let word = rs.encode(&[symbol(&rs, 1), symbol(&rs, 2)]).unwrap();
    assert!(matches!(
        rs.decode(&word, &[1, 1]),
        Err(CodeError::InvalidErasures)
    ));
    assert!(matches!(
        rs.decode(&word, &[5]),
        Err(CodeError::InvalidErasures)
    ));
    let mut received = word.clone();
    received[0] = received[0].add(&rs.field().one()).unwrap();
    received[3] = rs.field().zero();
    assert!(matches!(
        rs.decode(&received, &[3]),
        Err(CodeError::ResourceLimit { max_trials: 1 })
    ));
}

#[test]
fn advertised_mds_capacity_matches_independent_distance_formula() {
    for n in 2..=5 {
        for k in 1..n {
            let rs = ReedSolomon::new(field(), n, k, 1000).unwrap();
            assert_eq!(rs.minimum_distance(), n - k + 1);
            for v in 0..=n + 1 {
                let expected = if v > n - k {
                    None
                } else {
                    Some((n - k - v) / 2)
                };
                assert_eq!(rs.correction_capacity(v), expected);
                if let Some(e) = expected {
                    assert!(2 * e + v <= n - k);
                    assert!(2 * (e + 1) + v > n - k);
                }
            }
        }
    }
}
