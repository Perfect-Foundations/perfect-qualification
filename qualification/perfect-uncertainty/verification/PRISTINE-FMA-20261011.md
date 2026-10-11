# Independent Uncertainty conditioned-matrix pristine replay — 2026-10-11

## Source exactness

- Production: `Perfect-Foundations/perfect-uncertainty@66af44422c38f5c86d4e4b3c44f8ae2a37c1a59c` (draft PR #2).
- First pushed Qualification candidate: `Perfect-Foundations/perfect-qualification@f68baace1404bdfb53fc9d462e30e8cb424d2614` (draft PR #13).
- New detached clean working tree was created at exactly `f68baace1404bdfb53fc9d462e30e8cb424d2614` and had a clean Git status.
- Checked-in consumer Cargo.lock resolves one exact Uncertainty Git revision and `libm=0.2.15` with package checksum; no local path override.
- Clean-checkout Cargo.lock SHA-256: `b058d145f1913faae3a23e9ec4baf0042868786485e169c7a85b11daa845bf7a`.

## Independently verified corpus

| Frozen independent Python oracle | Case count | SHA-256 |
|---|---:|---|
| `gum2_v1.tsv` | 63 | `c6e23aaeaa5a6d20ac53a47ce32201d8f2544921b429edb07589593b312dd5c4` |
| `gum3_v1.tsv` | 40 | `dae756ec8b8578ff89300cd6246b87a4d3c8702ea190201b19db33aee8e8668a` |
| `adversarial_gum_v1.tsv` | 160 | `23e5c039b6728ade8cd56d42a5e5b3ebfd023821f2fe860e76fddcedc8fed8e4` |
| `conditioning_v1.tsv` | 234 | `b59e7aab50b1f89ab01661e01c119e50bb19e9fc9d511d3c2de208b036ffaa86` |

Total: **497** independently computed cases. The new 234 exact binary64-input cases were obtained using Python `Fraction.from_float` and a diagonal-plus-pairwise formula rather than the production FMA and matrix-vector method. Input/expected f64 bit patterns are frozen. New consumer threshold `relative error <= 1e-10` for strictly positive reference variances; this is limited regression evidence, not a universal correctly-rounded contract.

## Observed clean-checkout runs

In **each** of native Windows x86-64 and WSL2 Linux x86-64 (same physical machine), `cargo test --locked --offline` and `cargo clippy --locked --offline --all-targets ... -- -D warnings` passed for both `--no-default-features` and `--all-features`.

Seven independent Rust integration tests per host × feature configuration = **28/28** passing test executions. All four independent Python scripts, vector-byte SHA checks and exact lockfile/Git SHA graph verifier passed on both hosts from the detached Qualification source checkout. No expected result regenerated from production outputs.

Production source regression suite passed independently on Windows and WSL2 in both feature configurations (including the initial deliberately failing pre-fix case), and three configured no_std compile-only targets passed. Broader matrices, performance effects of software FMA, ARM64 runtime, official full GUM conformance, formal M1 closure and release gates are not claimed.
