# Independent Perfect Polynomial modular-experimental consumer

**Status: qualified behavioral subset of an experimental source revision, NOT production acceptance.**
Source: `Perfect-Foundations/perfect-polynomial` exactly `3f18bb57825a52c5704e82f7bab923b1707cfe77` (stacked draft PR #10 on #9 and #8). Direct pinned Arithmetic `1a54d3c7cbbae4e73325cc70fd2777a2427b1504` and Rational `35a8e9cc629ee578fe7b624e2134929ce7eeff8a`; no local-path source substitution.

Independent non-production source-exact consumer runs the **same 79-case SymPy GCD oracle and public semantic contract** as the conservative PR #8 consumer: canonical Z[x] content/sign and monic Q[x], cross-operation algebra, division/error semantics, and 4095-bit exact fallback. PASS on native Windows x86-64 and Ubuntu WSL2 x86-64 Rust 1.99.0 in all/no-default feature modes; Rustfmt, warning-denied Clippy, Rustdoc PASS. AArch64 Linux/wasm32v1/Thumb v7em/RISC-V no-default cross-compilation PASS **compile-only**.

Additional adversarial source-side evidence lives in separate Polynomial experimental branch `exp-poly-adversarial-m1-20261009`, with independent 79-pair SymPy finite-field expectations (35 positive modular certificates, 0 false positives) and complete GCD timing evidence. It revealed a reproducible **material** shared-factor slowdown vs PR #8 (~3.28→7.50 µs Z[x], ~6.16→9.17 µs Q[x]). A proposed low-degree-difference GCD replacement was mathematically correct but severely regressed unlucky-prime cases, so it was rejected. These negative measurements prevent PR #10 from becoming the default M1 baseline without further justification.

`Cargo.lock` retained for deterministic source resolution. Use `cargo test --offline --locked --all-features` and `cargo test --offline --locked --no-default-features`. WSL2 source cache was populated from a verified exact local Git bundle because Linux Git credentials were unavailable, without changing manifest Git URL/revision.

Remaining: full Q0-Q6 qualification, package publishability, advisory/license audit, native ARM64 runtime, broader property/fuzz/mutation testing, peak heap/stack, and release readiness. **No status percentage promoted**.
