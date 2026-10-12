# Independent Uncertainty qualification evidence — 2026-10-11

Source: `Perfect-Foundations/perfect-uncertainty` Git SHA
`82ebfc51ca3754507ddd36380615bf006ea6d08a`.
Production dependency exclusively on this pinned exact Git revision;
only additional transitive dependency `libm=0.2.15`. The independent
consumer retains a lockfile and disallows an implicit path override.
Independent Python standard-library Fraction reference computes the
quadratic form via diagonal sum plus 2×off-diagonal pairs, unlike
production row-major compensated binary64 accumulation.

Frozen corpora (ASCII LF exact SHA-256):
- `gum2_v1.tsv`: 63 pairs; `c6e23aaeaa5a6d20ac53a47ce32201d8f2544921b429edb07589593b312dd5c4`.
- `gum3_v1.tsv`: 40 triples; `dae756ec8b8578ff89300cd6246b87a4d3c8702ea190201b19db33aee8e8668a`.

103 exact mathematical reference vectors; Rust first-order variance and
square-root uncertainty checked within absolute/relative
`5e-12 * max(1, expected)`, with caller-selected PSD tolerance `1e-12`.
Additional source-exact consumer tests exercise singular/perfect correlation,
negative PSD, symmetry/diagonal/finite violations, source size limits,
nonfinite propagation, scalar standard uncertainty and explicit expansion
factor without implied probability.

Observed Windows native x86-64 Rust 1.99 and WSL2 Linux x86-64:
both `--no-default-features` and `--all-features` used
`cargo test --locked --offline` and
`cargo clippy --locked --offline --all-targets -- -D warnings`.
3 integration tests per feature/host configuration passed, 12 integration
test executions total. Python independently regenerated corpus hashes on both
hosts and source/lock verifier passed. No ARM64 hardware, formal acceptance,
published GUM profile, or release claim. Windows and WSL2 are distinct hosts
but the same physical x86-64 hardware.

## Numerical-stability source follow-up — same date

Uncertainty source pinned to
`82ebfc51ca3754507ddd36380615bf006ea6d08a`.
The source fixes a demonstrated catastrophic cancellation defect in the
first-order GUM kernel by using Neumaier-compensated evaluation of
`g^T (R g)`. For the all-ones rank-one PSD correlation matrix and
`[1e16,-1e16,1]`, the exact variance is 1; earlier multiplication-first
accumulation could return a grossly incorrect positive or negative value.

The **original immutable 103 exact Fraction vectors remain unchanged**.
Independent Python `scripts/adversarial_reference.py` separately computes
160 more exact arithmetic cases from diagonal and pairwise terms over
four structurally different PSD families and mixed magnitudes through
`10^18`. Its retained corpus
`vectors/adversarial_gum_v1.tsv` has SHA-256
`23e5c039b6728ade8cd56d42a5e5b3ebfd023821f2fe860e76fddcedc8fed8e4`.
Combined mathematical reference scope is **263 exact vectors**.

The separate public consumer additionally checks an exact application of
[NIST TN 1297 Appendix A, Eq. (A-3)](https://www.nist.gov/pml/nist-technical-note-1297/nist-tn-1297-appendix-law-propagation-uncertainty):
a test-derived example, not a reproduction of NIST's published numerical data.

Native Windows x86-64 and WSL2 Linux x86-64 both passed
`cargo test --locked --offline` and
`cargo clippy --locked --offline --all-targets -- -D warnings`
under `--no-default-features` and `--all-features`:
five integration tests per host-feature configuration, **20 passing test
executions**, with no failures. The 263-case independent Python reference,
all three frozen vector hashes, and exact pinned Cargo.lock graph verifier
passed on each host.

The WSL2 cache initially lacked the new Git commit, so an exact copy of
the same production Git object was fetched from the checked-out Git
repository into its private Cargo Git cache. There were no path overrides,
dependency substitutions, altered lockfile identities or source modifications.

This source-exact candidate is **not** accepted as formal M1 qualification.
Broader conditioned-matrix studies, independently reproduced published GUM
numerical datasets, physical ARM64 runtime, hosted CI and API/security review
remain pending. Perfectπ remained untouched.

## Current production pin and additional overflow refusal regression

**Current pinned production source:**
`1ca644f946cb9adea9f817675a464a5800713b25`.
This follow-up changes neither the 103 original nor the 160 additional
exact mathematical reference cases. Total independent Fraction cases: **263**.

A new independent public consumer verifies that the all-ones rank-one PSD
matrix with finite `g=[1e308,-1e308,1]` yields exact variance 1,
not `NonFiniteResult`. Previously the auxiliary absolute-magnitude
roundoff diagnostic overflowed despite a finite correct variance.
The production fix conservatively saturates that diagnostic without
silently changing the quadratic form itself.

Rust 1.99 Windows native x86-64 and WSL2 Linux x86-64 each passed
`cargo test --locked --offline` and warning-denied all-target Clippy
under both `--no-default-features` and `--all-features`.
Six integration tests per configuration × 4 configurations =
**24 passing independent integration test executions**; no failures.
Independent Python reference reproduction (263 cases) and the
exact-Git-source Cargo.lock/vector-hash verifier passed on both hosts.
WSL2's private Cargo Git cache imported the same exact local Git object;
no source path override, dependency redefinition or lockfile substitution.

This is not formal M1/Q6 acceptance, native ARM64 validation, hosted CI
or release authorization.


## Near-singular product-roundoff follow-up — October 11, 2026

**Current validated pinned source:** `66af44422c38f5c86d4e4b3c44f8ae2a37c1a59c`.
The preceding source-exact reference corpus remains unchanged (103 GUM
Fraction + 160 adversarial = 263 cases). A fourth frozen independent oracle,
`scripts/conditioning_reference.py`, calculates the exact rational quadratic
variance of *the actual binary64 inputs* using Python
`fractions.Fraction.from_float` and a diagonal-plus-paired-off-diagonal
formula, not the Rust matrix-vector product or fused residual algorithm.

`vectors/conditioning_v1.tsv`: **234** additional cases, source-independent
reference SHA-256:
`b59e7aab50b1f89ab01661e01c119e50bb19e9fc9d511d3c2de208b036ffaa86`.
Every input and expected result is retained by its exact binary64 bit pattern.
The cases cover 2x2, 3x3 and 4x4 equicorrelation PSD families, correlations
adjacent to ±1, and mixed scale/cancellation cases from `1e12` through
`1e150`. The reference generator refuses existing-file overwrites.
The public Rust consumer checks finite variances without clamping, within
a declared `1e-10` *relative* error budget on strictly positive expected
variances. This is a targeted numerical regression threshold, not a blanket
mathematical accuracy guarantee.

The full independent corpus now contains **497 mathematical vectors**
(63 + 40 + 160 + 234). On Windows native x86-64 and WSL2 Linux x86-64,
both `--no-default-features` and `--all-features` passed offline locked
tests and all-target warning-denied Clippy against this exact Git source.
Seven independent Rust tests per host/feature configuration were executed,
**28/28 total**, and all four independent reference generators and the
source-lock-vector verifier passed on both hosts.
Formal M1 acceptance, ARM64 physical runtime, broader all-matrix accuracy
bounds, and optimized performance benchmarking remain unearned.
