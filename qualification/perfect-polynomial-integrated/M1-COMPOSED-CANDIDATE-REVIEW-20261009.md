# Perfect Polynomial — combined conservative M1 candidate resource and contract review

**Review date:** 2026-10-09 (ET). **Source-exact candidate:** `34ffc473bf07960b66fdc8b0637bb0bf4aa8670e`, draft Polynomial PR #14. Parent PR #8 `d2365a0fc5af81877e64598301064ea2910e67f9`. Composed PR #12 metadata (`bda0e36762e82baaef62189a9dacfc698ba93555`) and PR #13 private tests (`643c13b15205ae94a583e64588c0c0b48a9a6bd2`). Changed paths compared with PR #8: `Cargo.toml` (metadata only), `src/dense.rs` (126 added lines wholly under cfg(test)), `docs/M1-BOUNDED-LCM-ASSURANCE.md`. **Production code and public behavior unchanged.** No commits merged into frozen PR #8 or protected branch. This file is a source-engineering review packet, **not authorized independent reviewer approval**.

## Exact source and direct dependency contract

`perfect-arithmetic` exact Git `1a54d3c7cbbae4e73325cc70fd2777a2427b1504`; `perfect-rational` exact Git `35a8e9cc629ee578fe7b624e2134929ce7eeff8a`; inherited `perfect-numeric` exact Git `19b6747cd852a47a694a020b97ba70b6b3ef259b`. All production dependency pins, features, Cargo.lock and `publish=false` unchanged. Existing registry packages `num-bigint 0.5.1`, `num-integer 0.1.47`, `num-traits 0.2.19`, `autocfg 1.5.1`; sources are exact Cargo.lock. No competing public Algebra hierarchy.

The public API remains `Polynomial<C>`, its explicit `IntegerPolynomial` and `RationalPolynomial` aliases, canonical constructors/coefficients/degree/zero, add/sub/mul/neg/evaluate/derivative; Q[x] `div_rem`, `monic`, `gcd`; Z[x] `content`, `primitive_part`, `div_exact`, `gcd`; `PolynomialError` with `DivisionByZero`, `NonExactDivision`, `NonIntegralQuotient`, `ArithmeticFailure`. No exposed generic Ring/Field trait. No public resource-refusal policy or configurable work budget is introduced.

## Source-exact contract findings (engineering, not formal acceptance)

| Approved requirement | Direct source check and executed evidence | Current disposition |
|---|---|---|
| REQ-CORE-0001 | `src/dense.rs` stores low-to-high Vec, `normalized` removes trailing zeros; zero empty and degree None; read-only coefficient slice. Source-exact independent consumer construction/canonical tests, Windows/Linux fingerprints. | **Engineering PASS within M1 scope**, independent signoff pending |
| REQ-CORE-0003 | `dense.rs` exact coefficient operations/Horner/derivative; `division.rs` Euclidean Q division, exact-only Z quotient, GCD; 79 structured + 500 seeded SymPy exact fixtures and 1,536 i128 arithmetic/evaluation cases, 192 GCD/factor invariants, all retained. | **Engineering PASS for tested M1 operations**; no universal proof |
| REQ-SEM-0001 | No float/tolerance/lossy conversion in production core; lower-layer Integer/Rational operations retain exactness and typed errors. Python Fraction/SymPy references and 512-bit private LCM tests. | **Engineering PASS within supported Z/Q domains** |
| REQ-SEM-0002 | Canonical empty zero, no degree for zero, structural equality, Z[x] gcd content/positive leading sign, Q[x] monic gcd; no M1 factor/root ordering API. | **Engineering PASS within exposed M1 semantics**, API stability review pending |
| REQ-ERR-0001 | `PolynomialError` typed division-by-zero, nonexact and nonintegral paths. Private 512-bit LCM/PRS guard `None` uses exact public fallback, never returns approximate success. Ordinary valid inputs do not use panic-driven control. | **Engineering PASS for exercised typed and guard paths**, OOM is not a typed guarantee |
| REQ-DEP-0001 | Direct dependencies only exact Arithmetic and Rational, transitive Numeric; strict Git revisions, no Algebra/CAS runtime; on-disk source and tree inspected. | **Engineering PASS for M1 production dependency boundary**, release supply-chain signoff pending |
| REQ-PORT-0001 | Windows 11 x86-64 and Ubuntu WSL2 x86-64 MSRV 1.99.0 all/no-default tests PASS on exact combined source. Four no-default targets are compile-only. | **PARTIAL:** physical Linux ARM64 runtime **NOT RUN**. Governing text calls Class-A Linux ARM64 *release-blocking*; its M1/milestone signoff interpretation remains for accepted authority, not automatically waived |
| REQ-SEC-0001 | `#![no_std]` + `#![forbid(unsafe_code)]` in production, no crate build.rs, mandatory FFI/native CAS/network/telemetry/auto-update; source and Cargo tree inspected. Transitive `num-traits` has a Rust build script via `autocfg`; test-only GlobalAlloc example uses unsafe outside production crate. | **Engineering PASS for stated M1 production boundary**, full transitive unsafe/supply-chain review still separate |

Later scope: REQ-CORE-0002/INT-0001/INT-0002, DET-0001, PERF-0001, VER-0001/0002, DOC-0001, PKG-0001 have later M2–M6 or release gates as in original approved requirement files. They are not silently imposed as new M1 mathematical blockers. Per approved `REQ-PORT-0001`, release-blocking Class-A hosts explicitly include Linux x86_64, Linux arm64 and Windows x86_64; physical Linux ARM64 remains the concrete unevidenced host.

## Bounded exact pathological resource experiment

All results in `resource-evidence/m1-pathologies-composed/`: immutable raw complete-operation CSV for **23 distinct operation/input shapes × 3 Windows + 3 Ubuntu WSL2 x86-64 release-process runs = 138 samples**, plus summary.csv, GNU time -v Linux process reports. Rust 1.99.0; `--offline --locked --release`. Identical deterministic inputs both hosts. Qualification-only `examples/resource_pathologies.rs` wraps System with unsafe GlobalAlloc **outside production**; it warms operations, resets counters, times and counts *only complete public operation including result drop*, with input construction and algebraic evaluation-at-one correctness checks outside the measurement interval. Counts reflect requested bytes, not allocator physical overhead/RSS/stack. No other threads during observed operation. Zero net incremental live requested bytes after each measured operation. Three-process means below are exploratory, not precision performance claims.

| Shape (source PR #14) | Windows mean | Linux mean | Allocations | Requested bytes | Peak incremental live bytes |
|---|---:|---:|---:|---:|---:|
| Dense Z[x], degree 256×256, 32-bit coefficients | ~4.27 ms | ~3.32 ms | 61,506 | 1,000,496 | 24,336 |
| Sparse Z[x], degree 512×512, 32-bit coefficients | ~2.7 µs | ~3.5 µs | 1 | 32,800 | 32,800 |
| Mixed-length sparse Z[x], degree 8×256, 128-bit | See CSV | See CSV | 9 | 8,768 | 8,648 |
| Rational uniform denominators, 256-bit | ~3.0 µs | ~2.5 µs | 31 | 2,008 | 1,256 |
| Rational distinct denominator LCM, 512-bit | ~6.1 µs | ~5.0 µs | 64 | 5,672 | 2,000 |
| Rational fallback, 513-bit LCM | ~5.8 µs | ~4.6 µs | 55 | 4,520 | 1,088 |
| Z[x] early PRS >4096-bit guard case | ~25 µs | ~10 µs | 118 | 67,088 | 14,768 |
| Z[x] late PRS continuation | ~0.98 ms | ~0.91 ms | 892 | 130,936 | 9,208 |

Coverage additionally includes dense degree 8/32/64/128, 1,024-bit coefficients, large cancellation, monomials, sparse length imbalance, rational LCM 256/511/512/513/768/1024, degree-33 guard refusal, 4098-bit original coefficient guard, and Q[x] late PRS continuation. Exact result properties verified separately with source-exact evaluation identities and known GCD reference fixtures. Ubuntu WSL2 *whole combined-executable process* RSS high-water in 3 runs: 2,752, 2,764, 2,772 KiB (GNU time kbytes); **not per-case RSS**.

### Resource conclusions and risk classifications

1. **Expected dense representation:** result has degree at most deg(a)+deg(b), with low-to-high vector capacity `len(a)+len(b)-1`. E.g. two sparse degree-512 inputs require 1,025 dense coefficient slots and a 32,800-byte output vector despite only a few nonzero coefficients. This is a known dense-first M1 design tradeoff (ADR-0001), not a correctness failure; a separate sparse representation is later scope.
2. **Expected quadratic work:** exact schoolbook dense multiplication visits O(mn) coefficient pairs (nonzero checks skip arithmetic). 257×257 dense 32-bit input produced 61,506 allocations / ~1 MB requested, while sparse 513×513 produces one output allocation. Large coefficients add separate arbitrary-precision integer/rational storage and normalization cost.
3. **Optimization bound vs crossover:** the bounded-LCM 512-bit limit is a *verified resource bound for denominator-clearing path*, NOT a measured universal best-performance cutoff. At 512→513 bits, fallback was modestly faster and made fewer allocations on this small fixture. Do not change threshold based on this comparison alone. The corrected uniform/uniform fixture explicitly asserts both operands have constant canonical denominator to exercise the intended different dispatch.
4. **PRS guards:** original source has max degree 32, input denominator LCM 512 bits and PRS intermediate coefficient 4096 bits. Early/late/degree/width guard cases preserve exact public results by bounded PRS or Euclidean fallback, with no sampled incorrect result. **These limits do not cap the later full field-Euclidean fallback**. Full worst-case GCD memory/stack guarantees are NOT established.
5. **Infallible API & length arithmetic:** general `mul` directly uses `vec![C::zero(); len_a+len_b-1]`, without a public fallible budget mechanism or local checked capacity expression. For actually materialized vectors of the substantial Integer/Rational wrapper types on tested 64-bit hosts, length overflow was neither reachable nor reproduced; **capacity failure/OOM remains process-level failure**, as already documented in `src/lib.rs`. No fabricated recoverability claim. A new configurable bounded API or change to infallible signature requires an explicit separate contract/design; adding an arbitrary degree cap would violate current exactness intent.
6. **Potential denial-of-service from untrusted inputs:** externally supplied enormous dense degrees, coefficient widths or huge Rational GCD fallback may require excessive time/memory. Callers processing untrusted input need an upstream operation/input budget. This is an **explicit resource-risk limitation**, not a newly demonstrated arithmetic defect.
7. **No corrective production source change justified** on the bounded measured cases. Source test-only PR #13 plus metadata PR #12 compose without conflicting definitions or dependency drift. Broader work-limit APIs/degree-sparse optimizations, heap/stack proofs, native ARM64 and release qualification remain separate work.

## M1 acceptance vs later release and review authority

The directly executed M1 semantic and source-boundary checks support an **engineering-acceptable candidate for independent M1 review**, not final accepted milestone. Reviewer authorization/approval was NOT obtained. Release-barring physical ARM64 runtime is not substituted with portable cross-compile or WSL2. Full Q0–Q6 cross-family acceptance and package-ready licensing remain separate gates.

`cargo package --offline --locked --allow-dirty` remains previously demonstrated blocked by unpublished registry `perfect-arithmetic`; it was **not retried** on unchanged preconditions. No owner license choice, publishing action, protected branch merge, Perfectπ write or milestone percentage update occurred.

Next review priority: independent reviewer adjudicates M1 semantic source and the Class-A ARM64 milestone/release boundary, while executable remaining work should target *bounded untrusted-input resource API contract* only if it is explicitly accepted as an M1 requirement, rather than silently changing existing infallible exact APIs.
