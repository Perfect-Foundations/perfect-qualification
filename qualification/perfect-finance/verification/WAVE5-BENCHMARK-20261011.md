# Wave 5 Finance exact reducer dual-revision benchmark — October 11, 2026

Both immutable source revisions were compiled in the same optimized Rust 1.99 executable using the committed manifest and Cargo.lock at `qualification/perfect-finance/benchmarks/exact-aggregation`:
- Baseline heuristic: `Perfect-Foundations/perfect-finance@90be47248298b49089d49f954866ed916bc57ed5`
- Corrected exact bounded reducer: `Perfect-Foundations/perfect-finance@f716cb02c34dc79d51cef3540d7f0c0cc2ba930f` (implementation originally committed at 705b92b)

Run `cargo clippy --locked --offline --all-targets -- -D warnings` and `cargo run --release --locked --offline` from the harness directory. Rust 1.99, AMD Ryzen AI 9 365 CPU shared by Windows MSVC x86-64 and Ubuntu WSL2 Linux x86-64, 1,000 calls/workload, one warmup, median of five measured iterations. No input parsing inside timed loops; cloned flow-grouping, sorting and exact aggregation allocations are part of the timed call. No allocator peak-memory sampling was performed.

| Workload | Windows baseline ns | Windows corrected ns | WSL2 baseline ns | WSL2 corrected ns |
|---|---:|---:|---:|---:|
| 3 inputs, integer cancellation | 963.80 | 1250.80 | 762.40 | 983.97 |
| 4 inputs, fractional cancellation | 1024.20 | 1378.10 | 788.09 | 996.10 |
| 128 inputs, 63 sparse high-exponent cancellation pairs | 74701.10 | 82836.20 | 63320.95 | 74770.55 |
| 3 inputs, previously failing mixed-quantum cancellation | 1412.00 | 2073.70 | 1130.15 | 1641.88 |

For the first three cases, both revisions return the same exact amount/quantum. New/old ratios: Windows 1.298, 1.346, 1.109; WSL2 1.291, 1.264, 1.181. Correctness incurs workload-dependent measured CPU time. For the last case, the baseline **fails** with `CoefficientLimitExceeded{required:3,limit:2}`, while the corrected implementation returns exact `-98e-3`; the last latency numbers do not compare equivalent results.

Algorithmic resource ceiling: for each group, at most n<=max_flows validated input coefficients of at most D decimal digits, a temporary Decimal coefficient bound `D + ceil(log10(n)) + 1` digits and deterministic in-memory sorting O(n log n). Decimal arithmetic itself is handled by the pinned Perfect Decimal crate. This is a proved policy-level digit bound, **not** a measured maximum heap allocation or wall-time guarantee. Benchmark results are preliminary one-machine numbers, not formal release or performance acceptance.
