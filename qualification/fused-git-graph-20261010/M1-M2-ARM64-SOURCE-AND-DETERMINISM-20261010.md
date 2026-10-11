# M1/M2 source-exact ARM64 QEMU source suite and V1/V2 parity

**Execution date:** 2026-10-10 local America/New_York (crossed into 2026-10-11 UTC).
**Status:** executed mathematical/portability evidence; **not** native physical ARM64 acceptance, independent reviewer approval, CI, or release.

## Immutable dependency graph and environment

- Polynomial: `59fa967bdf2c20dbde7a93acb22979c8625441af`.
- Qualification parent before these additional evidence files: `17f57b6c14f0846e7a7caea69545427193144bcf`.
- Arithmetic: `d99aef2a9ef230eb725adaef664b0cc0214dc4bd`.
- Rational: `b45de7e7ad63b0850e1d3bbc67e38163b3c8fa7c`.
- Numeric: `19b6747cd852a47a694a020b97ba70b6b3ef259b`.
- Source and consumer `Cargo.lock` unchanged by the ARM64 experiment.
- Host: Zen Windows x86-64; Ubuntu WSL2 x86-64; Rust 1.99.0.
- Executed target: `aarch64-unknown-linux-gnu`; cross linker `aarch64-linux-gnu-gcc`.
- Emulator `qemu-aarch64 8.2.2`, target Linux sysroot `/usr/aarch64-linux-gnu`.
- Reused local Polynomial source checkout `C:\Dev\PerfectFoundations\_poly_fused_integer_20261010` exactly at pinned SHA. No new checkout.
- Reused target: `/home/ubuntu/pf-arm64-qualification-target` for both feature configurations and deterministic examples, bounded to 2 Cargo jobs, disabled incremental/debug output.
- All source and consumer builds use `--locked --offline`. QEMU runner is configured via `CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_RUNNER=qemu-aarch64` with `QEMU_LD_PREFIX=/usr/aarch64-linux-gnu`.

## Actual full Polynomial source execution

Command shape, once for each feature mode (the manifest is the **source checkout**, not the consumer):

```sh
cargo test --manifest-path /mnt/c/Dev/PerfectFoundations/_poly_fused_integer_20261010/Cargo.toml \
  --locked --offline --target aarch64-unknown-linux-gnu --no-default-features
cargo test --manifest-path /mnt/c/Dev/PerfectFoundations/_poly_fused_integer_20261010/Cargo.toml \
  --locked --offline --target aarch64-unknown-linux-gnu --all-features
```

Both use the bounded environment documented above. The full suite, including `src/` unit tests and source `tests/` independent oracle/guard tests, executed:

| AArch64 emulated feature configuration | Source tests | Failures | Process exit |
|---|---:|---:|---:|
| No default features | 94 | 0 | 0 |
| All features | 94 | 0 | 0 |

QEMU source process runtimes were approximately 186 and 177 seconds respectively. Console output was observed through the connected terminal; there is **no committed complete raw source-test console log** from those runs. The earlier independent consumer `24/24` executions remain separately recorded under `M2-QEMU-AARCH64-EXECUTION-20261010.md`; they are not presented as new source executions.

## Current-source canonical V1/V2 parity across architectures

The **existing** consumer examples `examples/canonical_workload.rs` (V1) and `examples/canonical_boundary_v2.rs` (V2) were built at the exact pinned four-crate source graph for ARM64, then QEMU executed each existing example in both feature modes. Their explicit output-file argument was honored; output was compared as raw bytes to all four previously committed Windows and WSL2 Linux x86-64 references for its corpus. The examples and historical reference streams were unchanged.

The reproducible launcher is `references/verify_arm64_emulated_determinism.py`. Run it with `--target-dir /home/ubuntu/pf-arm64-qualification-target`; it verifies the immutable graph and Rust 1.99.0, reuses the Cargo target, builds both examples offline and locked, executes QEMU with the ARM64 sysroot, and verifies byte-for-byte equality including SHA-256, length and complete records. Temporary files reside on WSL storage and are removed after each execution. It retains four verified raw ARM64 output files in `determinism-evidence/arm64-qemu-59fa967/`.

| Source stream | Executions | Records in each | Exact bytes in each | SHA-256 in each |
|---|---:|---:|---:|---|
| V1 | 2 | 37 | 7,225 | `4bd81775891439f48cee151c08a4a58293b4d1bd1e9d3c1a28d4e9b4da4c388d` |
| V2 | 2 | 27 | 4,165 | `87c8a080eeedca85482b486e47c03226a7266a3d3898efeb6ac5fcc3fd27b3fc` |

All **four** QEMU programs exited zero and matched the original canonical streams **byte-for-byte**; the driver itself exited zero. This adds emulated ARM64 data to the existing four native x86-64 execution instances for each canonical corpus. No expected fingerprints were rewritten or relaxed.

## Newly independent bounded exhaustive mathematical assurance

The separate independent SymPy 1.14.0 `references/gen_m2_degree2_gcd_sympy.py` builds 37 canonical integer polynomials from signed degree-0..2 coefficient classes, augmented by explicit nonunit contents and sign cases. For **all 1,369 ordered pairs**, the generated pinned oracle stores full exact `Z[x]` positive-leading/content GCD and `Q[x]` monic GCD coefficient vectors. `references/m2_degree2_zq_gcd_sympy_v1.tsv` is 27,518 bytes, SHA-256 `622490f8c3c72bcffbd4889ba8d572043cf35ac7ef393801a79fa194bce256f6`. The consumer `tests/m2_exhaustive_zq_gcd_sympy.rs` checks both full GCD vectors and exact integer divisibility/reconstruction for each pair, not merely degree or leading coefficient.

New independent test: **1/1 pass** in **each** feature mode on Windows x86-64, WSL2 x86-64 and QEMU AArch64. The complete expanded Qualification consumer is now **25/25** in **both** modes on all three execution environments, with all six commands exiting zero. Existing Polynomial source remains pinned and unchanged.

Windows warning-denied all-target Clippy passed both feature modes; rustfmt check and `verify_graph.py` passed, confirming a single Arithmetic package with no override.

## Strict assurance boundary

QEMU executes AArch64 instructions but is not native physical ARM64 hardware. The evidence establishes exact corpus equality **on tested configurations only**; it does not prove arbitrary inputs, hardware timing, physical memory behavior, native AArch64, hosted GitHub Actions, API/security approval, release packaging, or milestone acceptance. Formal M1 remains pending at **20%**; no change to source status, protected branches, Perfectπ or LMES is asserted.
