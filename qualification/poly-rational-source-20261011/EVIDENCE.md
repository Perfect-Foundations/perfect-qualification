# Source-exact Polynomial + Rational production update qualification

**Executed:** 2026-10-10 America/New_York / 2026-10-11 UTC.

## Frozen four-crate graph

- Polynomial `8a40463f3df16393fd48210e5941cff146da9954` (derivative and sparse/LCM production improvements, with Rational repinned).
- Rational `1d2ad58ee8f0eae6dec5ae941574eaec538150ae` (canonical zero constructor optimization).
- Arithmetic `d99aef2a9ef230eb725adaef664b0cc0214dc4bd`.
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
