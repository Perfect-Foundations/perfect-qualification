//! Integer/rational-definite counterexamples, separate from valid PSD Gram vectors.
use perfect_uncertainty::{CorrelationMatrix, CorrelationPolicy, UncertaintyError};

fn policy(tolerance: f64) -> CorrelationPolicy {
    CorrelationPolicy {
        max_dimension: 16,
        psd_tolerance: tolerance,
    }
}
fn counterexample(n: usize, shift: usize, delta: f64) -> Vec<f64> {
    let mut data = vec![0.0; n * n];
    for i in 0..n {
        data[i * n + i] = 1.0;
    }
    let idx = [shift % n, (shift + 1) % n, (shift + 2) % n];
    // Valid correlation coefficient range and symmetry, but quadratic
    // witness g=(-2,1,1) has exact variance -2*delta, so matrix is not PSD.
    for (k, (a, b)) in [(0, 1), (0, 2), (1, 2)].iter().enumerate() {
        let correlation = if k == 2 { 1.0 - delta } else { 1.0 };
        let i = idx[*a];
        let j = idx[*b];
        data[i * n + j] = correlation;
        data[j * n + i] = correlation;
    }
    data
}
#[test]
fn invalid_gram_like_source_is_rejected_across_dimension_and_pivots() {
    for n in [3, 4, 6, 8, 12, 16] {
        for shift in 0..n {
            // Witness variance is -2^-31; well above the caller's 1e-12
            // PSD tolerance, so these must be rejected under that policy.
            let matrix = counterexample(n, shift, 2.0_f64.powi(-32));
            let result = CorrelationMatrix::new(n, matrix, policy(1e-12));
            assert!(
                matches!(
                    result,
                    Err(UncertaintyError::NotPositiveSemidefinite { .. })
                ),
                "invalid n={n}, shift={shift} accepted: {result:?}"
            );
        }
    }
}
#[test]
fn exact_rank_one_gram_is_psd_despite_singular_pivots() {
    for n in [3, 4, 6, 8, 12, 16] {
        let matrix = vec![1.0; n * n];
        assert!(
            CorrelationMatrix::new(n, matrix, policy(1e-12)).is_ok(),
            "rank-one valid matrix at n={n}"
        );
    }
}
