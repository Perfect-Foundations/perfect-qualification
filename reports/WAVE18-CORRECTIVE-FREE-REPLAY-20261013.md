# Wave 18 corrective source graph — free local replay (2026-10-13)

## Revisions and execution scope
- Exact **Error Correction** Git source: `3a57c79d6d34c1e0a975c70e9076e023a45eec87`.
- Exact **Finance** Git source: `be084cb969b9ab7b4c4c7696422a2174d8bc85e4`; its production source is unchanged from `82c6c71ae7b94942ceb47259dcc1f995b4147ba3`.
- Qualification source graph: `e1f579de8f261b36c65b7e4947ab93ea6f210e9b` (this report is documentation-only).
- Host: Zen Windows x86_64, Ubuntu WSL2 x86_64, and `qemu-aarch64` user-mode on the same x86_64 host.
- Rust 1.99.0, `--locked --offline` for all consumer test commands.
- Zero GitHub-hosted runner use. No paid billing or remote hosted CI.

## Local cache recovery
WSL2 Cargo could not fetch the new exact private Git revisions due to unavailable Linux Git authentication. Rather than alter dependencies, require billing, or use path substitutions, we created **local Git bundles** from the already checked-out, exact-HEAD Windows production repositories and fetched those into the corresponding Linux Cargo bare Git caches. Subsequently Cargo compiled the exact `git+https` dependency SHAs shown above while remaining locked/offline. This is a source-cache transport, **not** a dependency override or a new qualification oracle.

## Passed checks

| Check | Error Correction consumer | Finance consumer |
|---|---|---|
| Windows native x86_64 all-target tests (no-default and all features) | PASS | PASS |
| Ubuntu WSL2 x86_64 all-target tests (no-default and all features) | PASS | PASS |
| Ubuntu WSL2 x86_64 `clippy --all-targets -D warnings` (both feature modes) | PASS | PASS |
| Linux AArch64 user-mode QEMU `cargo test --target aarch64-unknown-linux-gnu --all-targets` (both feature modes) | PASS | PASS |

All four new full QEMU consumer commands exited 0, and the corrected GF(17) shorter/empty batch public-consumer regression passed. Windows Native format/Clippy checks were verified in the preceding corrective audit.

From the Qualification root, `python3 verification-wave3-graphs.py` exited 0 and confirmed exact Git dependency and lock identity, retained frozen-corpus SHA256 values, independent GF(7) Berlekamp–Welch regeneration (16), packed Hamming (48), Finance currency (6), Finance date-range (14), and exact historical Finance benchmark sources. Exact LF-normalized Cargo.lock SHA256: Error Correction `0330dad2449ccaa8ba8126044dcfe5e2c18db69fae9e992f811d296799e7ba19`; Finance `77f9780dec6d977f43201ba436880bf07fbe3a382e8adbed864b006113639afe`.

## Boundaries

This completes the formerly blocked Linux and emulated AArch64 **consumer verification** for the corrected source graph, without paid GitHub-hosted runners. It is not physical ARM64 hardware execution, native ARM64 benchmarks, formal M1/Q6 acceptance, hosted CI, or a merge/release. GitHub-hosted job provisioning remains externally blocked; no failed-to-start check is classified as success. The local Qualification worktree contained uncommitted same-purpose pin/test changes and unrelated `qualification/wave6-performance/` data. Those were preserved, not reset or deleted. No source code or frozen vector was modified by this report. No lifecycle percentages were promoted.
