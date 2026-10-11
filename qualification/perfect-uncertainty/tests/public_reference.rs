use perfect_uncertainty::{
    CorrelationMatrix, CorrelationPolicy, CoverageFactor, EvaluationMethod, ScalarMeasurement,
    UncertaintyError, propagate_first_order,
};
const TWO: &str = include_str!("../vectors/gum2_v1.tsv");
const THREE: &str = include_str!("../vectors/gum3_v1.tsv");

fn policy() -> CorrelationPolicy {
    CorrelationPolicy {
        max_dimension: 8,
        psd_tolerance: 1e-12,
    }
}
fn fraction(values: &[i64], pos: usize) -> f64 {
    values[pos] as f64 / values[pos + 1] as f64
}
fn check(actual: f64, expected: f64, label: &str) {
    let allowed = 5e-12 * expected.abs().max(1.0);
    assert!(
        actual.is_finite() && (actual - expected).abs() <= allowed,
        "{label}: actual={actual} expected={expected} bound={allowed}"
    );
}
fn check_reference(corpus: &str, dimension: usize, expected_rows: usize) {
    let mut rows = 0;
    for record in corpus
        .lines()
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
    {
        let nums: Vec<i64> = record
            .split_whitespace()
            .map(|v| v.parse().expect("independent exact rational fixture"))
            .collect();
        let (matrix, contributions, result) = match dimension {
            2 => {
                assert_eq!(nums.len(), 8);
                let rho = fraction(&nums, 0);
                (
                    vec![1.0, rho, rho, 1.0],
                    vec![fraction(&nums, 2), fraction(&nums, 4)],
                    fraction(&nums, 6),
                )
            }
            3 => {
                assert_eq!(nums.len(), 14);
                let r12 = fraction(&nums, 0);
                let r13 = fraction(&nums, 2);
                let r23 = fraction(&nums, 4);
                (
                    vec![1.0, r12, r13, r12, 1.0, r23, r13, r23, 1.0],
                    vec![fraction(&nums, 6), fraction(&nums, 8), fraction(&nums, 10)],
                    fraction(&nums, 12),
                )
            }
            _ => panic!("unsupported oracle dimension"),
        };
        let correlation = CorrelationMatrix::new(dimension, matrix.clone(), policy())
            .expect("reference correlation matrix must satisfy PSD policy");
        assert_eq!(correlation.dimension(), dimension);
        assert_eq!(correlation.coefficients(), matrix);
        let computed = propagate_first_order(&contributions, &correlation).unwrap();
        check(computed.variance(), result, record);
        check(computed.standard_uncertainty(), result.sqrt(), record);
        assert!(computed.variance() >= 0.0);
        rows += 1;
    }
    assert_eq!(rows, expected_rows);
}
#[test]
fn independently_generated_fraction_variance_and_sqrt_103_cases() {
    check_reference(TWO, 2, 63);
    check_reference(THREE, 3, 40);
}
#[test]
fn separately_validated_psd_rejection_and_refusal_paths() {
    let valid = CorrelationMatrix::identity(3, policy()).unwrap();
    assert_eq!(
        valid.coefficients(),
        &[1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]
    );
    let invalid = vec![1.0, -0.9, -0.9, -0.9, 1.0, -0.9, -0.9, -0.9, 1.0];
    assert!(matches!(
        CorrelationMatrix::new(3, invalid, policy()),
        Err(UncertaintyError::NotPositiveSemidefinite { .. })
    ));
    assert!(matches!(
        CorrelationMatrix::new(2, vec![1.0, 0.5, 0.3, 1.0], policy()),
        Err(UncertaintyError::NonSymmetric { .. })
    ));
    assert!(matches!(
        CorrelationMatrix::new(2, vec![1.0, 1.01, 1.01, 1.0], policy()),
        Err(UncertaintyError::InvalidCorrelationCoefficient { .. })
    ));
    assert!(matches!(
        CorrelationMatrix::new(2, vec![1.0, f64::NAN, f64::NAN, 1.0], policy()),
        Err(UncertaintyError::InvalidCorrelationCoefficient { .. })
    ));
    assert!(matches!(
        CorrelationMatrix::new(2, vec![1.0, 0.0, 0.0, 0.999999999], policy()),
        Err(UncertaintyError::InvalidUnitDiagonal { .. })
    ));
    assert!(matches!(
        CorrelationMatrix::new(
            2,
            vec![1.0, 0.0, 0.0, 1.0],
            CorrelationPolicy {
                max_dimension: 8,
                psd_tolerance: f64::NAN
            }
        ),
        Err(UncertaintyError::InvalidTolerance)
    ));
    assert!(matches!(
        CorrelationMatrix::identity(9, policy()),
        Err(UncertaintyError::ResourceLimit { limit: 8 })
    ));
    assert!(matches!(
        propagate_first_order(&[1.0, 2.0], &valid),
        Err(UncertaintyError::DimensionMismatch {
            expected: 3,
            found: 2
        })
    ));
    assert!(matches!(
        propagate_first_order(&[0.0, f64::INFINITY, 0.0], &valid),
        Err(UncertaintyError::NonFiniteValue)
    ));
    assert!(matches!(
        propagate_first_order(&[1e308, 1e308, 1e308], &valid),
        Err(UncertaintyError::NonFiniteResult)
    ));
}
#[test]
fn scalar_and_coverage_have_no_implicit_confidence_distribution() {
    let first = ScalarMeasurement::new(-7.0, 3.0, Some(EvaluationMethod::TypeB)).unwrap();
    assert_eq!(first.estimate(), -7.0);
    assert_eq!(first.standard_uncertainty(), 3.0);
    assert_eq!(first.evaluation_method(), Some(EvaluationMethod::TypeB));
    assert!(matches!(
        ScalarMeasurement::new(2.0, -0.001, None),
        Err(UncertaintyError::NegativeStandardUncertainty)
    ));
    let identity = CorrelationMatrix::identity(2, policy()).unwrap();
    let outcome = propagate_first_order(&[3.0, 4.0], &identity).unwrap();
    assert_eq!(outcome.variance(), 25.0);
    let expanded = outcome.expanded(CoverageFactor::new(2.0).unwrap()).unwrap();
    assert_eq!(expanded.value(), 10.0);
    assert_eq!(expanded.combined(), outcome);
    assert_eq!(expanded.factor().get(), 2.0);
    assert!(matches!(
        CoverageFactor::new(0.0),
        Err(UncertaintyError::InvalidCoverageFactor)
    ));
    assert!(matches!(
        CoverageFactor::new(f64::INFINITY),
        Err(UncertaintyError::InvalidCoverageFactor)
    ));
}
