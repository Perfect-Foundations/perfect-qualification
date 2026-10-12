# Exact-head Wave 4 Uncertainty Qualification replay — 2026-10-11

**Production** `Perfect-Foundations/perfect-uncertainty@dafe6a28a0dd69042e2b5fcc36ea254f8e16ac60` (draft PR #2; unmerged).
**Qualification tested HEAD** `Perfect-Foundations/perfect-qualification@779757324aad30a5cb1a47e791d7e41a0ba7929f` (draft PR #13, unmerged). A detached worktree was recreated from precisely that commit with a clean Git status, no path substitutions. Root verifier checked exact Cargo.lock SHA256 `25669a6482b01acad5507d1f5452e31b5f63070081ef9e7da4708b436496d587` and **seven immutable mathematical corpus** SHA256 values.

**Independent mathematical references:** 3,088 retained exact-reference rows (prior 2,998 plus 90 new exact dyadic 18D/24D/32D sign-Gram cases, SHA256 `4b49ae1139b51ed0bf73701bce6b2a69f92d07dde2cf3bb900dd6500904c43b3`). Source-free Python exact Fraction code recomputed and byte-checked all seven corpora from the pristine checkout on native Windows x86-64 and WSL2 Linux x86-64. New 90 cases were also checked with reversed simultaneous sensitivity/row/column permutation (180 validations total, not extra oracle rows). Previously retained arbitrary-matrix, invalid PSD and ill-conditioned cases remain in the source-exact suite.

**Published examples, separately counted:** Two executable official-reference tests passed: JCGM 100:2008 corrected 2010, §5.2.2, ten common-reference resistors (published `u_c=1 ohm`; uncorrelated comparison approximately `0.32 ohm`) and NIST TN 1297 1994, §7.3, mass uncertainty 0.35 mg with published k=2 and k=2.26 reporting. Exact official URLs, derivation limits and rounding documented in `PUBLISHED-METROLOGY-20261011.md`. Neither is an independently generated reference corpus.

**Exact pristine replay, Rust 1.99:** in each of Windows native x86-64 and Ubuntu WSL2 Linux x86-64, `cargo test --locked --offline` and `cargo clippy --locked --offline --all-targets -- -D warnings` passed in both `--no-default-features` and `--all-features`. 15 independent Rust integration test functions ×2 feature modes ×2 operating systems = **60/60 passing test executions**. The complete six independent Python reference scripts and root SHA verifier passed on both. Same physical CPU; no native ARM64 hardware or hosted CI evidence.

**Benchmarks reproduced from this pristine checkout** using the tracked exact revision dual-crate harness `benchmarks/psd-pivot-comparison`, `cargo run --release --locked --offline`, 1,000 calls per input and median five repetitions after one warmup. Corrected/baseline ratios in the fresh checkout:

| Case | Windows old ns | Windows pivoted ns | WSL2 old ns | WSL2 pivoted ns |
|---|---:|---:|---:|---:|
| 4D independent | 88.70 | 92.50 | 54.59 | 60.86 |
| 8D half correlation | 226.00 | 241.60 | 171.33 | 191.79 |
| 16D rank-one | 441.40 | 511.80 | 502.00 | 492.63 |
| 18D exact PSD sign-Gram | 1352.30 | 1798.10 | 1371.59 | 1577.44 |

The old source **incorrectly rejects** the last mathematically valid input; the pivoted version accepts. Their last-line timings are not a comparison of equivalent outputs. Preliminary same-hardware results vary by host/trial and are not general throughput guarantees. All benchmark sources, Git pins and lockfile are retained.

No formal M1/Q6 acceptance, general PSD classification guarantee, released crate, physical ARM64 execution or protected merge. Perfectπ unchanged.
