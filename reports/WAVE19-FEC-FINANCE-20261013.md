# Wave 19 — Functional Error Correction and Finance qualification

## Exact production graph
- Perfect Algebra `20b2f10ae3390808d412be84c7af76ffbcb2766d` unchanged.
- Perfect Error Correction `0c8b40568545fee98b5ebae8edc36b23e4463ab0` (PR #2).
- Perfect Finance `f91e5ae9b9d297721afe2d507d74ad22a5e689d0` (PR #2).
- Lower-layer pinned revisions remain as recorded in `verification-wave3-graphs.py`.
- The complete source/lock identity was validated without path substitution; frozen mathematical vector files and their SHA256 values were retained.

## New production API and correctness
**Error Correction:** `RsDecodeSession::decode_batch` reuses the bound decoder workspace and session-owned result buffers, returning caller-owned per-word outcomes. Global admission refusal preserves previously committed results, per-word errors remain isolated, and admitted output lengths match word count. `RsDecodeSession::rebind` validates and prepares a replacement workspace before committing a different code identity; on success it clears old-profile cached outputs. The existing single-word last-success contract remains intact. No new field arithmetic hierarchy or coding standard is claimed.

**Finance:** `LedgerOperation` and `OwnedCashFlowLedger::apply_operations` implement explicitly operation-bounded atomic mixed appends/replacements/removals, with ordered fresh stable IDs, duplicate-target rejection, no partial mutation, and no IDs consumed on refusal. `aggregate_cash_flows_in_range` and `net_cash_flows_by_currency_in_range` now validate all supplied Decimal operands **before** filtering dates, aligning the standalone APIs with validated borrowed batch semantics; previously an invalid *excluded* operand could be silently accepted. Arithmetic remains exact with explicit currency identity and no FX.

## Public independent consumer evidence
- `qualification/perfect-error-correction/tests/wave19_session.rs`: the existing independent frozen GF(17) n=16/k=12 message/codeword oracle, injected error position, malformed input, per-word status, admitted shrink/empty, global refusal, and field-profile rebind.
- `qualification/perfect-finance/tests/wave19_ledger.rs`: mixed transactional edits, retained stable IDs, failure/operation-limit refusal, independently specified exact USD/EUR outputs, indexed/standalone differential comparisons, and excluded-invalid-amount rejection.
- The earlier independent reference corpora and checked hashes remain unchanged. New platform repetitions do **not** add new independent reference cases.

## Executed matrices (Rust 1.99.0, locked/offline)
Both production crates passed Windows Native x86_64 and Ubuntu WSL2 x86_64 all-target tests under `--no-default-features` and `--all-features`; both configurations passed warning-denied Clippy and rustfmt. Windows warning-denied rustdoc passed. Windows no-default compile-only checks passed on `wasm32v1-none`, `thumbv7em-none-eabihf`, `riscv64imac-unknown-none-elf` (not hardware execution).

Independent consumer tests passed on each of Windows Native, Ubuntu WSL2 and Linux AArch64 user-mode QEMU, each under both feature configurations. For each single platform/configuration execution, 19 Error Correction and 12 Finance consumer tests passed, zero failed or ignored; these counts are **test executions** rather than newly generated frozen vectors. On Ubuntu WSL2, warning-denied Clippy passed for both consumers and both feature configurations.

Wave 19 production AArch64 user-mode QEMU all-target tests also exited successfully under both feature configurations: Error Correction 23 passed no-default, 19 passed/3 deliberately ignored in all-features; Finance 23 passed no-default and 23 passed/2 deliberately ignored in all-features. The ignored production tests are not counted as passing. Physical ARM64 hardware evidence remains separate.

`python3 verification-wave3-graphs.py` passed exact Git/Cargo.lock identities, corpus SHA256s, GF(7) BW regeneration (16), packed Hamming (48), Finance currency net (6), date ranges (14), and historical Finance dual-revision benchmark identity. LF-normalized lockfile SHA256: FEC `7162019f6414380b6b02fafc3e3cbcbf47066c1d5056080c5a58edd5b19f64a9`; Finance `3c99eee37340464f11f46aae2a40c799ae15e6e9fa8d4f9874017cd125a3ba9a`.

## Scope limits
No comparative new latency, allocation, scoped-peak, or retained-capacity measurements were accepted this wave; Wave 18 measurements remain revision-specific historical evidence, not claims about the new APIs. No physical ARM64, formal M1/Q6 acceptance, protected merge, publication, or release is claimed. Lifecycle progress remains unchanged. Perfectπ was not modified. Historical untracked Qualification and performance worktrees are preserved.

## Exact-source clean-checkout replay
Final detached worktree replay of the committed Qualification SHA must be recorded after commit and before final delivery; source-level local working tree tests above do not substitute for that gate.
