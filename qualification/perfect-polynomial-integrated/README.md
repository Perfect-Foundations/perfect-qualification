# Independent M1 Polynomial integration consumer

**Conservative baseline (not released or fully qualified):** `Perfect-Foundations/perfect-polynomial` exactly `d2365a0fc5af81877e64598301064ea2910e67f9` (draft PR #8).

Direct Git pins: Arithmetic `1a54d3c7cbbae4e73325cc70fd2777a2427b1504`; Rational `35a8e9cc629ee578fe7b624e2134929ce7eeff8a`. Transitive Numeric `19b6747cd852a47a694a020b97ba70b6b3ef259b`. Cargo.lock is retained.

Public-only independent consumer checks 79 deterministic, independently generated SymPy ZZ[x]/Q[x] exact GCD expectations, symmetric inputs, integer divisibility, dense/sparse/shared/repeated/negative/unlucky/boundary cases, plus arithmetic, evaluation, derivative, canonicalization, monic Q[x], division and typed errors, and 4095-bit coefficients.

Windows 11 x86-64 and Ubuntu WSL2 Linux x86-64 Rust 1.99.0 all/no-default test modes PASS. Rustfmt, warning-denied Clippy, Rustdoc PASS. AArch64 Linux, WebAssembly, Thumb v7em, RISC-V no-default checks PASS **compile-only**. Actual native ARM64 execution NOT RUN.

Both consumer configurations keep GitHub URL and exact SHA; once source objects were fetched, all tested builds used `--offline --locked`. WSL2 lacked Git credentials; Git objects from the verified exact source bundle were imported locally without source substitutions.

These results advance Q0/Q1/Q2/Q3/Q5 **subsets**, not whole gates. Full cross-family qualification, packaging/publishing, supply-chain/licensing audit, fuzz/mutation/Miri, supported native-target runtime breadth, memory/stack bounds and release Q6 NOT COMPLETE. No authoritative M1 status promotion. PR #10 remains experimental because adversarial shared-factor latency materially regresses against this baseline.

Reproduce: `cargo test --offline --locked --all-features`, `cargo test --offline --locked --no-default-features`, `cargo clippy --offline --locked --all-targets --all-features -- -D warnings`. Independent oracle generator: `python scripts/generate-adversarial-gcd.py`.
