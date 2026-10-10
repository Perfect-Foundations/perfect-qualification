# M1 bounded-LCM six-mutant resolution — 2026-10-09

## Exact source and scope

Frozen conservative source: Polynomial PR #8 at d2365a0fc5af81877e64598301064ea2910e67f9. Focused **test-only** source PR #13 at 643c13b15205ae94a583e64588c0c0b48a9a6bd2 (branch qual-poly-lcm-invariants-20261009). Only a cfg(test) module in src/dense.rs and docs/M1-BOUNDED-LCM-ASSURANCE.md were added. Production operations, dependencies, features, MSRV, license and publish=false are **unchanged**.

Mutation tool: cargo-mutants 27.1.0; focused 11-mutant selection (src/dense.rs / bounded_lcm), jobs=2, timeout=60 seconds, original and test-only candidate both built offline/locked. Structured mutant replay saved in this directory.

## Six originally surviving mutants — precise classification

| Mutation (src/dense.rs) | Mathematical meaning / reached? | Public result vs optimized path | New direct detector | Actual replay |
|---|---|---|---|---|
| bounded_lcm -> None | Rejects every admissible common denominator, not mathematical invalidity | Public generic Rational fallback remains exact but bypasses optimization | exact_lcm_and_scaled_integer_coefficients / width test | CAUGHT |
| delete ! from if !remainder.is_zero() | gcd(L,d) divides L by definition, so the original nonzero-remainder defensive branch cannot be taken on valid inputs; negation rejects normal zero remainders | Public fallback can hide optimization denial, but private successful LCM must fail | exact_lcm_and_scaled_integer_coefficients | CAUGHT |
| width guard > -> == | Rejects exactly 512, erroneously accepts widths 513+ | Violates **declared work bound**, not necessarily exact output; possible excess coefficient/denominator growth | exact_512_bit_width_acceptance_and_strict_decline | CAUGHT |
| width guard > -> < | Rejects small eligible widths and may accept >512 | Both unnecessary fallback and **work bound violation** | exact_512_bit_width_acceptance_and_strict_decline | CAUGHT |
| width guard > -> >= | Rejects exactly 512, valid upper bound case | Unnecessary fallback; no inherent wrong public result | exact_512_bit_width_acceptance_and_strict_decline | CAUGHT |
| mul_bounded_lcm -> None | Rejects every optimized multiplication call | Public generic fallback exact but optimization unexercised | optimized_convolution_matches_independent_fraction_oracle | CAUGHT |

Original already caught: bounded_lcm -> Some(Default::default()), which supplies zero, not an LCM. Four remaining compiler-unviable synthetic return-value variants for mul_bounded_lcm cannot be meaningfully classified as survived/equivalent. **No mutant was established equivalent**; the defensive original failure branch is mathematically unreachable, but the *inverted* mutant is reachable and now killed.

**Previous focused campaign:** 11 total / 1 caught / 6 missed / 4 unviable. **New actual campaign:** 11 total / 7 caught / **0 missed** / 4 unviable / 0 timeouts. The unmutated baseline succeeded. Read outcomes.json, mutants.json, caught.txt, missed.txt, unviable.txt and timeout.txt in this folder. A separate generated working tree (mutants.out/) and the older original campaign files are preserved locally, not deleted or overwritten. Changes to formatting and a Clippy-preferred slice expression following the new mutation replay did not change the original production algorithm or the tested private method behavior; final test-only review SHA was independently compiled after these adjustments.

## New private mathematical tests

- Exact LCM of 1/2, -2/3, canonical 0, 5/7 = 42 with scaled signed integer coefficients [21,-28,0,30]; exact LCM of 1/6,-1/15,7/10 = 30 with [5,-2,21]; verified divisibility and refusal for invalid nonmultiple 41.
- Actual LCM bit lengths 1,64,128,256,511,512 are ACCEPTED; 513,514,768 are DECLINED, including an accumulative >512-bit LCM from a 512-bit power-of-two and coprime denominator 3.
- Private optimized four-by-four Rational multiplication returns Some(correct result), not just public fallback equivalence; Python fractions.Fraction reference independently produces [-3/10,28/45,-5/108,-19/21,32/63,5/14,-10/49]. Reproducible independently in scripts/check-bounded-lcm-fraction-oracle.py.
- Canonical zero product and sparse inputs tested. When the private helper refuses a 513-bit denominator, public multiplication equals explicitly constructed (x^3+1/d)(x^3+1), preserving exact fallback and coefficient canonicality.

**Results:** all four private tests PASS Windows 11 Rust 1.99.0 and Ubuntu WSL2 Linux x86-64 Rust 1.99.0. Windows full all-features/no-default tests, rustfmt, Clippy with -D warnings and rustdoc with RUSTDOCFLAGS=-D warnings PASS. Linux full all/no-default suites PASS. Production code is identical to original PR #8. Existing portable-target compile-only evidence for that unchanged code remains scoped to the previously verified source. No native physical ARM64 claim.

## Preserved untracked independent integer oracle

Existing untracked tests/seeded_integer_oracle.rs was inspected rather than deleted or overwritten. Original bytes preserved in an isolated local preservation folder before applying rustfmt. Two public-API tests PASSED on Windows and WSL2:
- 1,536 seeded, bounded independent i128 cases comparing canonical degree, add/sub/mul, derivative and Horner evaluation.
- 192 seeded exact-factor division and GCD symmetry/divisibility cases (GCD uses algebraic invariants, not independently calculated GCD coefficients).

This source-exact external consumer still pins frozen PR #8, so it is independent of the new private source-test branch. It supplements but does not replace 579 previously retained SymPy input pairs or 10,000 previous native libFuzzer executions.

## Focused source/security and dependency review

Scope: source-boundary and Cargo metadata/tree inspection for unchanged PR #8 core. src/lib.rs forbids unsafe; production src/ has no visible unsafe operations, native FFI, network request, user-facing auto-update, or private build.rs. Existing public Rational div_rem and Integer div_exact have typed zero/nonexact failure paths, preserving domain distinction; no change was necessary. Four upstream Perfect-family crates remain exact Git revisions (Polynomial is root plus Arithmetic, Rational, Numeric), and four crates.io packages remain autocfg 1.5.1, num-traits 0.2.19, num-integer 0.1.47, num-bigint 0.5.1. Cargo metadata reports the four registry dependencies licensed MIT OR Apache-2.0 / Apache-2.0 OR MIT. **Perfect crates still lack owner-decided license metadata.** Transitive num-traits has a Rust custom-build step and autocfg build dependency; Polynomial itself has no custom build target or mandatory external CAS/native runtime. This is not a full line-by-line transitive unsafe audit.

Potential remaining resource limitation (NOT new defect): the generic exact dense mul path allocates an output of length len(a)+len(b)-1 without a caller-configurable work limit. Extremely high-degree/adversarial inputs can exhaust memory; Rust OOM is not promised recoverable. Bounded-LCM caps denominator work but not entire polynomial degree/output heap or stack. Do not claim formal denial-of-service/resource-hardening gate completed.

Previously cached advisory scan (8 locked packages, 0 findings) and unresolved private license-policy audit are retained unchanged; no registry packaging retried without changed prerequisites.

## Formal gate decision

REQ-CORE-0003/REQ-SEM-0001/REQ-ERR-0001/REQ-SEC-0001 receive **new direct private mathematical and mutation assurance evidence**, while Q4's scoped six-mutant observation closes (all viable scoped mutants caught). **Formal M1 acceptance remains unapproved**; this does not signify universal mutation adequacy, native ARM64 execution, full Q0-Q6, stack/peak-heap bounds, or package-release readiness. Owner-deferred license/publication issues are separate from mathematical M1 semantics. No project-status.toml promotion.
