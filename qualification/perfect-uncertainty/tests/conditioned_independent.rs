use perfect_uncertainty::{CorrelationMatrix, CorrelationPolicy, propagate_first_order};

const VECTORS: &str = include_str!("../vectors/conditioned_4_6_8_v1.tsv");

fn parse_bits(token: &str) -> f64 {
    f64::from_bits(u64::from_str_radix(token, 16).expect("frozen source f64 bits"))
}
fn check(actual: f64, expected: f64, vector: &str) {
    assert!(actual.is_finite() && actual >= 0.0, "{vector}: {actual:?}");
    if expected == 0.0 {
        // The exactly rank-one zero-variance integer cancellations should
        // never invent a positive uncertainty at these tested magnitudes.
        assert_eq!(actual, 0.0, "{vector}");
    } else {
        let relative = (actual - expected).abs() / expected;
        assert!(
            relative <= 1e-10,
            "{vector}: calculated={actual:e}, exact_binary64_reference={expected:e}, relative_error={relative:e}"
        );
    }
}
#[test]
fn independent_fraction_conditioned_900_high_dimension_cases() {
    let mut count = 0;
    for row in VECTORS
        .lines()
        .filter(|r| !r.is_empty() && !r.starts_with('#'))
    {
        let fields: Vec<_> = row.split_ascii_whitespace().collect();
        assert_eq!(fields.len(), 4, "{row}");
        let n: usize = fields[0].parse().unwrap();
        assert!(matches!(n, 4 | 6 | 8));
        let rho = parse_bits(fields[1]);
        let contributions: Vec<_> = fields[2].split(',').map(parse_bits).collect();
        assert_eq!(contributions.len(), n);
        let expected = parse_bits(fields[3]);
        let correlation = CorrelationMatrix::new(
            n,
            (0..n)
                .flat_map(|i| (0..n).map(move |j| if i == j { 1.0 } else { rho }))
                .collect(),
            CorrelationPolicy {
                max_dimension: 8,
                psd_tolerance: 1e-12,
            },
        )
        .expect("independently constructed PSD equicorrelation family");
        let result = propagate_first_order(&contributions, &correlation).unwrap();
        check(result.variance(), expected, row);
        count += 1;
    }
    assert_eq!(count, 900);
}
#[test]
fn equicorrelation_permutation_invariance_in_supported_ranges() {
    let mut count = 0;
    for row in VECTORS
        .lines()
        .filter(|r| !r.is_empty() && !r.starts_with('#'))
    {
        let fields: Vec<_> = row.split_ascii_whitespace().collect();
        let n: usize = fields[0].parse().unwrap();
        let rho = parse_bits(fields[1]);
        let mut contributions: Vec<_> = fields[2].split(',').map(parse_bits).collect();
        let expected = parse_bits(fields[3]);
        let correlation = CorrelationMatrix::new(
            n,
            (0..n)
                .flat_map(|i| (0..n).map(move |j| if i == j { 1.0 } else { rho }))
                .collect(),
            CorrelationPolicy {
                max_dimension: 8,
                psd_tolerance: 1e-12,
            },
        )
        .unwrap();
        contributions.reverse();
        let reversed = propagate_first_order(&contributions, &correlation).unwrap();
        check(reversed.variance(), expected, row);
        count += 1;
    }
    assert_eq!(count, 900);
}
