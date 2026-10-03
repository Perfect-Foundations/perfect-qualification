# Perfect Qualification

`perfect-qualification` is the **cross-family verification, compatibility, and release-assurance repository** for Perfect Foundations.

It does not own production algorithms. Each family crate owns its own implementation and crate-specific test suite. Perfect Qualification verifies that supported combinations of family crates continue to satisfy shared expectations.

## Qualification domains

- cross-crate API/version compatibility;
- dependency-DAG and layering checks;
- MSRV verification;
- `no_std` and feature-matrix verification where applicable;
- multi-target and cross-platform builds;
- determinism and reproducibility checks;
- canonical serialization and round-trip verification;
- cross-crate numeric conversion and precision tests;
- independent reference vectors and oracle comparisons;
- integration fuzzing and mutation testing where useful;
- detection of unintended mandatory foreign-language/runtime dependencies;
- release-candidate whole-family qualification.

## Evidence principle

A passing crate test suite demonstrates the crate's own tested contract.

A passing Perfect Qualification run demonstrates the tested **integration contract** across supported combinations.

## Planned structure

- `qualification/` — qualification plans and matrices
- `vectors/` — cross-crate reference vectors
- `fixtures/` — integration fixtures
- `scripts/` — deterministic qualification runners
- `reports/` — generated or retained qualification evidence as appropriate

No production crate should depend on `perfect-qualification`.
