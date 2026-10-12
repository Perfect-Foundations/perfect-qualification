# Exact-source FMA correction performance comparison — 2026-10-11

## Scope and interpretation

A small, reproducible same-host Rust 1.99 *microbenchmark* compares
Perfect Uncertainty source revisions:

- Baseline: `66af44422c38f5c86d4e4b3c44f8ae2a37c1a59c`
  (FMA multiplication residuals, prematurely collapsed compensated row).
- Corrected: `1450a418f5473b3e173a0cc79c65f0ce55d29427`
  (separate high/low row FMA products through the outer reduction).

The separately locked benchmark harness is retained at
`qualification/perfect-uncertainty/benchmarks/fma-comparison`
with exact Git refs. It does not change the independently pinned
Qualification consumer's production dependency graph.

## Reproduction commands

From the benchmark harness directory:

`cargo run --release --locked --offline`

Use a reused external `CARGO_TARGET_DIR` if preserving workspace disk
or comparing between checks. Both compared crate revisions are built
together in **one** optimized binary. Cases are constructed identically;
the production calls and their results are `black_box`-protected.

Rust `Instant` timing, 20,000 calls per case, one warm-up measurement,
then the median of five elapsed-time-per-call measurements. These results
are *host observations*, not platform guarantees or certified profiling.
No attempt was made to lock CPU clocks, isolate cores, or control
background workload. No significance/confidence interval is claimed.

## Tested hardware

- Physical CPU: **AMD Ryzen AI 9 365 with Radeon 880M**.
- Windows: `rustc 1.99.0` host `x86_64-pc-windows-msvc`, optimized release.
- Linux: Ubuntu WSL2 kernel `6.18.33.2-microsoft-standard-WSL2`,
  `rustc 1.99.0` host `x86_64-unknown-linux-gnu`, optimized release.
- Windows and WSL2 are on the **same physical processor**.

## Re-executed median observations (nanoseconds/call)

| Workload | Windows baseline | Windows corrected | WSL2 baseline | WSL2 corrected |
|---|---:|---:|---:|---:|
| 4D independent | 108.64 | 119.81 | 140.31 | 154.73 |
| 8D, rho=0.5 | 397.99 | 421.58 | 487.93 | 522.51 |
| 4D, rho=1−2^-53 | 130.81 | 139.21 | 164.54 | 172.56 |
| 8D, rho=1−2^-40 | 487.68 | 507.40 | 579.21 | 604.98 |

Computed corrected/baseline ratios:

| Workload | Windows | WSL2 |
|---|---:|---:|
| 4D independent | 1.103× | 1.103× |
| 8D half-correlated | 1.059× | 1.071× |
| 4D near-singular | 1.064× | 1.049× |
| 8D near-singular | 1.040× | 1.044× |

The observed numerical cost of the correctness change was approximately
**4–10%** on these four inputs. This cannot be generalized to all
workloads, compilers, CPUs, distributions or matrix conditions.

The near-singular four-dimensional case **does not have semantically
equivalent baseline output**:

- Baseline variance: `3.22044606035254333`
- Corrected variance: `3.22044604925031308`
- Independent exact Fraction reference: `3.220446049250313`

The faster baseline in that case is mathematically less accurate.
Do not treat a speed comparison of these unequal numerical results
as an algorithmic superiority claim.

No performance gate, formal M1 acceptance, native ARM64 benchmark,
independent cross-machine replication or released version is implied.
