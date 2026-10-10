# Source-matched three-generation Rational GCD measurement — 2026-10-10

Three immutable source candidates compared using identical copies of `examples/division_targeted.rs --gcd-selected-only` with Rust 1.99.0 `--release --locked` on the same Windows x86-64 machine, three alternating independent process runs each. Complete public `Q::gcd` was measured; data are allocator-requested bytes (not RSS or formal worst-case bounds).

- Conservative M1: Polynomial PR #14, `34ffc473bf07960b66fdc8b0637bb0bf4aa8670e`, protected Arithmetic dependency `1a54d3c7cbbae4e73325cc70fd2777a2427b1504`.
- Existing fraction-free: Polynomial PR #17, `6bf6b51563a134591df04557fe4bab4b4faaaa5e`, same protected Arithmetic dependency.
- Fused candidate: Polynomial PR #18 `ce89bad2b80a2c887f91a912a36a78c80303ba1c`, separate experimental Arithmetic `73c3e6f30278556cda090b8d5cce2ed2d2a422c1` resolved through the existing test-only local patch.

Fixture: `A=(B*B)+1` with monic degree-20 dense `B`, 96-bit lower coefficient magnitudes. `deg(A)=40`, `deg(B)=20`, exact canonical `gcd(A,B)=1`. All benchmark programs assert expected exact GCD before sampling. A dedicated `#[cfg(test)]` assertion in fused source proves the degree-40/20 Euclidean stage entered the bounded fraction-free path after degree-40 primitive PRS refusal. Do **not** generalize that dispatch to other GCD families.

| Metric | Conservative M1 | Fraction-free PR #17 | Fused PR #18 |
|---|---:|---:|---:|
| Mean runtime (µs, 3 runs) | 353.33 | 75.73 | 60.07 |
| Allocation calls | 4,768 | 1,253 | 832 |
| Requested bytes | 134,440 | 49,200 | 36,064 |
| Peak incremental live requested bytes | **9,976** | 10,968 | 10,968 |
| Reallocation calls | 0 | 0 | 0 |

Fused versus conservative: 82.55% fewer allocations, 73.17% fewer requested bytes, 83.0% lower mean runtime **on this fixture**, but ~9.94% higher measured peak incremental live requested bytes. Fused versus fraction-free: 33.60% fewer allocations, 26.70% fewer requested bytes, 20.7% lower mean runtime; the measured peak is unchanged. Relative differences are specific to this micro-workflow and the tested toolchain/platform.

Evidence: `m1-gcd-win-[1-3].csv`, `fractionfree-gcd-win-[1-3].csv`, `fused-gcd-win-[1-3].csv`.
Retain the distinct pinned fixture and its standalone mathematical exact-reference tests. These measurements alone do not close formal M2, safety/resource-bound, native ARM64, hosted CI, or release qualification.
