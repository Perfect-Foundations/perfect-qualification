# Independent Perfect Polynomial M1 qualification consumer

**Active conservative candidate source:** `Perfect-Foundations/perfect-polynomial`, exact Git revision **`34ffc473bf07960b66fdc8b0637bb0bf4aa8670e`** (draft PR #14). Composed from frozen M1 production PR #8 `d2365a0fc5af81877e64598301064ea2910e67f9`, metadata-only PR #12 and cfg(test)-only PR #13. **Production mathematical code is identical to frozen PR #8**; this source transition is expressly documented, not a path substitution.

Direct Git pins: Arithmetic `1a54d3c7cbbae4e73325cc70fd2777a2427b1504`, Rational `35a8e9cc629ee578fe7b624e2134929ce7eeff8a`, transitive Numeric `19b6747cd852a47a694a020b97ba70b6b3ef259b`. Retained Cargo.lock locks the exact Git SHA and all resolved registry packages. **This does not constitute a release or formal M1 signoff.**

Independent SymPy **79 adversarial + 500 seeded cases** check canonical Z[x] and Q[x] GCD, symmetry, exact integer divisibility, addition, multiplication and derivative against external expected coefficients. `tests/seeded_integer_oracle.rs` adds **1,536 bounded i128 arithmetic/evaluation cases + 192 factor/GCD invariant cases**. Public contract tests cover canonicalization, zero behavior, typed errors, Rational Euclidean quotient and GCD monic normalization, Integer exact quotient and 4095-bit inputs. The source-exact 579-case semantic fingerprint was **rerun on combined PR #14**: Windows and Ubuntu WSL2 both produced `435349aa9a62498a`, matching the frozen PR #8 fingerprint.

Windows 11 x86-64 and Ubuntu WSL2 Linux x86-64 Rust 1.99.0 all-features/no-default tests PASS on the **combined source**; warning-denied Clippy and Rustdoc PASS. AArch64 Linux/wasm32v1/Thumb v7em/RISC-V no-default cross-builds are compile-only, never native ARM64 execution. WSL2 Cargo Git objects for the combined revision were supplied by a verified exact Git bundle because the remote credential flow was unavailable there; GitHub source URL and SHA were never replaced with local paths.

The **real 10,000-execution bounded libFuzzer campaign** used frozen PR #8, recorded in test-only `fuzz/Cargo.toml` and `fuzz-evidence/` and not rerun because the production source is identical. Focused private 512-bit LCM mutation campaign on test-only PR #13: **7 caught, 0 missed, 4 unviable**, results in `mutation-evidence/lcm-targeted-20261009`.

Latest combined-SHA pathological resource matrix: `examples/resource_pathologies.rs` and `resource-evidence/m1-pathologies-composed/` plus `M1-COMPOSED-CANDIDATE-REVIEW-20261009.md`. These are exploratory operation-only heap requested-bytes and latency, not formal stack/physical heap bounds or an OOM recovery guarantee. Source review identifies expected dense/sparse memory tradeoff and acknowledges potential untrusted-input resource exhaustion. No actual new production correctness defect has been shown.

**Reproduction:**
```sh
cargo test --offline --locked --all-features
cargo test --offline --locked --no-default-features
cargo clippy --offline --locked --all-targets --all-features -- -D warnings
cargo doc --offline --locked --no-deps --all-features
cargo run --offline --locked --release --example resource_pathologies
python scripts/generate-robustness-v1.py
```

No owner license selection, publish=true, native ARM64 run, registry package success, full independent M1 signoff or Q0–Q6 qualification is claimed. Original Package exit-101 registry dependency logs remain in `package-evidence/`; do not rerun the blocked packaging step without changed prerequisites. No Perfectπ modifications.
