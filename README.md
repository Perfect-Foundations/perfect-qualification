# Perfect Qualification

`perfect-qualification` is the cross-family verification and compatibility repository for Perfect Foundations.

It does **not** own production algorithms. Individual crates own their implementations and crate-specific tests. This repository verifies that the family works together without turning the family into a monolith.

## Qualification scope

- Cross-crate API and version compatibility.
- Dependency-DAG and layering checks.
- MSRV verification.
- `no_std` and feature-matrix verification where applicable.
- Multi-target and cross-platform builds.
- Determinism and reproducibility checks.
- Canonical serialization and round-trip verification.
- Cross-crate numeric conversion and precision tests.
- Independent reference vectors and oracle comparisons.
- Integration fuzzing and mutation testing where valuable.
- Release-candidate whole-family qualification.
- Detection of unintended mandatory foreign-language/runtime dependencies.

## Rule

A passing crate test suite proves that crate's own contract. A passing Perfect Qualification run proves that supported combinations of family crates continue to satisfy the shared family contract.
