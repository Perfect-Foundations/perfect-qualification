# Wave 15: source-exact Algebra-to-Reed–Solomon qualification (2026-10-12)

These are **draft, unmerged** development source revisions:
- Perfect Algebra production: `da90fb38b1491aac8caf8600c0943d5a3bab388c`, optimizing private polynomial helpers without changing finite-field identities or APIs.
- Perfect Error Correction production: `c7b6ebfb4ef10630378f2f724f7e81bdc7e4f1fb` consuming the exact Algebra revision via Git/Cargo.lock.
- Independent Perfect Algebra Qualification PR #11: `3402fe565eed6716213887b674f542f4bfebf909`, source-pinned to that Algebra commit.
- Finance production remains `1752f18f9d17fc59a8440dcb48abb9f3d9f986f3` (unchanged).

## Actual independent reference and source verification

The prior immutable mathematical vector corpora remain unchanged. No new independent row count is asserted. The existing GF(5), GF(7), GF(16), and GF(17) exact-reference and differential/erasure tests were rerun at the new source graph, as were the independent Finance frozen references (graph generator).

The downstream `verification-wave3-graphs.py` checks exact named SHA pins in manifests, complete Cargo.lock identities, and immutable corpus hash digests for both consumers. After repinning FEC and Algebra, Windows Native and Ubuntu WSL2 x86-64 passed locked/offline `--no-default-features` and `--all-features` Rust tests and warning-denied `--all-targets` Clippy. This is one physical x86-64 computer using two operating system environments.

## Repeated same-host before/after FEC performance

A pristine detached FEC baseline at `55a5fd368f008dd8b8bbbea1751829d5e1dc99aa` (which pins Algebra `6a2d4924466ebeab79b849f56e6d9a8b9acfdf24`) was compiled separately from current FEC `c7b6ebf` (pinning Algebra `da90fb3`). Each optimized Windows Rust 1.99 build used a **separate Cargo release output directory**, avoiding known cross-revision cache aliasing. The same retained `tests/heap_measure.rs` benchmark was run 15 times per revision in alternating order, preserving equivalent decoded messages, fields, word lengths and error/erasure positions.

| GF(16) RS workload (n=8,k=4) | Baseline median ns | Current median ns | Current/baseline | Baseline min–max ns | Current min–max ns |
|---|---:|---:|---:|---:|---:|
| 2 errors, reused workspace | 523800 | 377200 | 0.7201 | 505300–762700 | 365200–404100 |
| 2 errors, fresh workspace | 590100 | 416700 | 0.7062 | 567800–632600 | 406600–442400 |
| Erasures, reused workspace | 100500 | 68600 | 0.6826 | 95900–128500 | 67800–82200 |
| Erasures, fresh workspace | 147800 | 102500 | 0.6935 | 133700–222900 | 97100–119700 |

Median observed latency improvement ranged from approximately **28% to 32%** in these narrow workloads. No universal guarantee or hardware-independent claim.

**Allocation counts did not decrease** for the paired downstream FEC benchmarks. Example: GF(16) two errors, reused workspace: **1,541 allocations** on both baseline and current, with 488,312 requested bytes; fresh workspace: 1,670 allocations on both. The speedup follows fewer unnecessary base-field operations, **not** fewer heap allocations. Scoped peak heap figures likewise have no established improvement.

The independent Algebra source evidence and its own 15-alternating-run dense-op benchmark live under `perfect-algebra/verification/WAVE15-SPARSE-POLYNOMIAL-20261012.md`.

## Remaining gates

The final exact Qualification commit's detached clean checkout is verified separately after commit. No physical ARM64, hosted CI, formal M1/Q6 review, protected merge, or release is claimed.
