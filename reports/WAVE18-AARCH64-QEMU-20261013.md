# Wave 18 — AArch64 Linux user-mode execution evidence (2026-10-13)

## Exact sources and host

- Production Error Correction: `f08e8a5435d9a067558fb69009bb618bfa948fb3` (PR #2).
- Production Finance: `82c6c71ae7b94942ceb47259dcc1f995b4147ba3` (PR #2).
- Independent Qualification: `a2b199a9317b03436aa69989e027053299ffc3ed` (PR #12, **pre-evidence-document HEAD**).
- Algebra dependency: `20b2f10ae3390808d412be84c7af76ffbcb2766d`; lower-layer dependencies are recorded in `verification-wave3-graphs.py`.
- Executor: Zen, x86_64 Ubuntu 24.04.5 WSL2 under Windows; **AArch64 user-mode emulation**, not physical ARM64 hardware or an ARM64 kernel.
- Rust toolchain: 1.99.0; target `aarch64-unknown-linux-gnu`; GNU AArch64 cross-linker `aarch64-linux-gnu-gcc`; QEMU user-mode `qemu-aarch64` version 8.2.2; AArch64 glibc loader under `/usr/aarch64-linux-gnu/lib`.
- All commands used `--locked --offline` with no path-substituted dependencies. The production repositories were checked at the exact SHAs above; the Qualification workspace also matched the SHA above.

## Runtime test matrix

Each row represents TWO actual QEMU-executed `cargo test --all-targets` invocations: `--no-default-features` and `--all-features`. The table counts **the combined executions across both configurations**, not independent mathematical reference rows.

| Source | Rust integration/unit tests passed | Failed | Ignored |
|---|---:|---:|---:|
| Error Correction production | 44 | 0 | 4 |
| Finance production | 42 | 0 | 2 |
| Error Correction independent public consumer | 36 | 0 | 0 |
| Finance independent public consumer | 22 | 0 | 0 |
| **Combined** | **144** | **0** | **6** |

Each of the eight full-matrix commands exited with code 0. The deliberately ignored production heap-measurement tests were **not** counted as passing tests. An earlier targeted four-test Error Correction ARM64 decode run passed, but is **already included** in the full production matrix and must not be counted twice.

## Reproduction

Set these environment variables before each `cargo +1.99.0 test` invocation:

```sh
export QEMU_LD_PREFIX=/usr/aarch64-linux-gnu
export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_RUNNER=qemu-aarch64
export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc
```

From each exact production source checkout, run both:

```sh
cargo +1.99.0 test --locked --offline --target aarch64-unknown-linux-gnu --no-default-features --all-targets
cargo +1.99.0 test --locked --offline --target aarch64-unknown-linux-gnu --all-features --all-targets
```

From the pinned Qualification checkout, run those same two commands within each of `qualification/perfect-finance` and `qualification/perfect-error-correction` (using a shared external `CARGO_TARGET_DIR` is optional).

From the Qualification root, the host-side independent graph and reference verifier also passed:

```sh
python3 verification-wave3-graphs.py
```

Verified output includes `WAVE3_GRAPH_LOCK_FROZEN_VECTORS_PASS`, `BW_GF7_INDEPENDENT_REGEN_PASS`, the independent packed Hamming, Finance currency/date-range regenerate passes, and `EXACT_FINANCE_DUAL_REVISION_BENCH_PASS`. LF-normalized lock SHA256 fingerprints: Error Correction `74485757d54b1f7ff98b691a3b4e89b29dde48b13eca0c977552ae0b99b0f49b`; Finance `6f265938966fc209c8375404d185fba9e39253fa03b928954248a4cda4233ef2`.

## Scope and open gates

This is **new emulated AArch64 Linux runtime evidence** at the exact pinned source revisions. It strengthens portability confidence beyond compile-only checks without falsely asserting physical ARM64 hardware validation, independent ARM64 hardware/kernel, native ARM64 performance, or an independent AArch64 operating system. QEMU user-mode shares the x86_64 host's Linux kernel; portability/runtime conclusions are limited accordingly.

The earlier Windows Native and Ubuntu WSL2 x86_64 pristine replay remains separate evidence. Neither this runtime pass nor the source-independent mathematical replay closes formal M1/Q6 gates. Hosted Actions remains skipped on draft PRs, and no protected merge, crates.io publication or release was performed. The production SHAs and mathematical-vector corpora were not changed by creating this evidence-only report.
