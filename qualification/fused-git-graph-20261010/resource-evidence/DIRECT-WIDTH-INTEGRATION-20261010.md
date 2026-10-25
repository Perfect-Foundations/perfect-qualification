# Direct signed-magnitude bit inspection: final immutable Git integration evidence (2026-10-10)

## Source provenance

| Package | Immutable Git revision |
|---|---|
| Arithmetic | `d99aef2a9ef230eb725adaef664b0cc0214dc4bd` |
| Rational | `0afd08d27374f472127f3bf3cf5728e0a1170bd7` |
| Polynomial | `f28aa8751cd85526fe0ada6c94c3d49b38d8cd94` |
| Numeric | `19b6747cd852a47a694a020b97ba70b6b3ef259b` |

Original graph at Qualification commit `1333caa009d0cf9790c6274d9f14b46dc7c43bbc` is retained verbatim in `history/source-exact-1333caa.json` and `history/Cargo-1333caa.lock`; the original resource CSVs remain unchanged. This new graph uses pure Git Cargo revisions, single Arithmetic package identity through direct and Rational-transitive edges, no `[patch]` or local sibling-source dependency, and a canonicalized-LF Cargo.lock hash verified by `verify_graph.py`.

## Exact source and mathematical contract

`Integer::unsigned_bit_length()` uses a borrowed `BigInt::magnitude().bits()` to return the exact unsigned magnitude bit count, with zero 0 and sign-invariant widths; no heap clone. The existing `Natural::bit_length()` already uses `BigUint::bits()`. Arithmetic source `tests/integer_fused.rs` and independent Qualification consumer test `direct_unsigned_width_exact_power_boundaries_and_fused_operators` check powers-of-two, neighbors, sign, and zero up to 16384 bits.

Polynomial changed 11 width-only guard call sites, preserving all degree, Rational denominator LCM, and retained width limits. The PRS constant divisor-leading width is cached; noncancelling product-width guard uses saturating addition to avoid overflowing `u64`; affected coefficients continue to be checked **after exact fused multiply/subtract** to preserve valid cancellation above transient limits. The input constant `2^2048` times leading `2^2047` produces a 4096-bit retained coefficient and is accepted; increasing input to `-2^2049` is exactly 4097 bits and correctly declines. Seven-step late continuation, GCD content, rational monic normalization and fallback all pass unchanged.

The independent consumer's **six** mathematical tests pass on Windows and Ubuntu WSL2 in both feature matrices with Rust 1.99.0; fmt, warning-denied Clippy, warning-denied rustdoc pass. The complete Polynomial source suite likewise passed Windows and WSL2 all/no-default, exact SymPy vectors, and typed errors.

**Compile-only portability:** Windows `wasm32v1-none`, `thumbv7em-none-eabihf`, `riscv64imac-unknown-none-elf`; Ubuntu WSL2 `aarch64-unknown-linux-gnu` with `--no-default-features --locked` all PASS. None constitutes physical ARM64 runtime execution.

## Windows release complete-operation resource measurements

Instrumented Polynomial `division_targeted` harness; Rust 1.99.0; three fresh process runs per mode; same mathematical inputs as prior graph evidence.

| Complete workflow | Previous graph allocs | Current allocs | Previous requested bytes | Current requested bytes | Current peak incremental live bytes | Current mean ms |
|---|---:|---:|---:|---:|---:|---:|
| Integer degree64/64 exact division | 4,163 | 4,163 | 110,600 | 110,600 | 9,304 | 0.29037 |
| Rational degree128/128 nonmonic division | 18,275 | **16,469** | 1,019,312 | **939,992** | 44,640 | 1.36280 |
| Rational degree40/20 coprime GCD | 832 | **708** | 36,064 | **32,464** | 10,968 | 0.05037 |
| Integer multiply→divide degree128 | 9 | 9 | 32,960 | 32,960 | 20,576 | 0.50320 |
| Rational multiply→divide degree128 | 19,115 | **17,308** | 1,144,040 | **1,064,264** | 59,200 | 1.78217 |
| Z[x] late-refusal PRS GCD | 817 | **595** | 119,432 | **99,792** | 9,208 | 0.92460 |
| Q[x] late-refusal PRS GCD | 861 | **639** | 120,664 | **101,024** | 9,208 | 0.92140 |

The exact Integer degree64 case already gained its fused update in the prior revision, so no additional gain is claimed from bit inspection. No sampled peak-live metric improved. The PRS late-continuation timings are flat within these samples, so only allocation/requested-byte improvement is claimed.

Ubuntu WSL2 three process samples (native Linux x86-64; not physical ARM64) independently measured: Q[x] degree128 nonmonic division **16,469 allocations / 939,992 requested bytes / 44,640 peak / 1.10932 ms mean**; Z[x] late PRS GCD **595 / 99,792 / 9,208 / 0.98733 ms mean**; Q[x] late PRS GCD **639 / 101,024 / 9,208 / 0.97588 ms mean**. Linux release timings must not be directly compared to Windows as if environments were identical.

Windows CSVs `width-{integer-exact,division,gcd-selected,workflows}-win-{1,2,3}.csv`; Linux CSVs `width-{prs,division}-linux-{1,2,3}.csv`. All are complete public operation samples with test-harness instrumentation, not an RSS or power measurement.

## Security and acceptance boundaries

The implementation adds no mandatory runtime dependency, no build script, no unsafe production code, no FFI or native runtime, and no non-Git source override. `num-bigint` and Numeric pins remain unchanged. The width inspection is exact for all representable BigInt values; library allocation remains unbounded for genuinely unbounded arbitrary-precision operations. Existing `publish = false`, no_std + alloc, and safe Rust policies remain unchanged.

GitHub hosted Actions for Arithmetic run `38039452645` (nine jobs) and Rational run `38039504402` (seven jobs) still fail with **no assigned runner and zero executed steps**. This is a platform/account gate, not evidence of source-level pass/fail; root cause is not established. Independent API approval, native physical ARM64 runtime evidence, formal M1/M2 milestone signoff, broader family Q0–Q6 and owner license/release decisions remain outstanding. `project-status.toml` implementation stage and reported progress are not promoted.
