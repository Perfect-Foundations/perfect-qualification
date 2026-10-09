# Perfect Arithmetic qualification configuration

Target revision: `0e61fb6854a6c9b5052d4c1deca71b606f1ba7b6`

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

## Canonical Natural/Integer Hash extension — 2026-10-08

The exact Arithmetic M6 candidate `0e61fb6854a6c9b5052d4c1deca71b606f1ba7b6` adds qualification of the
versioned public `Hash` write-stream contract, without reading internal
big-integer representation or production implementation details.

`scripts/verify_hash_oracle.py` independently regenerates 25 fixed expected
streams using Python's arbitrary-precision integer operations. They are stored
in `vectors/canonical_hash.tsv`; `tests/hash_contract.rs` captures the output
of public `Natural::hash` and `Integer::hash` and compares each byte. Cases
include canonical zero, positive/negative values, limb boundaries, and 1024/2048-bit
magnitudes. The existing 72 Numeric/Integer/GCD independent vectors and the
previous semantic fingerprint remain separate regression oracles.

Exact-snapshot **local Ubuntu WSL2 x86-64, Rust 1.99.0** evidence is in
`verification/LOCAL-ZEN-ARITHMETIC-HASH-20261008.json`: 25 Python cases PASS;
8/8 independent consumer tests in both feature modes, Clippy `-D warnings`
in both modes, rustdoc, formatting, and opt-in `high-throughput` Hash contract
PASS. The source and dependency graph used for this result are recorded exactly.

Hosted three-Class-A Q0–Q5 matrix and real native ARM64 remain **NOT RUN/BLOCKED**
at this revision. These local checks neither qualify Q6 nor establish published
registry dependencies. The verification consumer is not a production dependency.
