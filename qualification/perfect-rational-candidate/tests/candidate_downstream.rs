//! Candidate-only full-family independent integration, reusing Qualification tests.
//! All production dependency revisions are source-exact.
#[path = "../../perfect-four-crate-integration/tests/phase1_arithmetic_division_oracle.rs"]
mod arithmetic_division;
#[path = "../../perfect-four-crate-integration/tests/phase1_exact_graph.rs"]
mod cross_family;
#[path = "../../perfect-four-crate-integration/tests/phase1_primitive_differential.rs"]
mod decimal_primitive;
#[path = "../../perfect-four-crate-integration/tests/phase1_quantize_oracle.rs"]
mod decimal_quantization;
#[path = "../../perfect-four-crate-integration/tests/phase1_float_rounding_oracle.rs"]
mod float_rounding;
