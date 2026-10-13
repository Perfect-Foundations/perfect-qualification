//! Independent GF(7) evaluation oracle with seeded decoder reuse.
//! Fixed generator; every reference codeword is computed with u64 mod 7.
use perfect_error_correction::{
    CodeError, PrimeField, ReedSolomon, RsBatchLimits, RsBatchWord, RsDecodeOutcome,
    RsDecodeSession, RsField,
};
use perfect_number_theory::{PrimalityResult, PrimeModulus, classify_u64};
const SEED: u64 = 0x2200_aa75_0199_33c4;
fn next(s: &mut u64) -> u64 {
    *s ^= *s >> 12;
    *s ^= *s << 25;
    *s ^= *s >> 27;
    s.wrapping_mul(0x2545f4914f6cdd1d)
}
#[test]
fn seeded_gf7_evaluation_unknown_errors_and_mixed_erasures() {
    let PrimalityResult::ProvenPrime(proof) = classify_u64(7) else {
        panic!("proof");
    };
    let f = RsField::Prime(PrimeField::new(PrimeModulus::from_proven(proof).unwrap()));
    let code = ReedSolomon::new(f, 6, 3, 8192).unwrap();
    let mut session = RsDecodeSession::new(code.clone()).unwrap();
    let mut seed = SEED;
    for trial in 0..84 {
        let c = [
            next(&mut seed) % 7,
            next(&mut seed) % 7,
            next(&mut seed) % 7,
        ];
        let message = c
            .iter()
            .map(|&v| code.field().ordinal(v).unwrap())
            .collect::<Vec<_>>();
        let oracle = (0..6)
            .map(|x| {
                let x = x as u64;
                code.field()
                    .ordinal((c[0] + c[1] * x + c[2] * x * x) % 7)
                    .unwrap()
            })
            .collect::<Vec<_>>();
        assert_eq!(
            code.encode(&message).unwrap(),
            oracle,
            "trial={trial}: independent evaluation"
        );
        assert_eq!(
            session.decode(&oracle, &[]).unwrap(),
            RsDecodeOutcome::Clean
        );
        assert_eq!(session.last_successful().message, message);
        for wrong in 0..6 {
            let mut received = oracle.clone();
            let delta = 1 + next(&mut seed) % 6;
            let ordinal =
                ((c[0] + c[1] * wrong as u64 + c[2] * (wrong * wrong) as u64) % 7 + delta) % 7;
            received[wrong] = code.field().ordinal(ordinal).unwrap();
            assert_eq!(
                session.decode(&received, &[]).unwrap(),
                RsDecodeOutcome::Corrected,
                "trial={trial}: unknown error {wrong}"
            );
            assert_eq!(session.last_successful().codeword, oracle);
            assert_eq!(session.last_successful().corrected_positions, [wrong]);
            for erasure in 0..6 {
                if erasure == wrong {
                    continue;
                }
                let mut damaged = received.clone();
                damaged[erasure] = code.field().zero();
                assert_eq!(
                    session.decode(&damaged, &[erasure]).unwrap(),
                    RsDecodeOutcome::Corrected,
                    "trial={trial}: mixed {wrong} {erasure}"
                );
                assert_eq!(session.last_successful().message, message);
                assert_eq!(session.last_successful().codeword, oracle);
                let mut positions = vec![wrong, erasure];
                positions.sort_unstable();
                assert_eq!(session.last_successful().corrected_positions, positions);
            }
        }
    }
}
#[test]
fn seeded_batch_admission_refusal_and_release_preserve_success() {
    let PrimalityResult::ProvenPrime(proof) = classify_u64(7) else {
        panic!("proof");
    };
    let f = RsField::Prime(PrimeField::new(PrimeModulus::from_proven(proof).unwrap()));
    let code = ReedSolomon::new(f, 6, 3, 8192).unwrap();
    let message = [1, 2, 3]
        .iter()
        .map(|&i| code.field().ordinal(i).unwrap())
        .collect::<Vec<_>>();
    let word = code.encode(&message).unwrap();
    let mut session = RsDecodeSession::new(code).unwrap();
    session.decode(&word, &[]).unwrap();
    let good = session.last_successful().clone();
    let mut bad = word.clone();
    bad[1] = bad[1].add(&session.code().field().one()).unwrap();
    let words = [
        RsBatchWord {
            received: &word,
            erasures: &[],
        },
        RsBatchWord {
            received: &bad,
            erasures: &[],
        },
        RsBatchWord {
            received: &word[..5],
            erasures: &[],
        },
    ];
    let mut results = Vec::new();
    let limits = RsBatchLimits {
        max_words: 3,
        max_input_items: 18,
        max_result_slots: 72,
    };
    session.decode_batch(&words, limits, &mut results).unwrap();
    assert_eq!(results.len(), 3);
    assert!(matches!(results[0], Ok(RsDecodeOutcome::Clean)));
    assert!(matches!(results[1], Ok(RsDecodeOutcome::Corrected)));
    assert!(matches!(results[2], Err(CodeError::InvalidLength { .. })));
    assert_eq!(session.batch_outputs().len(), 3);
    assert_eq!(session.last_successful(), &good);
    assert!(matches!(
        session.decode_batch(
            &words,
            RsBatchLimits {
                max_words: 2,
                ..limits
            },
            &mut results
        ),
        Err(CodeError::BatchLimit {
            dimension: "words",
            ..
        })
    ));
    assert_eq!(results.len(), 3);
    assert_eq!(session.batch_outputs().len(), 3);
    session.release_transient_storage();
    session
        .decode_batch(&words[..1], limits, &mut results)
        .unwrap();
    assert_eq!(session.batch_outputs().len(), 1);
    session.release_batch_output_storage();
    assert_eq!(session.retained_batch_output_capacity(), 0);
    assert_eq!(session.last_successful(), &good);
}
