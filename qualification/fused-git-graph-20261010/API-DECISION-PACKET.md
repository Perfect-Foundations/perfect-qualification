# Exact mutable Integer API — independent review decision packet

**Owning source:** Perfect Arithmetic draft PR #15, Git SHA `73c3e6f30278556cda090b8d5cce2ed2d2a422c1`. This packet is qualification evidence, **not** independent reviewer approval.

## Contracts and consumers

| Public method | Exact effect | Verified existing consumer |
|---|---|---|
| `sub_mul_assign(&mut self, &left, &right)` | `self_new = self_old - left*right` | `IntegerPolynomial::div_exact` |
| `scale_sub_mul_assign(&mut self, &factor, &left, &right)` | `self_new = self_old*factor - left*right` | `RationalPolynomial` fraction-free division and primitive `Z[x]` PRS |

Both methods are exact signed arbitrary-precision transformations; no rounding, saturating, fixed-width truncation, panic-based normal errors or hidden mutating state. `&mut self` plus borrowed input operands enforces normal Rust aliasing: callers cannot pass the mutable receiver itself as an immutable operand in safe Rust. No unsafe production code, FFI, global mutation, or new required dependency. The canonical BigInt zero and signed-unit handling are tested directly at 128, 512, 2048, 8192 and 16384 bits.

`sub_mul_assign` bypasses constructing a product when either factor is zero or ±1. `scale_sub_mul_assign` similarly handles 0/±1 scale exactly; wide generic nonunit multiplies preserve the existing dispatch predicate for accelerated x86-64 multiplication. Nonaccelerated scale multiplies use the established mutable backend operation. Exact cancellation to zero and wide borrow/carry transitions are covered by fixed reference expectations. The owner remains `perfect-arithmetic`, not duplicated in specialist crates.

## Evidence

- Owner source: `tests/integer_fused.rs`, independent Python decimal expected values and dispatcher-boundary tests.
- Integer division: source-exact prior/new degree64/64 counts **8,385→4,163** allocations and **211,880→110,600** requested bytes, no error contract change.
- Rational division: dense degree128/128 **18,275** allocations / **1,019,312** requested bytes retained after Git graph integration.
- Primitive PRS: complete late-resume Z[x] GCD **889→817** allocations; no latency improvement proven.
- Pure Git-pinned downstream Rational and Polynomial and separate Qualification consumer pass Windows/WSL2 Rust 1.99.0 all/no-default tests; direct/mutual transitive source graph has exactly one Arithmetic package. Windows wasm32v1-none compile-only passes.

## Proposed decision for authorized reviewer

**Keep both method signatures as experimental candidates**, because independent downstream arithmetic operations require their distinct exact semantics. Do not stabilize or merge automatically. Acceptance conditions remain: reviewer scrutinizes API naming and overflow/alias semantics, confirms dispatch behavior across supported backends and MSRV targets, considers benchmark tradeoffs where PRS latency does not improve, and approves version/release gates. Hosted GitHub CI is still blocked before runner assignment. No native physical ARM64 evidence, formal milestone acceptance or crates.io publication is claimed.
