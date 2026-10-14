# Git-pinned source-exact performance replay, Windows 2026-10-10

Source revisions: Arithmetic `73c3e6f30278556cda090b8d5cce2ed2d2a422c1`, Rational `53816615d28d47cbc8210efb239f05f632ecd854`, Polynomial `7633fc00e96cf37b8baa731011efaa95ada8b385`; Numeric `19b6747cd852a47a694a020b97ba70b6b3ef259b`. No path override. Same fully instrumented release `division_targeted.rs` example. Windows x86-64 Rust 1.99.0; 3 independent process invocations per mode. Report **requested heap bytes**, not RSS or formal worst-case resource bounds.

| Complete operation | Alloc calls | Requested bytes | Peak incremental live bytes | Mean latency |
|---|---:|---:|---:|---:|
| Integer exact division, dense degree 64/64, 128-bit | 4,163 | 110,600 | 9,304 | 295.67 µs |
| Rational division, dense degree 128/128, nonmonic | 18,275 | 1,019,312 | 44,640 | 1.43893 ms |
| Rational GCD, dense coprime degree 40/20 | 832 | 36,064 | 10,968 | 56.87 µs |
| Z[x] complete multiply→divide degree 128 | 9 | 32,960 | 20,576 | 0.4975 ms |
| Q[x] complete multiply→divide degree 128 | 19,115 | 1,144,040 | 59,200 | 1.8048 ms |
| Z[x] multiply→GCD→divide small degree | 13 | 3,192 | 2,128 | sample-specific |
| Q[x] multiply→GCD→divide small degree | 8 | 2,464 | 1,624 | 6.27 µs |

The exact Integer division 64/64 resource counts equal the previous path-patched measurement **4,163/110,600**, versus frozen preceding control **8,385/211,880**, and the new full-Git source maintains the earlier 18,275/1,019,312 large Rational division and 832/36,064 selected Rational GCD figures. Full-source graph did not erase those measured allocation improvements. Small GCD composition is not claimed to improve. Different process samples can shift microsecond latency, so previous run means cannot establish an exact source-change speed ratio.

Additional separate primitive PRS comparison (Polynomial source `bench-evidence/prs-*.csv`) uses the **same** unpatched Arithmetic/Rational dependency graph: late guard-resume Z[x] GCD **889→817** calls and **130,456→119,432** requested bytes, while latency and small PRS samples provide no general speedup proof. See `docs/FUSED-PRIMITIVE-PRS-20261010.md`.

Evidence here: `integer-exact-win-{1,2,3}.csv`, `division-win-{1,2,3}.csv`, `gcd-selected-win-{1,2,3}.csv`, `workflows-win-{1,2,3}.csv`. Each row is a real complete public operation, not an extrapolated operation-count model. Qualification approval, native ARM64 runtime, hosted CI and publication remain separate gates.
