//! Actual published output values, distinct from independently generated
//! Fraction oracles. Sources and page/section provenance are in
//! verification/PUBLISHED-METROLOGY-20261011.md.
use perfect_uncertainty::{
    CorrelationMatrix, CorrelationPolicy, CoverageFactor, ScalarMeasurement, propagate_first_order,
};
fn close(actual: f64, expected: f64, tol: f64) {
    assert!(
        (actual - expected).abs() <= tol,
        "actual {actual:.16e}, expected {expected:.16e}, tol={tol}"
    );
}
#[test]
fn jcgm_100_2008_section_5_2_2_ten_perfectly_correlated_resistors() {
    // Published example: 10 nominally 1,000-ohm resistors each calibrated
    // against the same reference (standard uncertainty 100 milliohm).
    // Their combined 10,000-ohm reference's u_c is 1 ohm.
    let policy = CorrelationPolicy {
        max_dimension: 16,
        psd_tolerance: 1e-12,
    };
    let all_correlated = CorrelationMatrix::new(10, vec![1.0; 100], policy).unwrap();
    let g = [0.1_f64; 10]; // 100 milliohm = 0.1 ohm, sensitivity +1 each
    let correlated = propagate_first_order(&g, &all_correlated).unwrap();
    close(correlated.variance(), 1.0, 1e-14);
    close(correlated.standard_uncertainty(), 1.0, 1e-14);
    let uncorrelated = CorrelationMatrix::identity(10, policy).unwrap();
    let incorrectly_independent = propagate_first_order(&g, &uncorrelated).unwrap();
    // The source explicitly contrasts 1 ohm with approximately 0.32 ohm
    // obtained by incorrectly neglecting common-reference correlation.
    close(
        incorrectly_independent.standard_uncertainty(),
        0.316227766016838,
        1e-14,
    );
    assert!((incorrectly_independent.standard_uncertainty() - 1.0).abs() > 0.6);
}
#[test]
fn nist_tn1297_section_7_3_published_mass_uncertainty_reporting() {
    // NIST example gives m_s = 100.02147 g; u_c = 0.35 mg; k=2
    // gives U=0.00070 g; k=2.26 gives U=0.00079 g after reporting rounding.
    // The actual measurement-input decomposition is NOT published here.
    // A single equivalent component is used solely to test expanded-u arithmetic.
    let mass = ScalarMeasurement::new(100_021.47, 0.35, None).unwrap(); // mg throughout
    close(mass.estimate(), 100_021.47, 1e-10);
    let identity = CorrelationMatrix::identity(
        1,
        CorrelationPolicy {
            max_dimension: 1,
            psd_tolerance: 1e-12,
        },
    )
    .unwrap();
    let combined = propagate_first_order(&[mass.standard_uncertainty()], &identity).unwrap();
    close(combined.standard_uncertainty(), 0.35, 1e-14); // mg
    let u2 = combined
        .expanded(CoverageFactor::new(2.0).unwrap())
        .unwrap();
    close(u2.value() * 0.001, 0.00070, 1e-15); // reported in g
    let u226 = combined
        .expanded(CoverageFactor::new(2.26).unwrap())
        .unwrap();
    close(u226.value() * 0.001, 0.000791, 1e-15); // unrounded g
    close((u226.value() * 100.0).round() / 100.0, 0.79, 1e-14); // published mg to 0.01
}
