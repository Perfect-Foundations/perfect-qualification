# Reproducible PSD diagonal-pivot correction comparison — 2026-10-11

## Exact source and arithmetic comparison

Benchmark package:
`qualification/perfect-uncertainty/benchmarks/psd-pivot-comparison`
with committed Cargo.toml, tracked Cargo.lock and Safe Rust `src/main.rs`.

Both immutable production sources are compiled **into one optimized executable**:
- Pre-correction `Perfect-Foundations/perfect-uncertainty@1450a418f5473b3e173a0cc79c65f0ce55d29427`.
- Corrected `Perfect-Foundations/perfect-uncertainty@dafe6a28a0dd69042e2b5fcc36ea254f8e16ac60`.

The only relevant change between those revisions is private Schur-complement
symmetric diagonal pivot selection; no changed numerical API or dependency.
Each test constructs identical matrices, passes owned clones to
`CorrelationMatrix::new` and uses `std::hint::black_box` around inputs,
returned classification and matrix result. The benchmark measures full
constructor latency including allocation, source validation and PSD checking,
not propagation latency. The invariant `psd_tolerance=1e-12` is shared.

## Reproduction and method

```sh
cargo clippy --locked --offline --all-targets -- -D warnings
cargo run --release --locked --offline
```

Run in the harness directory with Rust 1.99, optimizing release profile,
`1000` calls per workload, one warmup repetition, then the **median of five**
measured elapsed time-per-call samples. Both hosts share one physical AMD
Ryzen AI 9 365 CPU; Windows uses MSVC x86-64, Ubuntu WSL2 uses Linux x86-64
in a virtualized environment. These are preliminary, same-machine, unpaired
whole-call microbenchmarks rather than precision platform guarantees.

## Actually measured final harness source

| Workload | Windows baseline ns | Windows pivoted ns | WSL baseline ns | WSL pivoted ns |
|---|---:|---:|---:|---:|
| 4D independent | 85.90 | 88.10 | 52.69 | 56.51 |
| 8D equicorrelation 0.5 | 218.10 | 228.90 | 167.49 | 180.26 |
| 16D rank-one | 419.40 | 494.30 | 453.49 | 473.59 |
| 18D exact PSD sign-Gram (prior false rejection) | 1259.80 | 1525.10 | 1290.92 | 1599.37 |

Corrected/baseline approximate ratios:

- Windows: `1.026`, `1.050`, `1.179`, `1.211`.
- WSL2: `1.072`, `1.076`, `1.044`, `1.239`.

For the first three workloads both revisions correctly **accept** their
matrices; observed overhead is about **2.6–17.9% on Windows** and
**4.4–7.6% on WSL2**. For the fourth workload the old source
**incorrectly rejects** a mathematically exact PSD matrix but the new
source **correctly accepts** it; a faster rejected result is **not**
semantically equivalent to a correct accepted result. The numeric latency
comparison is shown only to disclose the cost of the correctness repair.
No formal performance gate, native ARM64 execution, hosted CI result or
cross-hardware replication has been earned.
