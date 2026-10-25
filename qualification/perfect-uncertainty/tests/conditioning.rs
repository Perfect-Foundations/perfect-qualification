//! Independently derived exact binary64-input Fraction oracle over
//! near-singular, rank-one and mixed-scale equicorrelation matrices.
use perfect_uncertainty::{CorrelationMatrix, CorrelationPolicy, propagate_first_order};
const VECTORS: &str = include_str!("../vectors/conditioning_v1.tsv");
fn from_hex(value: &str) -> f64 {
    f64::from_bits(u64::from_str_radix(value, 16).unwrap())
}
#[test]
fn independent_binary64_fraction_oracle_ill_conditioned_234_cases() {
    let mut count = 0;
    for row in VECTORS
        .lines()
        .filter(|s| !s.is_empty() && !s.starts_with('#'))
    {
        let data: Vec<&str> = row.split_ascii_whitespace().collect();
        assert_eq!(data.len(), 7, "malformed independent oracle row");
        let n: usize = data[0].parse().unwrap();
        assert!((2..=4).contains(&n));
        let rho = from_hex(data[1]);
        let contributions: Vec<f64> = data[2..2 + n].iter().map(|x| from_hex(x)).collect();
        let expected = from_hex(data[6]);
        assert!(expected.is_finite() && expected > 0.0);
        let mut matrix = vec![rho; n * n];
        for i in 0..n {
            matrix[i * n + i] = 1.0;
        }
        let valid = CorrelationMatrix::new(
            n,
            matrix,
            CorrelationPolicy {
                max_dimension: 4,
                psd_tolerance: 1e-12,
            },
        )
        .unwrap();
        let actual = propagate_first_order(&contributions, &valid).unwrap();
        let relerr = ((actual.variance() - expected) / expected).abs();
        assert!(
            actual.variance().is_finite() && relerr <= 1e-10,
            "case {count}, n={n}, rho={rho}, g={contributions:?}, actual={}, exact_f64={}, relative_error={relerr}",
            actual.variance(),
            expected
        );
        assert!(
            !actual.clamped_roundoff(),
            "unexpected approximate clamp: {row}"
        );
        count += 1;
    }
    assert_eq!(count, 234);
}
