# Source-exact Polynomial + Rational production update qualification

**Executed:** 2026-10-10 America/New_York / 2026-10-11 UTC.

## Frozen four-crate graph

- Polynomial `a7909f9ce2b41db9d2cf318ee326b5f44e0a1b31` (derivative and sparse/LCM production improvements, with Rational repinned).
- Rational `dfe536c88c69dc36b188893045dc03434855371c` (canonical zero constructor optimization).
- Arithmetic `65d8f55641739d5d6a11c17fe7d68c8898d984d7`.
- Numeric `19b6747cd852a47a694a020b97ba70b6b3ef259b`.

The `source-exact.json` manifest records and verifies the normalized `Cargo.lock` hash, all four exact Git source identities, the 10 imported independent test-source hashes, and the immutable SymPy GCD fixture hash. The `verify_graph.py` script refuses mismatches. There are no path patches, source overrides or duplicate Arithmetic package identities; `cargo tree -d --locked --offline` returned no duplicates.

**Original Qualification corpus is not rewritten.** This new consumer imports the existing ten frozen independent test files from `../fused-git-graph-20261010/tests` via test-only `#[path]` directives and protects their exact normalized bytes. The earlier Qualification source graph and its V1/V2 determinism fingerprints remain valid only for their earlier tested SHAs.

## Actual execution

Cargo `--locked --offline`, Rust 1.99.0, two jobs, incremental builds disabled, no changes to production numerical semantics except the separately verified optimizations.

| Executable environment | No-default features | All features |
|---|---:|---:|
| Native Windows x86-64 | 25 existing + 2 new passed | 25 existing + 2 new passed |
| Native Ubuntu WSL2 x86-64 | 25 existing + 2 new passed | 25 existing + 2 new passed |
| Linux AArch64 via QEMU user emulation | 25 existing + 2 new passed | 25 existing + 2 new passed |

**27/27 per mode/host, six zero-exit executions.** The two new tests verify CPython Fraction full polynomial product coefficients, signed zero denominator normalization, exact division/monic GCD and derivative, and 4096-bit fused Arithmetic cancellation and signed magnitude bit widths. Both Windows Clippy all-targets modes passed with `-D warnings`, plus rustfmt and exact lockfile/fixture checks. WSL2 dependencies were imported from existing Windows Cargo Git object caches after online WSL Git fetch stalled; the graph was tested `--locked --offline` and remained unchanged.

Reproduce using `python verify_graph.py` then `cargo test --locked --offline --no-default-features` and `cargo test --locked --offline --all-features`. For the emulated configuration add `--target aarch64-unknown-linux-gnu`, `CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc`, `CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_RUNNER=qemu-aarch64`, and `QEMU_LD_PREFIX=/usr/aarch64-linux-gnu`.

**Limitations:** QEMU is not physical ARM64; these finite exact tests do not constitute formal API/security review, proof for all mathematical inputs, hosted CI, accepted M1/M2/M4/M5 gates or crates.io readiness. GitHub Actions is separately billing-blocked before runner assignment. Perfectπ was not touched. No milestone percentage is advanced.

## Final-graph verification provenance

Revalidated after the original `43a8ba6` consumer revision against the final production graph above, without changing the ten imported test files or their oracle hashes. Six `cargo test --locked --offline` executions each passed 25 imported and 2 cross-operation integration tests: native Windows x86-64, Ubuntu WSL2 x86-64, and QEMU Linux AArch64; both no-default and all-features. Windows Qualification Clippy `--all-targets -- -D warnings` passed in both feature modes. Windows and WSL2 source-exact graph verification succeeded; `cargo tree -d --locked --offline` reported no duplicates. Full Arithmetic, Rational and Polynomial tests also passed in both modes on Windows and WSL2. Focused M4/M5 changed-operation regressions passed under QEMU in both modes.

The additional `verify_m4_m5_python.py` script checks 14 independent exact Integer/Fraction reference vectors in CPython on Windows and Ubuntu; both reported SHA-256 `87ab8e114247fbb3df5489708821e8fe985693e461ef7ee1f4e7dd9c59b204b6`. It uses exact Python arithmetic rather than the production Rust implementation.

Historical evidence for the prior graph is retained in this consumer's parent Git revision `43a8ba6`, and the separate historical `fused-git-graph-20261010` directory is unchanged. Hosted CI, formal milestone acceptance, and physical AArch64 have not been completed.