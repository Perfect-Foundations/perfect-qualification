use perfect_error_correction::{
    CodeError, PrimeField, ReedSolomon, RsBatchLimits, RsBatchWord, RsDecodeOutcome,
    RsDecodeSession, RsField, RsSymbol,
};
use perfect_number_theory::{PrimalityResult, PrimeModulus, classify_u64};
const GF17: &str = include_str!("../vectors/rs_gf17_n16_v1.tsv");
fn field(p: u64) -> RsField {
    let PrimalityResult::ProvenPrime(proof) = classify_u64(p) else {
        panic!("prime");
    };
    RsField::Prime(PrimeField::new(PrimeModulus::from_proven(proof).unwrap()))
}
fn word(f: &RsField, raw: &[u64]) -> Vec<RsSymbol> {
    raw.iter().map(|&n| f.ordinal(n).unwrap()).collect()
}
#[test]
fn wave19_session_batch_and_profile_rebind_match_independent_gf17() {
    let rs = ReedSolomon::new(field(17), 16, 12, 4096).unwrap();
    let row = GF17
        .lines()
        .find(|s| !s.is_empty() && !s.starts_with('#'))
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(row.len(), 28);
    let expected_message = word(rs.field(), &row[..12]);
    let expected_codeword = word(rs.field(), &row[12..]);
    assert_eq!(rs.encode(&expected_message).unwrap(), expected_codeword);
    let mut session = RsDecodeSession::new(rs).unwrap();
    session.decode(&expected_codeword, &[]).unwrap();
    let original_success = session.last_successful().clone();
    let mut error = expected_codeword.clone();
    error[6] = error[6].add(&session.code().field().one()).unwrap();
    let inputs = [
        RsBatchWord {
            received: &error,
            erasures: &[],
        },
        RsBatchWord {
            received: &expected_codeword[..15],
            erasures: &[],
        },
        RsBatchWord {
            received: &expected_codeword,
            erasures: &[],
        },
    ];
    let mut outcomes = Vec::new();
    let limits = RsBatchLimits {
        max_words: 3,
        max_input_items: 50,
        max_result_slots: 140,
    };
    session
        .decode_batch(&inputs, limits, &mut outcomes)
        .unwrap();
    assert_eq!(outcomes.len(), 3);
    assert!(matches!(outcomes[0], Ok(RsDecodeOutcome::Corrected)));
    assert!(matches!(outcomes[1], Err(CodeError::InvalidLength { .. })));
    assert!(matches!(outcomes[2], Ok(RsDecodeOutcome::Clean)));
    assert_eq!(session.batch_outputs().len(), 3);
    assert_eq!(session.batch_outputs()[0].message, expected_message);
    assert_eq!(session.batch_outputs()[0].codeword, expected_codeword);
    assert_eq!(session.batch_outputs()[0].corrected_positions, [6]);
    assert_eq!(*session.last_successful(), original_success);
    assert!(matches!(
        session.decode_batch(
            &inputs,
            RsBatchLimits {
                max_words: 2,
                ..limits
            },
            &mut outcomes
        ),
        Err(CodeError::BatchLimit {
            dimension: "words",
            ..
        })
    ));
    assert_eq!(session.batch_outputs().len(), 3);
    assert_eq!(outcomes.len(), 3);
    session
        .decode_batch(&inputs[..1], limits, &mut outcomes)
        .unwrap();
    assert_eq!(session.batch_outputs().len(), 1);
    session
        .decode_batch(
            &[],
            RsBatchLimits {
                max_words: 0,
                max_input_items: 0,
                max_result_slots: 0,
            },
            &mut outcomes,
        )
        .unwrap();
    assert!(session.batch_outputs().is_empty());
    let different = ReedSolomon::new(field(19), 16, 12, 4096).unwrap();
    session.rebind(different).unwrap();
    assert!(session.last_successful().message.is_empty());
    assert!(session.batch_outputs().is_empty());
    assert!(matches!(
        session.decode(&expected_codeword, &[]),
        Err(CodeError::IncompatibleField)
    ));
    let m = (0..12)
        .map(|n| session.code().field().ordinal(n).unwrap())
        .collect::<Vec<_>>();
    let codeword = session.code().encode(&m).unwrap();
    assert_eq!(
        session.decode(&codeword, &[]).unwrap(),
        RsDecodeOutcome::Clean
    );
    assert_eq!(session.last_successful().message, m);
}
