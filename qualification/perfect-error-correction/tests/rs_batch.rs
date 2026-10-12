use perfect_error_correction::{
    CodeError, DecodeStatus, PrimeField, ReedSolomon, RsBatchLimits, RsBatchWord,
    RsDecoderWorkspace, RsField, RsSymbol,
};
use perfect_number_theory::{PrimalityResult, PrimeModulus, classify_u64};
const ORACLE: &str = include_str!("../vectors/rs_gf17_n16_v1.tsv");
fn field() -> RsField {
    let PrimalityResult::ProvenPrime(proof) = classify_u64(17) else {
        panic!()
    };
    RsField::Prime(PrimeField::new(PrimeModulus::from_proven(proof).unwrap()))
}
#[test]
fn bounded_batch_reuses_workspace_across_independent_gf17_words() {
    let code = ReedSolomon::new(field(), 16, 12, 1000).unwrap();
    let mut expected_messages: Vec<Vec<RsSymbol>> = Vec::new();
    let mut expected_words: Vec<Vec<RsSymbol>> = Vec::new();
    let mut corrupted: Vec<Vec<RsSymbol>> = Vec::new();
    let mut erasures: Vec<Vec<usize>> = Vec::new();
    for (case, row) in ORACLE
        .lines()
        .filter(|s| !s.is_empty() && !s.starts_with('#'))
        .enumerate()
    {
        let data: Vec<u64> = row.split_whitespace().map(|s| s.parse().unwrap()).collect();
        assert_eq!(data.len(), 28);
        let msg = data[..12]
            .iter()
            .map(|&x| code.field().ordinal(x).unwrap())
            .collect::<Vec<_>>();
        let original = data[12..]
            .iter()
            .map(|&x| code.field().ordinal(x).unwrap())
            .collect::<Vec<_>>();
        assert_eq!(code.encode(&msg).unwrap(), original);
        let mut received = original.clone();
        let missing = match case % 4 {
            0 => vec![],
            1 => {
                for i in [0, 15] {
                    received[i] = received[i].add(&code.field().one()).unwrap();
                }
                vec![]
            }
            2 => {
                for i in [0, 3, 11, 15] {
                    received[i] = code.field().zero();
                }
                vec![0, 3, 11, 15]
            }
            _ => {
                for i in [1, 4] {
                    received[i] = code.field().zero();
                }
                received[13] = received[13].add(&code.field().one()).unwrap();
                vec![1, 4]
            }
        };
        expected_messages.push(msg);
        expected_words.push(original);
        corrupted.push(received);
        erasures.push(missing);
    }
    assert_eq!(corrupted.len(), 8);
    let inputs = (0..8)
        .map(|i| RsBatchWord {
            received: &corrupted[i],
            erasures: &erasures[i],
        })
        .collect::<Vec<_>>();
    let limits = RsBatchLimits {
        max_words: 8,
        max_input_items: 150,
        max_result_slots: 352,
    };
    let mut workspace = RsDecoderWorkspace::new(&code).unwrap();
    assert!(matches!(
        code.decode_batch_with_workspace(
            &inputs,
            RsBatchLimits {
                max_words: 7,
                ..limits
            },
            &mut workspace
        ),
        Err(CodeError::BatchLimit {
            dimension: "words",
            ..
        })
    ));
    assert!(matches!(
        code.decode_batch_with_workspace(
            &inputs,
            RsBatchLimits {
                max_input_items: 1,
                ..limits
            },
            &mut workspace
        ),
        Err(CodeError::BatchLimit {
            dimension: "input_items",
            ..
        })
    ));
    for _ in 0..3 {
        let outcomes = code
            .decode_batch_with_workspace(&inputs, limits, &mut workspace)
            .unwrap();
        for (i, result) in outcomes.into_iter().enumerate() {
            let result = result.unwrap();
            assert_eq!(result.message, Some(expected_messages[i].clone()));
            assert_eq!(result.codeword, Some(expected_words[i].clone()));
            if i % 4 == 0 {
                assert_eq!(result.status, DecodeStatus::Clean);
            }
        }
    }
    let malformed = [RsSymbol::Prime(
        PrimeField::new({
            let PrimalityResult::ProvenPrime(proof) = classify_u64(17) else {
                panic!()
            };
            PrimeModulus::from_proven(proof).unwrap()
        })
        .zero(),
    )];
    let sequence = [
        RsBatchWord {
            received: &malformed,
            erasures: &[],
        },
        RsBatchWord {
            received: &expected_words[0],
            erasures: &[],
        },
    ];
    let outcomes = code
        .decode_batch_with_workspace(
            &sequence,
            RsBatchLimits {
                max_words: 2,
                max_input_items: 32,
                max_result_slots: 88,
            },
            &mut workspace,
        )
        .unwrap();
    assert!(matches!(outcomes[0], Err(CodeError::InvalidLength { .. })));
    assert_eq!(
        outcomes[1].as_ref().unwrap().message.as_ref(),
        Some(&expected_messages[0])
    );
}
#[test]
fn gf16_batch_matches_frozen_independent_polynomial_evaluations() {
    use perfect_error_correction::{ExtensionField, ExtensionLimits, Natural};
    const GF16: &str = include_str!("../vectors/rs_gf16_v1.tsv");
    let PrimalityResult::ProvenPrime(proof) = classify_u64(2) else {
        panic!("prime")
    };
    let base = PrimeField::new(PrimeModulus::from_proven(proof).unwrap());
    let modulus = [1u64, 1, 0, 0, 1]
        .into_iter()
        .map(|i| base.element(Natural::from(i)).unwrap())
        .collect();
    let field =
        RsField::Extension(ExtensionField::new(base, modulus, ExtensionLimits::default()).unwrap());
    let code = ReedSolomon::new(field, 8, 4, 1024).unwrap();
    let mut messages = Vec::new();
    let mut original = Vec::new();
    let mut received = Vec::new();
    let mut erasures = Vec::new();
    for (case, row) in GF16
        .lines()
        .filter(|s| !s.is_empty() && !s.starts_with('#'))
        .enumerate()
    {
        let nums: Vec<u64> = row.split_whitespace().map(|v| v.parse().unwrap()).collect();
        assert_eq!(nums.len(), 12);
        let message = nums[..4]
            .iter()
            .map(|&v| code.field().ordinal(v).unwrap())
            .collect::<Vec<_>>();
        let codeword = nums[4..]
            .iter()
            .map(|&v| code.field().ordinal(v).unwrap())
            .collect::<Vec<_>>();
        assert_eq!(code.encode(&message).unwrap(), codeword);
        let mut wrong = codeword.clone();
        let erased = if case % 2 == 0 {
            for i in [2, 6] {
                wrong[i] = wrong[i].add(&code.field().one()).unwrap();
            }
            vec![]
        } else {
            wrong[0] = code.field().zero();
            wrong[6] = wrong[6].add(&code.field().one()).unwrap();
            vec![0]
        };
        messages.push(message);
        original.push(codeword);
        received.push(wrong);
        erasures.push(erased);
    }
    assert_eq!(original.len(), 4);
    let words = (0..4)
        .map(|i| RsBatchWord {
            received: &received[i],
            erasures: &erasures[i],
        })
        .collect::<Vec<_>>();
    let limits = RsBatchLimits {
        max_words: 4,
        max_input_items: 40,
        max_result_slots: 100,
    };
    let mut workspace = RsDecoderWorkspace::new(&code).unwrap();
    for _ in 0..3 {
        let results = code
            .decode_batch_with_workspace(&words, limits, &mut workspace)
            .unwrap();
        for (i, result) in results.into_iter().enumerate() {
            let decoded = result.unwrap();
            assert_eq!(decoded.message, Some(messages[i].clone()));
            assert_eq!(decoded.codeword, Some(original[i].clone()));
        }
    }
}
