# Wave 24 — Polynomial independent consumer qualification
Date: 2026-10-13. Evidence only; no formal M1/M2 acceptance or release.

## Source-exact graph
- Production: Perfect Polynomial PR #18, `7774e361c8a86c1ad3f5358d8df7a353371713d9`.
- Independent source-consumer: `qualification/poly-rational-source-20261011`.
- The consumer's manifest, Cargo.lock, source-exact.json and verify_graph.py are pinned to the exact new commit. Existing Arithmetic, Rational and Numeric source identities are unchanged.
- `python qualification/poly-rational-source-20261011/verify_graph.py`: PASS; frozen independent mathematical fixtures and previous source graph preserved.
- Immutable release work has not been modified; source is unmerged and draft.

## Work done
- Higher-degree direct-coefficient schoolbook Integer oracle using 513-bit signed coefficients, sparse and dense degree/dispatch boundaries at 63/64/95/96/97/127/128/256. Product and canonical derivative compared with independent coefficient arrays.
- Signed monic/nonmonic Integer exact-division and refusal boundary tests.
- Independent same-degree Rational quotient/remainder coefficient oracle for near-opposite operands; lower-degree case checked by Euclidean identity instead of invalidly assuming a constant quotient.
- All committed production APIs and algorithms remain unchanged. New permanent tests add assurance, not a claimed production speedup.
- The 128-record canonical Wave 24 test covers high-degree products, derivatives, Rational division, output serialization, source-exact external consumer execution. The full canonical cross-platform verifier is in Qualification #12.
- The old Wave 23 source-exact and 48-record canonical reference baseline is preserved separately and not counted again as new.

## Qualification matrix
Rust 1.99, locked and offline, full consumer `--all-targets`:
| Execution | no-default features | all-features |
|---|---|---|
| Windows native x86-64 | PASS | PASS |
| Ubuntu WSL2 x86-64 | PASS | PASS |
| Local AArch64/QEMU Linux | PASS | PASS |
Windows `cargo clippy --all-targets -- -D warnings` passed in both feature modes; rustfmt --check passed after local formatting.
All 128 new canonical output records matched Windows/WSL2/QEMU with SHA-256 `7120fd33313cee79447d11fbac5c49e84c06cb6f921c78670308adcf1c5a9ff2` (group identifier POLY, ASCII W24 lines, LF, final LF); generated hashes were checked by the cross-family Wave 24 verifier in Qualification #12.

## Mutation and remaining limits
An initial isolated 9-mutant campaign covering the `opposite_rational_polynomials` shortcut with the older `cross_optimized_workflows` target returned 4 caught and 5 missed; the missed conditions are coverage gaps, not verified production defects. A separate Wave 24 near-opposite test now exercises quotient and remainder invariants. Mutation retest status is recorded separately once independently established. Full field-wide mutation/fuzz review, formal acceptance, physical ARM64 and release gates remain OPEN.

## Reproducible commands
```
python qualification/poly-rational-source-20261011/verify_graph.py
cargo +1.99.0 test --manifest-path qualification/poly-rational-source-20261011/Cargo.toml --locked --offline --all-targets --no-default-features
cargo +1.99.0 test --manifest-path qualification/poly-rational-source-20261011/Cargo.toml --locked --offline --all-targets --all-features
```
The same exact Cargo commands were run under Ubuntu WSL2, and under `--target aarch64-unknown-linux-gnu` with local QEMU and cross-linker environment; none of those executions proves physical ARM64.
