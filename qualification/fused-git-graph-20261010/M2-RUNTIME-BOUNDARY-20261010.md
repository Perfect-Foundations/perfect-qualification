# Additional source-exact M2 runtime-boundary assurance — 2026-10-10

**Current immutable consumer graph:** Polynomial `a64b9a108447c93b60d9bcc469ab99f3ae2484dd`, Arithmetic `d99aef2a9ef230eb725adaef664b0cc0214dc4bd`, Rational `b45de7e7ad63b0850e1d3bbc67e38163b3c8fa7c`, Numeric `19b6747cd852a47a694a020b97ba70b6b3ef259b`. This evidence supersedes the `5251f37` graph for the new tests; original sources, lockfiles, V1/V2 exact bytes and SHAs remain preserved in history. No algorithm code or public API changed in this follow-up Polynomial SHA, only private test observability, tests and evidence.

## Windows build-capacity incident and completion

Zen exposes one Windows filesystem volume (C:), initially **385,183,744 bytes free** at this execution's start (improved from the prior zero-byte exhaustion, still critically low). Ubuntu WSL2 `df` reported 764G available inside its ext4 filesystem, but that is **not** independent host physical space. Verified failed isolated GitHub checkout `_qual_source_prs_5251f37_clean` at exact `5251f37b854e21a74b9850aaa965c628f5e08955`, clean Git status, target created during failed build, ignored via `.gitignore: **/target/`. Only that isolated checkout's generated Cargo output was cleaned using `cargo +1.99.0 clean --manifest-path <isolated>/qualification/fused-git-graph-20261010/Cargo.toml`: 938 files, 97.8 MiB. No project source, Git branch/worktree, untracked evidence, original Cargo cache/toolchain, WSL virtual disk or unrelated application data removed.

Completed the exact interrupted **Windows fresh GitHub checkout `--locked --no-default-features` test** with Rust 1.99.0 and the original `5251f37` graph by setting process-local `CARGO_INCREMENTAL=0`, `CARGO_PROFILE_DEV_DEBUG=0`, `CARGO_PROFILE_TEST_DEBUG=0`. All 10 independent mathematical tests passed (exit code 0). Measured free C: space after the successful repeat: **447,164,416 bytes**. No alternate physical Windows volume was available. The capacity hazard remains and requires an owner-controlled storage plan for sustained builds; do not create massive duplicate targets.

## Deep PRS resource-boundary verification

`src/primitive_prs.rs` retains the original verified 4096-bit exact post-product correction. Additional `#[cfg(test)]` source tests directly inspect `within_bounds`, `lift_q`, `try_z_gcd` and `try_q_gcd` to establish eligibility boundaries **degree 32 accepted / 33 refused**, **denominator LCM width 512 accepted / 513 refused**, and **retained lifted Integer magnitude width 4096 accepted / 4097 refused**. Public Q GCD stays mathematically exact and monic after private refusal. Prior PRS cancellation-at-the-boundary and last completed-pair continuation tests remain in full test matrices.

## Fraction-free runtime growth and independent exact field fallback

New `#[cfg(test)]` `INTERMEDIATE_REFUSAL` flag is emitted only when the existing runtime 4096-bit scale/remainder check actually returns `None`; old flags remain thread-local and are reset before every assertion. No new production runtime state or public API. New source regression tests `A=x^{60}`, `B=x+C`, `C=2^{96}+3`. Its preflight estimate qualifies for fraction-free division, but intermediate coefficient growth triggers the **runtime guard** (not step/degree/LCM/preflight refusal). Public `div_rem` then executes field fallback. Complete exact expected coefficients, derived independently from division by linear `x+C`:

```text
Q[k] = (-C)^(59-k), 0 <= k <= 59
R[0] = (-C)^60, R has degree zero and 5761-bit numerator
A = B*Q + R
```

Both selected internal event flags (`INTERMEDIATE_REFUSAL`, `FIELD_FALLBACK`) and full coefficient results are asserted in source tests. Qualification `tests/m2_runtime_fallback.rs` independently checks all 60 Q coefficients, nonzero R, degree and exact reconstruction from the public API, plus Q/Z zero-GCD, signed leading/content normalization and exact ±1/nonunit constant division. Qualification `references/verify_runtime_guard_oracle.py` separately reconstructs the full identity using Python's arbitrary-precision Fraction, and SymPy 1.14.0 verifies ZZ/QQ zero-input GCD expected complete coefficients (oracle pass). This is new M2 behavioral coverage, not a benchmark.

**Source-exact verification:** full Windows x86-64 and WSL2 Linux x86-64 Rust 1.99.0 Polynomial tests all/no-default, warning-denied Clippy on both, Windows rustdoc; current Qualification **12/12 independent Rust tests** all/no-default on both hosts, `verify_graph.py` one Arithmetic package identity and SHA-verified Cargo.lock. No production dependency feature changes or local path overrides. WASM/Thumb/RISC-V/Linux AArch64 remain compile-only based on earlier source revision; physical ARM64 and hosted GitHub runner execution remain unverified. Source ready for independent review, **not** independently approved, M1/M2 accepted, qualified for release, or published. No new source-exact latency/allocation benchmark was run.
