# Exact mutable Integer API — independent review decision packet

**Owning source:** Perfect Arithmetic draft PR #15, Git SHA `d99aef2a9ef230eb725adaef664b0cc0214dc4bd`. This packet is qualification evidence, **not** independent reviewer approval.

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

## Additional experimental API — direct unsigned magnitude width

Source owner Arithmetic PR #15 `d99aef2a9ef230eb725adaef664b0cc0214dc4bd` adds `Integer::unsigned_bit_length(&self) -> u64`. Exact signless magnitude width: zero=0, ±(2^n)=n+1, ±(2^n-1)=n. It reads the borrowed backend `BigInt::magnitude().bits()` and neither clones the backend's limb storage nor constructs a `Natural`. Independent power-of-two and sign vectors through 16384 bits passed in Arithmetic and the Qualification-owned consumer, in both Rust 1.99.0 feature modes on Windows and Ubuntu WSL2. The former `unsigned_abs().bit_length()` caused one heap allocation per 128–16384-bit inspection; the direct method produced zero, with source-exact allocator measurements committed in Arithmetic `bench-evidence/integer-width-win-*.csv`.

**New downstream consumers:** 11 width-only uses in `perfect-polynomial/src/{primitive_prs,division}.rs` on `f28aa8751cd85526fe0ada6c94c3d49b38d8cd94`. Changed PRS width calculation caches a constant divisor-leading width and uses saturating arithmetic in a conservative bound; its *affected* coefficient guard remains after exact subtraction to preserve cancellation. Qualification explicitly tests exact 4096-bit accepted and 4097-bit refused unaffected-width paths through source tests.

**Review questions:** evaluate the public name versus `Natural::bit_length` convention; document whether zero and negative magnitude width are intuitive; verify backend independence if BigInt changes; check backend `bits()` contract and source safety; consider whether this small exact reusable capability should stabilize independently of the two fused mutation APIs. No formal approval has been claimed.
