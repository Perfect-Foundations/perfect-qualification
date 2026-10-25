# Independent Gram-covariance Qualification expansion — October 11, 2026

## Exact source and domain

Source candidate remains **`Perfect-Foundations/perfect-uncertainty@1450a418f5473b3e173a0cc79c65f0ce55d29427`**. No production algorithm, API, dependency, `project-status.toml` gate, or protected Perfectπ source was changed by this qualification-only expansion. The independent consumer uses the exact Git-pinned Cargo.lock without local path overrides or substituted registry packages.

This expansion addresses the gap left by the earlier equicorrelation-only conditioned numerical tests: general **non-equicorrelated valid PSD correlations** and negative PSD source rejection, including high dimensions and permutations.

## Mathematical construction, independent of Rust

`scripts/gram_reference.py` uses seeded Python standard-library integer signs and `fractions.Fraction.from_float` to derive an *exact mathematical oracle* for the binary64 input values. For each sample, rows of an integer sign matrix `S` have sixteen entries in `{-1,+1}`. Consequently,

`G = S S^T / 16`

is exactly positive semidefinite and has diagonal 1. In the near-perturbed construction,

`R = (1 - 2^-40) G + 2^-40 I`,

which is also exactly PSD. `G` and `R` contain dyadic entries representable in binary64; the positive-semidefinite proof is independent of the production validator. Families include identical rows (rank one), duplicated rows (rank deficient), seeded non-equicorrelated rows, and near-perturbed Gram matrices.

For the exact binary64 contributions, the Python oracle computes

`variance = sum(g_i*g_i) + 2*sum(R_ij*g_i*g_j, i<j)`

using exact `Fraction` arithmetic and rounds only the **final** expected result to one finite positive binary64 value. Each matrix coefficient, contribution, and expected value is frozen in exact hexadecimal binary64 bit-pattern form. Generating with `--generate` refuses overwriting a corpus already present.

- New corpus: `vectors/gram_covariance_v1.tsv`
- New cases: **1,601** (3D 320; 4D 248; 6D 274; 8D 253; 12D 252; 16D 254)
- Corpus SHA256: `d10b42da25b5746390c1b1cd47677a6b5704d7e9f03eeaab483898b933560aaf`
- New public Rust consumer: `tests/gram_psd.rs`, testing each case with original ordering **and reversed permutation**, 3,202 propagation runs
- Relative variance acceptance tolerance: `<= 1e-10` for strictly positive expected variance. This bound applies only to these cases; no universal high-dimensional accuracy or correctly-rounded guarantee is claimed.
- New `tests/gram_reject.rs`: for dimensions 3, 4, 6, 8, 12 and 16, a known-indefinite 3x3 principal block with `R12=R13=1` and `R23=1-2^-32` has exact witness `(-2,1,1)` with quadratic form `-2^-31`. All 49 cyclic embeddings must be rejected under caller PSD tolerance `1e-12`. Six singular all-ones PSD control matrices must be accepted.

Earlier frozen mathematical references remain untouched (103 original + 160 adversarial + 234 near-singular + 900 conditioned = 1,397); the new total is **2,998 independently computed reference rows**. Permutation runs and invalid-matrix checks are additional test executions, *not* additional mathematical reference rows.

## Executed source-exact testing

Native Windows x86-64 and WSL2 Linux x86-64, same physical machine, Rust 1.99: both `--no-default-features` and `--all-features` pass `cargo test --locked --offline` and warning-denied `cargo clippy --locked --offline --all-targets ... -- -D warnings`. The expanded Qualification suite has **12 Rust integration-test functions per host-feature configuration, 48/48 passing executions**. Five independent Python scripts regenerate all six mathematical corpora (two corpora belong to the same base script); six frozen SHA-256 checks and exact source-lock graph verification pass on both hosts.

Previous production source regression, no_std target and preliminary FMA benchmarks remain revision-bound historical evidence and were not rerun here. This is not M1/Q6 promotion, a general real-matrix error guarantee, full GUM conformance, physical ARM64 runtime or production release evidence.
