//! Source-independent exact-dyadic Gram PSD and first-order covariance.
use perfect_uncertainty::{CorrelationMatrix, CorrelationPolicy, propagate_first_order};
const CASES: &str = include_str!("../vectors/gram_covariance_v1.tsv");
fn parse_bits(value: &str) -> f64 {
    f64::from_bits(u64::from_str_radix(value, 16).unwrap())
}
fn run_case(row: &str, reverse: bool) {
    let fields: Vec<&str> = row.split_ascii_whitespace().collect();
    let n: usize = fields[0].parse().unwrap();
    assert!((3..=16).contains(&n));
    assert_eq!(fields.len(), 3 + n * n + n);
    let label = fields[1];
    let input_matrix: Vec<f64> = fields[2..2 + n * n].iter().map(|s| parse_bits(s)).collect();
    let input_g: Vec<f64> = fields[2 + n * n..2 + n * n + n]
        .iter()
        .map(|s| parse_bits(s))
        .collect();
    let expected = parse_bits(fields[2 + n * n + n]);
    assert!(expected.is_finite() && expected > 0.0);
    let mut r = vec![0.0; n * n];
    let mut g = vec![0.0; n];
    for i in 0..n {
        let src = if reverse { n - 1 - i } else { i };
        g[i] = input_g[src];
        for j in 0..n {
            let col = if reverse { n - 1 - j } else { j };
            r[i * n + j] = input_matrix[src * n + col];
        }
    }
    let mat = CorrelationMatrix::new(
        n,
        r,
        CorrelationPolicy {
            max_dimension: 16,
            psd_tolerance: 1e-12,
        },
    )
    .unwrap_or_else(|error| {
        panic!("valid exact PSD Gram rejected case={label} reversed={reverse} error={error:?}")
    });
    let out = propagate_first_order(&g, &mat).unwrap_or_else(|error| {
        panic!("propagation failed case={label} reverse={reverse} error={error:?}")
    });
    let rel = ((out.variance() - expected) / expected).abs();
    assert!(
        out.variance().is_finite() && rel <= 1e-10,
        "case={label} reversed={reverse} n={n} relative={rel} expected={expected:e} actual={:e}",
        out.variance()
    );
    assert!(!out.clamped_roundoff(), "unexpected clamping case={label}");
}
#[test]
fn exact_1601_psd_gram_variances_and_reverse_permutations() {
    let mut count = 0;
    for row in CASES
        .lines()
        .filter(|s| !s.is_empty() && !s.starts_with('#'))
    {
        run_case(row, false);
        run_case(row, true);
        count += 1;
    }
    assert_eq!(count, 1601);
}
