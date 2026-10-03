# Perfect Numeric qualification configuration

Target revision: `ef85b646fc299f5c0b275c1bc29062e047771932`

This directory is a non-production Perfect Qualification consumer for the
retained Perfect Numeric M6 candidate configuration.

## Direct qualification coverage

- **Q0 repository integrity:** exact pinned revision, crate identity/version/MSRV,
  zero dependency and no-build-script boundary.
- **Q1 build matrix:** Rust 1.99.0/MSRV, current stable, Class-A hosted targets,
  and the three declared Class-B `no_std` compile targets.
- **Q2 semantic integration:** downstream use of rounding, status/loss,
  conversion, range-direction, and structured-error vocabulary without access
  to private implementation state.
- **Q3 reproducibility:** retained qualification fingerprint executed twice on
  each Class-A target and compared across Linux x86_64, Linux arm64, and Windows.
- **Q4 robustness:** exhaustive finite-state/status algebra and complete
  16-bit-to-8-bit consumer conversion domains. Numeric M5 mutation/pathological
  evidence is retained upstream at revision `4f6a981012e022264526e5ea88767821cf255e98`.
- **Q5 external reference:** independent Python Decimal and integer-domain oracle
  verifies retained qualification vectors.

## Q2 retained real-consumer evidence

Perfect Arithmetic is the serious family consumer. Its M1 integration revision
`25fed81cfe9397db6881278b13b4958a0816eed0` passed all seven hosted jobs in
Arithmetic CI run 37152265151 and uses Perfect Numeric `Conversion<T>`,
`ConversionError`, and `RangeDirection` through checked arbitrary-precision
integer conversions.

## Q6 status

Q6 Release Candidate is **not yet claimed complete** by this initial bundle.
Numeric still has prerelease gates outside direct Q0-Q5 execution, including its
license decision, final dependency/license/security review, release
documentation, package presentation, and release-process records. The
qualification report must record those explicitly before Q6 can pass.
