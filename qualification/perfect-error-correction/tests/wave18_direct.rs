use perfect_error_correction::{
    CodeError, ExtensionField, ExtensionLimits, Natural, PrimeField, ReedSolomon, RsBatchLimits,
    RsBatchWord, RsDecodeBuffers, RsDecodeOutcome, RsDecodeSession, RsDecoderWorkspace, RsField,
    RsSymbol,
};
use perfect_number_theory::{PrimalityResult, PrimeModulus, classify_u64};
const GF17: &str = include_str!("../vectors/rs_gf17_n16_v1.tsv");
const GF16: &str = include_str!("../vectors/rs_gf16_v1.tsv");
fn prime(p: u64) -> PrimeField {
    let PrimalityResult::ProvenPrime(proof) = classify_u64(p) else {
        panic!("prime")
    };
    PrimeField::new(PrimeModulus::from_proven(proof).unwrap())
}
fn word(field: &RsField, nums: &[u64]) -> Vec<RsSymbol> {
    nums.iter().map(|&i| field.ordinal(i).unwrap()).collect()
}
#[test]
fn independent_gf17_reusable_output_session_and_batch() {
    let code = ReedSolomon::new(RsField::Prime(prime(17)), 16, 12, 4096).unwrap();
    let mut workspace = RsDecoderWorkspace::new(&code).unwrap();
    let mut output = RsDecodeBuffers::default();
    let mut session = RsDecodeSession::new(code.clone()).unwrap();
    let mut sources = Vec::new();
    let mut inputs = Vec::new();
    let mut known = Vec::new();
    for (case, line) in GF17
        .lines()
        .filter(|x| !x.is_empty() && !x.starts_with('#'))
        .enumerate()
    {
        let data = line
            .split_whitespace()
            .map(|v| v.parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(data.len(), 28);
        let message = word(code.field(), &data[..12]);
        let encoded = word(code.field(), &data[12..]);
        assert_eq!(code.encode(&message).unwrap(), encoded);
        let mut received = encoded.clone();
        let erasures = match case % 4 {
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
        for _ in 0..2 {
            let result = code
                .decode_into_with_workspace(&received, &erasures, &mut workspace, &mut output)
                .unwrap();
            assert_eq!(output.message, message);
            assert_eq!(output.codeword, encoded);
            assert_eq!(
                result,
                if case % 4 == 0 {
                    RsDecodeOutcome::Clean
                } else {
                    RsDecodeOutcome::Corrected
                }
            );
            let session_status = session.decode(&received, &erasures).unwrap();
            assert_eq!(session_status, result);
            assert_eq!(session.last_successful(), &output);
        }
        sources.push((message, encoded));
        inputs.push(received);
        known.push(erasures);
    }
    assert_eq!(sources.len(), 8);
    let refs = (0..inputs.len())
        .map(|i| RsBatchWord {
            received: &inputs[i],
            erasures: &known[i],
        })
        .collect::<Vec<_>>();
    let limits = RsBatchLimits {
        max_words: 8,
        max_input_items: 170,
        max_result_slots: 352,
    };
    let mut outputs = Vec::new();
    let mut results = Vec::new();
    for _ in 0..2 {
        code.decode_batch_into_with_workspace(
            &refs,
            limits,
            &mut workspace,
            &mut outputs,
            &mut results,
        )
        .unwrap();
        assert_eq!(outputs.len(), 8);
        for (i, entry) in outputs.iter().enumerate() {
            assert_eq!(entry.message, sources[i].0);
            assert_eq!(entry.codeword, sources[i].1);
            assert!(matches!(
                results[i],
                Ok(RsDecodeOutcome::Clean | RsDecodeOutcome::Corrected)
            ));
        }
    }
    let original = outputs.clone();
    let old_count = results.len();
    assert!(matches!(
        code.decode_batch_into_with_workspace(
            &refs,
            RsBatchLimits {
                max_words: 7,
                ..limits
            },
            &mut workspace,
            &mut outputs,
            &mut results
        ),
        Err(CodeError::BatchLimit {
            dimension: "words",
            ..
        })
    ));
    assert_eq!(outputs, original);
    assert_eq!(results.len(), old_count);
    let received = inputs[0].clone();
    assert!(matches!(
        code.decode_into_with_workspace(&received[..15], &[], &mut workspace, &mut output),
        Err(CodeError::InvalidLength { .. })
    ));
    let before = output.clone();
    assert_eq!(
        code.decode_into_with_workspace(&received, &[0, 1, 2, 3, 4], &mut workspace, &mut output)
            .unwrap(),
        RsDecodeOutcome::DetectedUncorrectable
    );
    assert_eq!(output, before);
    workspace.release_transient_storage();
    assert_eq!(
        code.decode_into_with_workspace(&received, &[], &mut workspace, &mut output)
            .unwrap(),
        RsDecodeOutcome::Clean
    );
}
#[test]
fn independent_gf16_two_errors_and_erasure_mixture_stay_exact() {
    let base = prime(2);
    let modulus = [1u64, 1, 0, 0, 1]
        .iter()
        .map(|&x| base.element(Natural::from(x)).unwrap())
        .collect();
    let f =
        RsField::Extension(ExtensionField::new(base, modulus, ExtensionLimits::default()).unwrap());
    let code = ReedSolomon::new(f, 8, 4, 4096).unwrap();
    let mut ctx = RsDecodeSession::new(code.clone()).unwrap();
    let mut count = 0;
    for (i, line) in GF16
        .lines()
        .filter(|x| !x.is_empty() && !x.starts_with('#'))
        .enumerate()
    {
        let n = line
            .split_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(n.len(), 12);
        let message = word(code.field(), &n[..4]);
        let encoded = word(code.field(), &n[4..]);
        assert_eq!(code.encode(&message).unwrap(), encoded);
        let mut received = encoded.clone();
        let missing = if i % 2 == 0 {
            for j in [2, 6] {
                received[j] = received[j].add(&code.field().one()).unwrap();
            }
            vec![]
        } else {
            received[0] = code.field().zero();
            received[6] = received[6].add(&code.field().one()).unwrap();
            vec![0]
        };
        assert_eq!(
            ctx.decode(&received, &missing).unwrap(),
            RsDecodeOutcome::Corrected
        );
        assert_eq!(ctx.last_successful().message, message);
        assert_eq!(ctx.last_successful().codeword, encoded);
        count += 1;
    }
    assert_eq!(count, 4);
}
