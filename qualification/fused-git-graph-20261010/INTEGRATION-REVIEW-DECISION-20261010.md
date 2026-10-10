# Perfect Foundations exact integration — independent review decision packet
2026-10-10 · Experimental source only · Review order: Arithmetic → Rational → Polynomial → Qualification → milestone authority

## Immutable source under review

| Repository | Base / preceding evidence | Review source |
|---|---|---|
| Perfect Arithmetic PR #15 | Protected `1a54d3c7cbbae4e73325cc70fd2777a2427b1504` | `d99aef2a9ef230eb725adaef664b0cc0214dc4bd` |
| Perfect Rational PR #4 | Polynomial-compatible `35a8e9cc629ee578fe7b624e2134929ce7eeff8a` | `b45de7e7ad63b0850e1d3bbc67e38163b3c8fa7c` |
| Perfect Polynomial PR #18 | Previous Git integration `7633fc00e96cf37b8baa731011efaa95ada8b385`; protected M1 PR #14 `34ffc473bf07960b66fdc8b0637bb0bf4aa8670e` | `68f6698456c7aac494d12a460a113850dd9d5459` |
| Perfect Numeric | Approved retained dependency | `19b6747cd852a47a694a020b97ba70b6b3ef259b` |
| Perfect Qualification PR #10 | Prior frozen graph `1333caa009d0cf9790c6274d9f14b46dc7c43bbc` | This separate `qualification/fused-git-graph-20261010/` at current PR head; source manifest/lock frozen independently |

## Reviewer questions and mathematical invariants

**Arithmetic:** Approve/reject independently the three new experimental public methods `sub_mul_assign(&mut self,&Self,&Self)`, `scale_sub_mul_assign(&mut self,&Self,&Self,&Self)`, and `unsigned_bit_length(&self)->u64`. The first two satisfy exact `old - left*right` and `old*factor - left*right`; the third is exact nonnegative magnitude width with zero=0 and equal width for both signs. Borrowed backend magnitude is allocation-free; zero and signed unit paths and multiplication acceleration dispatch are preserved. Owner has not approved stability.

**Rational:** Review only the source-exact pinned Arithmetic dependency, unchanged canonical Rational arithmetic and error handling, `=0.1.0`, `default-features=false`, std propagation, unchanged Numeric.

**Polynomial:** Review the 11 width-only guard replacements, cached divisor-leading bit width, saturating conservative *uncancellable* product-width arithmetic, and post-cancellation retained-width guard on affected coefficients. Degree 32, LCM 512 bits, retained 4096 bits remain dispatch thresholds, not input restrictions; late PRS must resume from the last complete remainder pair. Existing fractional pseudo-division must preserve exact Q/R reconstruction and typed errors. No unsanctioned production dependency pins changed.

**Qualification:** Verify retained lock SHA and `verify_graph.py`: exactly one Git Arithmetic package through direct and Rational-transitive paths; zero duplicate packages, `[patch]` entries, or hidden local source substitution. Historical graph retained in `history/`. Exact independent Python Integer/Fraction and SymPy 1.14.0 oracle vectors now cover signed 128–16384-bit boundaries, 4096/4097-bit PRS acceptance/refusal, wide cancellation, repeated shared factors, Integer content, Rational monic GCD and exact division/error semantics.

## Verified engineering evidence and known tradeoffs

- Fresh Windows network Git and WSL2 native-filesystem source checkouts passed Rust 1.99.0 locked all-feature/no-default tests; full source Polynomial suite, separate independent Qualification consumer, warning-denied Clippy/rustdoc, rustfmt. WSL2 private Git objects came from verified Windows Git objects, **not independent Linux network authentication**.
- Compile-only targets: Windows wasm32v1-none, thumbv7em-none-eabihf, riscv64imac-unknown-none-elf; Linux WSL2 aarch64-unknown-linux-gnu. No native ARM64 execution claim.
- Source-exact Windows 3-run complete-operation allocations: degree64/64 Z exact division 4,163 unchanged; degree128/128 Q division **18,275→16,469**; degree40/20 Q GCD **832→708**; Z late-continuation GCD **817→595**; Q late-continuation GCD **861→639**. Requested bytes also decrease; sampled peak live bytes do not. The late PRS runtime does **not** show a material speedup. See `resource-evidence/DIRECT-WIDTH-INTEGRATION-20261010.md` and source-specific CSVs.
- No new mandatory Rust dependency, production unsafe, FFI, native build script, source patch or external runtime. Arbitrary-precision input can still demand unbounded memory; a universal OOM guarantee is neither specified nor asserted.
- Latest observed hosted Actions: Arithmetic `38039452645` nine and Rational `38039504402` seven failure jobs, **all zero steps / no assigned runner**. Exact service-side root cause unknown. No paid infrastructure or workflow suppression.

## Acceptance disposition

**Engineering candidate:** exact mathematics, Git-pinned single dependency identity, standalone build, cross-OS independent test execution, measured selective resource gains: **ready to submit for independent source/API review**.

**Not accepted:** independent reviewer signoff, controlling Polynomial M1 and M2 milestone approval, applicable family Q0–Q6 qualification, physical native ARM64 execution, executable hosted CI/approved equivalent, owner license and publication decisions. Source lifecycle remains **implementation**; do not alter `project-status.toml` percentages or release state without the controlling gate's evidence.

Review in order: (1) Arithmetic three contracts and backend semantics; (2) Rational exact compatibility and source pin; (3) Polynomial guard/cancellation/GCD behavior and resource tradeoffs; (4) independent Qualification source and retained lockfiles; (5) authorized milestone and release authorities. None is self-approved by the implementer.

## Fresh RustSec advisory scan (same pinned lockfiles)

On 2026-10-10, `cargo-audit-audit 0.22.2` refreshed its RustSec advisory database (1,296 advisories) and completed with **exit 0 and zero reported vulnerabilities** against: Arithmetic `Cargo.lock` (11 dependencies), Rational `Cargo.lock` (9), Polynomial `Cargo.lock` (8), and the separate pinned Qualification consumer `Cargo.lock` (9). These are direct source-exact locked-dependency checks, **not** proof that private Git crate logic is secure, that future advisories will remain absent, that license approval is complete, or that arbitrary-precision resource exhaustion is impossible. No dependencies were added or source pins changed by this audit.

## Post-review engineering findings — same mathematical graph, CI-pin repair and boundary assurance

At the next evidence checkpoint, the experimental Rational PR #4 source is `b45de7e7ad63b0850e1d3bbc67e38163b3c8fa7c` and Polynomial PR #18 source is `68f6698456c7aac494d12a460a113850dd9d5459`; Arithmetic `d99aef2a9ef230eb725adaef664b0cc0214dc4bd` and Numeric `19b6747cd852a47a694a020b97ba70b6b3ef259b` remain unchanged. The preceding graph is retained separately as `history/source-exact-ae9439c.json` and `history/Cargo-ae9439c.lock`. Earlier performance CSVs remain evidence **only for the exact historical source revisions under which they were collected**; they are not fresh measurements on this updated CI/coverage-only source.

**Confirmed defect — Rational experimental CI metadata:** Earlier Rational CI `PERFECT_ARITHMETIC_REV`, source-snapshot composite action and standalone benchmark manifest/lock still used protected old Arithmetic `1a54d3c7...`, contradicting experimental Cargo `d99aef2...`. This would have failed the workflow's own pin assertion **if** the GitHub runner started. Fixed all explicit source references in Rational commit `b45de7e7...` and introduced `scripts/verify_ci_pins.py`, invoked by the Quality/MSRV workflow. The verifier and both Rational feature modes and benchmark Cargo tests pass on Windows/WSL2. This is independent of the still-unresolved *zero-runner-step* GitHub failure.

**Confirmed assurance gap — post-subtraction exact width:** Two narrowly selected real mutations to PRS `> MAX_INTERMEDIATE_BITS` comparisons originally yielded one caught and one surviving mutant. The mutation at the *affected, post-subtraction* coefficient check (replacing `> 4096` with `>= 4096`) survived. New Rust regression proves `prem(x^4, x^3-2^4095)` has primitive remainder `x` and must accept a 4096-bit affected coefficient. Rerun: **two caught / zero survived**. Polynomial commit `68f66984...` contains the test and its method/evidence, and also pins the corrected Rational SHA; it does **not** change the verified production PRS recurrence. The full Polynomial all/no-default-feature matrix passed Windows and WSL2.

**Unchanged scope/authority:** no formal external reviewer approved Arithmetic APIs; no milestone status or percentage was promoted. RustSec audit and historical performance evidence remain source-specific. Native physical ARM64 remains unverified; compile-only or WSL x86-64 is not a substitute. Hosted Actions status remains failed at pre-execution runner assignment.

## Focused Arithmetic API mutation review (independent scope)

On exact experimental Arithmetic source `d99aef2a9ef230eb725adaef664b0cc0214dc4bd`, `cargo-mutants 27.1.0` selected **six** precise mutations in `src/integer.rs`: both `unsigned_bit_length` constant-return replacements (0, 1); the zero-product early-exit `|| → &&` in `sub_mul_assign`; and three scale-factor `== → !=` transformations in `scale_sub_mul_assign`. Baseline passed; result **five caught, one missed**. The missed `|| → &&` variant is **mathematically equivalent** for any zero factor because the product remains exactly zero and subtracting it leaves the accumulator unchanged. Its significance is that an explicit zero-product fast path may execute unnecessary work, not that an incorrect Integer result would escape tests. This selected mutant does not justify a production algorithm change or a false universal mutation score; retaining the fast path is still intentional. Scope excludes other Arithmetic operations and optional backend throughput tuning.

## Superseding current source snapshot

The earlier source table is historical. Current Polynomial is `36c052130633804c0b7b2d5ecbfe1f7d8dec4358` (traceability-only update after `68f6698`). Current Qualification Cargo.lock/source-exact.json take precedence. Previous `54d8dc9` graph is retained in `history/`. See `M2-COVERAGE-20261010.md` for eight oracle tests and four identical 37-record cross-host streams. No formal gate promoted.

## Latest source-exact engineering reviewer handoff — PRS guard closure

The preceding snapshots are retained as history. **Current mathematical source**: Polynomial `30d9d02b8ee681ffeee2a07d4d5bb531461602a1`, Arithmetic `d99aef2a9ef230eb725adaef664b0cc0214dc4bd`, Rational `b45de7e7ad63b0850e1d3bbc67e38163b3c8fa7c`, Numeric `19b6747cd852a47a694a020b97ba70b6b3ef259b`. Current Qualification source-exact graph and Cargo.lock supersede previous `216ff26` graph, retained in `history/`.

**Review issue 1, real resource-contract defect:** `primitive_prs.rs::pseudo_remainder` would retain a 4097-bit coefficient when `a0=2^2048-1` is multiplied by `b3=2^2049-1`; lower-bound precheck incorrectly permitted it. After exact multiplication of an **uncancellable** coefficient, add allocation-free `unsigned_bit_length()>4096` refusal. Preserve exact affected-coefficient cancellation and accepted 4096-bit result. Reproducer/test and mathematical SymPy oracle accompany source. Public Z/Q GCD stays exactly one on primitive test pair via Q field fallback; original PRS late continuation tests pass. Three targeted post-check mutants caught.

**Review issue 2, algorithm-path evidence gap:** Degree-pair-only test diagnostic counter replaced with `#[cfg(test)]` thread-local event flags. Direct tests prove selected fraction-free Q division and exact field fallback for step count, divisor degree, denominator LCM and growth preflight boundaries, and verify Q/R on each fixture. Source production Q/R recurrence unchanged; warning-denied Clippy passes.

**Review issue 3, cross-host qualification:** Previous V1 corpus and exact source/fingerprint retained; new independent **V2** corpus exercises the new boundaries, wide signed typed division errors, negative Q divisor lead, selected/fallback Q steps and dense 63–65 multiplication. Windows/WSL2 with both feature configurations each produce **4,165 identical ASCII bytes**, SHA-256 `87c8a080eeedca85482b486e47c03226a7266a3d3898efeb6ac5fcc3fd27b3fc`, all independently verified through Python Fraction/SymPy. Ten fixed Rust Qualification tests (old eight plus two boundary contracts) pass both modes on both hosts. See `M2-COVERAGE-20261010.md` for exact reproduction and remaining gaps.

**Approval remains absent.** An independent reviewer must examine API/contract and proof obligations; hosting CI zero-step failures, physical ARM64 runtime, formal M1/M2 gates, licensing/release, complete mutation/property/security testing remain open. No Perfectπ changes or protected source merges.
