//! Exhaustive bounded GF(5) and GF(7) correction-radius oracle.
//! Evaluation f(x) is computed in primitive u64 modular arithmetic, not through
//! Algebra or the production Reed–Solomon encoder / Berlekamp–Welch solver.
use perfect_error_correction::{
    PrimeField, ReedSolomon, RsDecodeBuffers, RsDecodeOutcome, RsDecoderWorkspace, RsField,
    RsSymbol,
};
use perfect_number_theory::{PrimalityResult, PrimeModulus, classify_u64};

fn code(p: u64, n: usize, k: usize) -> ReedSolomon {
    let PrimalityResult::ProvenPrime(proof) = classify_u64(p) else {
        panic!("prime proof");
    };
    ReedSolomon::new(
        RsField::Prime(PrimeField::new(PrimeModulus::from_proven(proof).unwrap())),
        n,
        k,
        8192,
    )
    .unwrap()
}
fn expected(code: &ReedSolomon, coeffs: &[u64], prime: u64) -> Vec<RsSymbol> {
    (0..code.code_len())
        .map(|x| {
            let mut power = 1_u64;
            let mut value = 0_u64;
            for &c in coeffs {
                value = (value + c * power) % prime;
                power = (power * x as u64) % prime;
            }
            code.field().ordinal(value).unwrap()
        })
        .collect()
}
fn damaged(code: &ReedSolomon, original: &[RsSymbol], index: usize, delta: u64) -> Vec<RsSymbol> {
    let mut result = original.to_vec();
    let add = code.field().ordinal(delta).unwrap();
    result[index] = result[index].add(&add).unwrap();
    result
}
#[test]
fn exhaustive_gf5_all_messages_single_errors_and_two_erasure_symbols() {
    let code = code(5, 4, 2);
    let mut scratch = RsDecoderWorkspace::new(&code).unwrap();
    let mut output = RsDecodeBuffers::default();
    let mut verified = 0usize;
    for a in 0..5 {
        for b in 0..5 {
            let message = [a, b]
                .iter()
                .map(|&v| code.field().ordinal(v).unwrap())
                .collect::<Vec<_>>();
            let original = expected(&code, &[a, b], 5);
            assert_eq!(code.encode(&message).unwrap(), original);
            assert_eq!(
                code.decode_into_with_workspace(&original, &[], &mut scratch, &mut output)
                    .unwrap(),
                RsDecodeOutcome::Clean
            );
            assert_eq!(output.message, message);
            for index in 0..4 {
                for delta in 1..5 {
                    let corrupted = damaged(&code, &original, index, delta);
                    assert_eq!(
                        code.decode_into_with_workspace(&corrupted, &[], &mut scratch, &mut output)
                            .unwrap(),
                        RsDecodeOutcome::Corrected,
                        "message={a},{b},index={index},delta={delta}"
                    );
                    assert_eq!(output.message, message);
                    assert_eq!(output.codeword, original);
                    assert_eq!(output.corrected_positions, [index]);
                    verified += 1;
                }
            }
            for i in 0..4 {
                for j in i + 1..4 {
                    for di in 1..5 {
                        for dj in 1..5 {
                            let first = damaged(&code, &original, i, di);
                            let both = damaged(&code, &first, j, dj);
                            assert_eq!(
                                code.decode_into_with_workspace(
                                    &both,
                                    &[j, i],
                                    &mut scratch,
                                    &mut output
                                )
                                .unwrap(),
                                RsDecodeOutcome::Corrected,
                                "message={a},{b},erasures={i},{j},delta={di},{dj}"
                            );
                            assert_eq!(output.message, message);
                            assert_eq!(output.codeword, original);
                            assert_eq!(output.corrected_positions, [i, j]);
                            verified += 1;
                        }
                    }
                }
            }
        }
    }
    assert_eq!(verified, 2800); // 25 * (4*4 + C(4,2)*4*4)
}
#[test]
fn exhaustive_gf7_mixed_one_error_one_erasure_and_refusal_atomicity() {
    let code = code(7, 5, 2);
    let mut workspace = RsDecoderWorkspace::new(&code).unwrap();
    let mut output = RsDecodeBuffers {
        message: vec![code.field().one()],
        codeword: vec![code.field().one()],
        corrected_positions: vec![999],
    };
    let mut verified = 0usize;
    for a in 0..7 {
        for b in 0..7 {
            let message = [a, b]
                .iter()
                .map(|&v| code.field().ordinal(v).unwrap())
                .collect::<Vec<_>>();
            let original = expected(&code, &[a, b], 7);
            assert_eq!(code.encode(&message).unwrap(), original);
            for error in 0..5 {
                for erasure in 0..5 {
                    if error == erasure {
                        continue;
                    }
                    for delta in 1..7 {
                        let received =
                            damaged(&code, &damaged(&code, &original, error, delta), erasure, 1);
                        let status = code
                            .decode_into_with_workspace(
                                &received,
                                &[erasure],
                                &mut workspace,
                                &mut output,
                            )
                            .unwrap();
                        assert_eq!(
                            status,
                            RsDecodeOutcome::Corrected,
                            "message={a},{b},error={error},erasure={erasure},delta={delta}"
                        );
                        assert_eq!(output.message, message);
                        assert_eq!(output.codeword, original);
                        let mut positions = vec![error, erasure];
                        positions.sort_unstable();
                        assert_eq!(output.corrected_positions, positions);
                        verified += 1;
                    }
                }
            }
            let before = output.clone();
            assert!(
                code.decode_into_with_workspace(&original, &[5], &mut workspace, &mut output)
                    .is_err()
            );
            assert_eq!(output, before);
        }
    }
    assert_eq!(verified, 5880); // 49 * 5 * 4 * 6
}
