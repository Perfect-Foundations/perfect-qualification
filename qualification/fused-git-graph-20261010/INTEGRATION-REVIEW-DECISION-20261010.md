# Perfect Foundations exact integration — independent review decision packet
2026-10-10 · Experimental source only · Review order: Arithmetic → Rational → Polynomial → Qualification → milestone authority

## Immutable source under review

| Repository | Base / preceding evidence | Review source |
|---|---|---|
| Perfect Arithmetic PR #15 | Protected `1a54d3c7cbbae4e73325cc70fd2777a2427b1504` | `d99aef2a9ef230eb725adaef664b0cc0214dc4bd` |
| Perfect Rational PR #4 | Polynomial-compatible `35a8e9cc629ee578fe7b624e2134929ce7eeff8a` | `0afd08d27374f472127f3bf3cf5728e0a1170bd7` |
| Perfect Polynomial PR #18 | Previous Git integration `7633fc00e96cf37b8baa731011efaa95ada8b385`; protected M1 PR #14 `34ffc473bf07960b66fdc8b0637bb0bf4aa8670e` | `f28aa8751cd85526fe0ada6c94c3d49b38d8cd94` |
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
