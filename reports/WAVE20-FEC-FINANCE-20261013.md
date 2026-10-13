# Wave 20 — Error Correction and Finance source-exact qualification (2026-10-13)

## Exact production revisions
- Perfect Error Correction PR #2: `46339760ae61506793192cbf8b15ca23b1c19d29`.
- Perfect Finance PR #2: `090dabc4b5b8860acb8ea4b07cfded14840ef4a6`.
- Perfect Algebra PR #3 remains `20b2f10ae3390808d412be84c7af76ffbcb2766d`. Lower-level production dependencies were not repinned.
- This Qualification change updates only the two exact Git source pins, their Cargo.lock files, the graph verifier, new public-consumer tests, and this evidence report. No existing mathematical reference corpus was changed.

## Implemented public behavior
**Error Correction:** `HammingCodeProfile` and `ReedSolomonCodeProfile` now retain immutable, validated informational provenance. A documented reference does **not** assert standards or wire interoperability. RS profiles bind and validate decoder sessions, checked single/batch decode and profile-safe rebind, and expose the code's operational trial budget distinctly from the field/evaluation identity. `RsDecodeSession` offers explicit batch-output capacity inspection and storage release independently of its last successful single-word output.

**Finance:** `OwnedCashFlowLedger::apply_operations` now preflights ordered stable IDs, bounds, Decimal validation and failure modes using an ID plan and only edited cash-flow values instead of cloning the entire ledger. All edits remain atomic; failure leaves records and next-ID allocation unchanged. `CashFlowBatch::group_counts_into` preflights exact per-range cardinality by `DateAndCurrency` or `Currency`, with aggregate bounds and transactional caller-owned output. A successful cardinality preview is **not** a guarantee that an eventual exact monetary sum is representable.

## Verification actually executed
On Zen, Rust 1.99.0:
- **Production:** Windows 11 Native x86_64 and Ubuntu WSL2 x86_64 full `cargo test --locked --offline --all-targets` in no-default and all-feature modes: PASS for both crates. Clippy `-D warnings` in both modes: PASS. Rustfmt: PASS. Windows rustdoc with warnings denied: PASS. Compile-only `wasm32v1-none`, `thumbv7em-none-eabihf`, `riscv64imac-unknown-none-elf`: PASS for both crates.
- **Independent Qualification consumers:** Windows and WSL2 x86_64 full all-target tests and warning-denied Clippy in both modes: PASS. New public tests exercise prime GF(17) and extension GF(16) exact frozen codewords across profile bind/rebind, corrupted frames, bounded mixed batches, refusal and output release; and a cross-date/cross-currency exact ledger simulation with transactional edits, duplicated/empty ranges and preflight limits.
- **Local AArch64 user-mode QEMU:** full all-target production tests and source-pinned independent consumer tests, in both no-default and all-feature modes: PASS for both crates. This is emulation, **not** physical ARM64 evidence. Existing deliberately ignored benchmark tests remain ignored.
- **Independent graph/reference check:** `python verification-wave3-graphs.py` exited 0 with the two repinned commits, exact Git/Cargo.lock identities and unchanged independent reference corpus hashes. GF(7) Berlekamp–Welch regeneration (16), packed Hamming (48), exact Finance currency (6) and date-range (14) references passed. Historical source-pinned Finance dual-revision benchmark identity remained intact.
- LF-normalized Cargo.lock SHA256 from the actual local checker: Error Correction `e79cc826c08cd28f8ea4626661960eb0c8256d9991e1ea52d2eab3e69ccec287`; Finance `787031331abcf99343c16f218a75a1d0034e1f0a1bfe2e226a0cce8d9b17f07e`.

## Finance transaction memory and performance (512-flow ledger)
Baseline `f91e5ae9b9d297721afe2d507d74ad22a5e689d0`; committed candidate `090dabc4b5b8860acb8ea4b07cfded14840ef4a6`. Same Windows machine, separate release-mode targets, identical test harness, 15 alternating repetitions per revision, six cases (180 case observations). The source-isolated replay was repeated after the production commit. Values are 15-run medians:

| Operation | Allocations before / after | Requested heap bytes before / after | Scoped peak heap bytes before / after | Runtime ns before / after |
|---|---:|---:|---:|---:|
| One append | 9 / 9 | 98,361 / 77,945 | 65,584 / 41,072 | 30,900 / 16,200 |
| One replace | 6 / 6 | 32,811 / 4,211 | 32,803 / 4,203 | 19,700 / 7,800 |
| One remove | 3 / 3 | 32,776 / 4,176 | 32,776 / 4,176 | 10,400 / 1,700 |
| Mixed edits | 15 / 14 | 98,437 / 78,229 | 65,608 / 41,312 | 24,200 / 15,100 |
| Late resource refusal | 31 / 22 | 98,616 / 13,256 | 65,689 / 9,065 | 21,100 / 6,400 |
| Duplicate-ID refusal | 6 / 6 | 32,819 / 4,291 | 32,811 / 4,283 | 21,500 / 1,800 |

The raw 180-observation CSV was generated and retained locally on Zen as `C:\\Scratch\\wave20_finance_15_alternating.csv` and copied into the Wave 20 Qualification worktree. **It was not uploaded in this connector-only commit because Zen's Windows connection dropped before the final Git push; retrieve and commit the original CSV when the machine reconnects. Do not recreate rows from the medians.** These are scoped allocator-requested/peak numbers, not process RSS. Timing dispersion overlaps in some cases and this does not prove universal speedups.

## Verification boundaries and next gate
A pristine detached-checkout replay of the *final Qualification SHA* remains **not performed**, despite the locally tested exact dependency pins; it must be executed when Zen's original worktree connection is available. The remote-source commit was assembled from GitHub's prior Qualification tree and locally executed Wave 20 public test source, formatted using the separate existing Linux Rustfmt tool. Reconcile the preserved local, uncommitted Wave 20 Qualification worktree with this branch before any future edits; do not reset or overwrite it blindly.

M1 and Q6 formal approval, physical ARM64 hardware, full release qualification, packaging/licensing and publication remain open. Do not promote lifecycle percentages or treat blocked hosted CI as passing. No billing changes, paid resources, protected merge, release, Perfectπ changes or MachinaMesh repository modification were made.
