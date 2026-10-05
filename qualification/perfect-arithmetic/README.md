# Perfect Arithmetic qualification configuration

Target revision: `9ec660f9abfd7fe93e441e77dbb1158c371603f5`

Required Perfect Numeric revision: `19b6747cd852a47a694a020b97ba70b6b3ef259b`

This directory is a non-production Perfect Qualification consumer for the
retained Perfect Arithmetic M6 candidate configuration.

## Direct qualification coverage

- **Q0 repository integrity:** exact Arithmetic/Numeric revisions, crate identity,
  version/MSRV, dependency inventory, private publication state, no build script,
  and production unsafe/FFI/network boundary checks.
- **Q1 build matrix:** Rust 1.99.0/MSRV, current stable, three Class-A hosts, and
  all three declared Class-B `no_std + alloc` targets. The opt-in
  `high-throughput` profile is also compile-qualified on Class-B fallback paths.
- **Q2 semantic integration:** an independent consumer uses only public
  `Natural`/`Integer` APIs and checks exact arithmetic, structured domain
  behavior, conversions, sign/magnitude, shifts, GCD, and quotient/remainder
  reconstruction.
- **Q3 reproducibility:** an independent retained semantic fingerprint is
  executed repeatedly under default, all-feature, and no-default-feature
  configurations on Linux x86_64, Linux arm64, and Windows.
- **Q4 robustness:** exhaustive small-domain consumer checks and boundary cases
  complement the retained Arithmetic M5 evidence at
  `3678a29e4b464864797881dca65408d9ac84210f`, which passed 50,000 ASan fuzz
  iterations and production mutation testing with zero missed viable mutants.
- **Q5 external reference:** independent Python arbitrary-precision integers
  regenerate and verify qualification-owned Natural, Integer, and GCD vectors
  that are also consumed through the public Rust API.

## Retained downstream-family evidence

Perfect Rational and Perfect Float are production downstream consumers of
Perfect Arithmetic. Arithmetic M4 retained their exact dependency revisions and
green seven-job hosted matrices. This qualification bundle adds an independent
non-production consumer; it does not replace those real downstream integrations.

## Q6 status

Q6 is intentionally separate from this Q0-Q5 bundle. Perfect Arithmetic still
has prerelease/release gates that must be handled explicitly, including the
bottom-up registry sequence required before its final `cargo package` and
packaged-offline-build evidence can exist. Q0-Q5 success must not be inflated
into Q6 or public-release readiness.
