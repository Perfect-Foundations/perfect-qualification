use perfect_uncertainty::{CorrelationMatrix, CorrelationPolicy, propagate_first_order};

const CORRELATIONS: [[[f64; 3]; 3]; 4] = [
    [[1.0, 1.0, 1.0], [1.0, 1.0, 1.0], [1.0, 1.0, 1.0]],
    [[1.0, 1.0, 0.0], [1.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
    [[1.0, 0.0, 1.0], [0.0, 1.0, 0.0], [1.0, 0.0, 1.0]],
    [[1.0, -1.0, 1.0], [-1.0, 1.0, -1.0], [1.0, -1.0, 1.0]],
];

#[test]
fn independent_fraction_adversarial_160_source_exact() {
    let matrices: Vec<_> = CORRELATIONS
        .iter()
        .map(|rows| {
            CorrelationMatrix::new(
                3,
                rows.iter().flat_map(|row| row.iter().copied()).collect(),
                CorrelationPolicy {
                    max_dimension: 3,
                    psd_tolerance: 0.0,
                },
            )
            .unwrap()
        })
        .collect();
    let mut count = 0;
    for row in include_str!("../vectors/adversarial_gum_v1.tsv").lines() {
        if row.starts_with('#') || row.is_empty() {
            continue;
        }
        let fields: Vec<i64> = row
            .split_ascii_whitespace()
            .map(|v| v.parse().expect("exact frozen integer Fraction oracle"))
            .collect();
        let [mode, a, b, c, expected]: [i64; 5] = fields.try_into().unwrap();
        let contributions = [a as f64, b as f64, c as f64];
        let actual = propagate_first_order(&contributions, &matrices[mode as usize]).unwrap();
        assert_eq!(actual.variance(), expected as f64, "{row}");
        assert_eq!(
            actual.standard_uncertainty(),
            (expected as f64).sqrt(),
            "{row}"
        );
        assert!(!actual.clamped_roundoff(), "{row}");
        count += 1;
    }
    assert_eq!(count, 160);
}

#[test]
fn independently_derived_nist_tn1297_a3_linear_2_input_controls() {
    // Applying published NIST TN1297 Appendix A equation A-3 to
    // sensitivities (2,-3) and standard uncertainties (1/2,1/4)
    // yields contributions (1,-3/4). These numbers are constructed
    // for this test; they are not a claimed published NIST data table.
    for (rho, exact_variance) in [(0.0, 25.0 / 16.0), (0.5, 13.0 / 16.0)] {
        let matrix = CorrelationMatrix::new(
            2,
            vec![1.0, rho, rho, 1.0],
            CorrelationPolicy {
                max_dimension: 2,
                psd_tolerance: 0.0,
            },
        )
        .unwrap();
        assert_eq!(
            propagate_first_order(&[1.0, -0.75], &matrix)
                .unwrap()
                .variance(),
            exact_variance,
        );
    }
}
