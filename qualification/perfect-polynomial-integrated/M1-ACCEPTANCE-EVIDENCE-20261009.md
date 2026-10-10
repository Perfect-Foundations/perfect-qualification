# Polynomial M1 acceptance evidence — 2026-10-09

Source: PR #8 d2365a0fc5af81877e64598301064ea2910e67f9, no source change from qualification consumer. Rust 1.99.0, pinned Arithmetic 1a54d3c7cbbae4e73325cc70fd2777a2427b1504, Rational 35a8e9cc629ee578fe7b624e2134929ce7eeff8a, inherited Numeric 19b6747cd852a47a694a020b97ba70b6b3ef259b.

## Private packaging: BLOCKED, NOT PASS

Exact-source cargo package --offline --locked --allow-dirty and --no-verify both exit 101. Cargo cannot resolve unpublished registry perfect-arithmetic v0.1.0 when preparing the registry-compatible package. The same failure persists after permitted metadata fixes on a distinct isolated source branch. No crate archive was generated or verified. Cargo package --list inventories files only; do not call this packaging PASS. All raw failure logs retained in package-evidence. Neither license, production pins nor publish=false was changed. The isolated metadata branch adds repository, readme, future docs.rs URL, categories and keywords without changing source semantics.

## Independent robustness and mutation

Independent SymPy generator with seed 20261009 produced 500 cases degree 0–10, dense/sparse, signed/nonprimitive content, shared linear and nonmonic factors, zero operands. The source-pinned qualification consumer checks GCD, symmetry, add, multiply, derivative, exact divisibility against independently generated coefficients. PASS Windows and Ubuntu WSL2 Linux Rust 1.99.0 both all/default and no-default modes.

Installed cargo-mutants executed on bounded denominator LCM implementation: **11 total, 1 caught, 6 missed, 4 unviable, zero reported timeouts**. Structured outputs in mutation-evidence. Misses mainly disable or alter denominator-clearing optimization while an exact fallback preserves the result; these demonstrate weak optimization-path observability, not verified wrong mathematics. Mutation qualification PARTIAL.

## Supply-chain

cargo audit --no-fetch --stale --json returned exit 0, **0 findings across 8 locked dependencies**, using local advisory database SHA 7eebec69c352c7191b1f13eb95dd510eeca5d1de from 2026-10-09. This is a cached advisory snapshot, not a guarantee about future discoveries. Report: package-evidence/cargo-audit.json.

cargo deny check licenses advisories exited 4: **advisories OK, licenses FAILED**, including unpublished Perfect Polynomial/Rational crates lacking selected license expressions. Raw results: package-evidence/cargo-deny-licenses-advisories.txt. License selection remains owner-deferred; no license was invented. A complete transitive safety/FFI/build-script/license review remains pending.

## Benchmark-only incremental live heap accounting

The nonproduction qualification example resource_census.rs uses System GlobalAlloc instrumentation (unsafe only in the example, not the production crate). Inputs are constructed outside the measurement; each public call is warmed, executed, black-boxed and dropped within the measured interval. Three Windows and three Ubuntu WSL2 Linux release processes produced matching rows (resource-evidence folder).

| Operation | Alloc count | Total requested bytes | Peak incremental live requested bytes |
|---|---:|---:|---:|
| Integer shared GCD | 8 | 1600 | 1056 |
| Rational shared GCD | 11 | 2400 | 1056 |
| Rational multiplication | 1 | 392 | 392 |
| Rational div/rem | 2 | 560 | 560 |
| Integer exact division | 2 | 896 | 896 |
| Integer coprime GCD | 8 | 1344 | 928 |

Each case has zero net live delta after result drop. This is incremental requested allocator accounting, NOT process RSS, physical malloc footprint, peak heap including persistent operands, stack bound or an OOM guarantee.

## Disposition

These are independent, source-exact increments toward M1 semantic and robustness acceptance, NOT formal qualification completion. M1-GAP-MATRIX.md distinguishes M1 blockers from M4/Q6 and owner-deferred public-release obligations. Physical native Linux ARM64, expanded robustness/mutation, full supply-chain policy review, successful registry-backed private package verification and Q6 acceptance remain incomplete. Authoritative project-status.toml percentages unchanged. No protected PR merged, no registry publication, no PerfectPi edits.

## Cross-host semantic fingerprint (Q3 subset)

The independent consumer example semantic_fingerprint.rs hashes canonical Polynomial coefficient sequences and public gcd/add/mul/derivative outputs using the versioned canonical Integer Hash encoding and explicit length-framed FNV-1a. It covers **579 independently constructed input pairs** (79 adversarial + 500 seeded). Rust 1.99.0 offline/locked release execution produced identical fingerprints on Windows x86-64 and Ubuntu WSL2 Linux x86-64:

    source=d2365a0fc5af81877e64598301064ea2910e67f9 cases=579 fnv64=435349aa9a62498a

This is an explicit sampled cross-host deterministic semantic fingerprint, not a collision-free serialization proof or universal cross-platform determinism claim. It adds **partial Q3 evidence**; native ARM64 runtime is still untested.

## Real bounded libFuzzer increment (Q4 subset)

In the separate nonproduction qualification fuzz/Cargo.toml, source-exact PR #8 was exercised with Linux nightly cargo 1.101.0-nightly and libfuzzer-sys 0.4.13. Two native libFuzzer runs each completed 5,000 byte-generated inputs successfully with no crashes. The second reproducibly seeded run (20261009) archived full log and a 327-file corpus under fuzz-evidence; final cov=1124 and ft=4579. Harness checks public Polynomial exact ring laws, GCD symmetry/divisibility, and Rational monic invariants for coefficients [-11,11], degrees <=8. It does not prove full sanitizer/Miri/large-input robustness or eliminate the six missed bounded-LCM mutants. Source and production dependency pins unchanged.

## Additional bounded-LCM targeted assurance — source branch PR #13

Source PR #13 commit 643c13b15205ae94a583e64588c0c0b48a9a6bd2 adds cfg(test)-only direct denominator LCM/512-bit guard/fallback tests and proof notes, leaving production semantics unchanged. The real focused 11-mutant cargo-mutants replay progressed from 1 caught/6 missed/4 unviable to **7 caught/0 missed/4 unviable**, with zero timeouts. See mutation-evidence/lcm-targeted-20261009/README.md and structured data. The six previously missed mutants included one defensive exact-division check inversion, three width-bound conditions, and both optimized-path disabling mutations. Invalid width acceptance may violate work bounds even while results remain exact.

A previously preserved untracked public-test file tests/seeded_integer_oracle.rs was inspected, its original bytes preserved separately, formatted and run against exact frozen PR #8 on Windows and Ubuntu WSL2: 1,536 independent bounded i128 arithmetic/evaluation cases and 192 exact-factor/GCD divisibility cases PASS. A reproducible independent fractions.Fraction script for the new private convolution fixture is scripts/check-bounded-lcm-fraction-oracle.py. The old mutations.out working tree remains untouched/untracked. No full M1 signoff or packaging-state change is implied.

## Current composed M1 candidate — source-exact review superseding historical PR #8 consumer pin

**Current tested consumer source:** Polynomial draft PR #14 `34ffc473bf07960b66fdc8b0637bb0bf4aa8670e`, composed cleanly from PR #8 frozen production, PR #12 metadata and PR #13 cfg(test) assurance. Combined source uses exactly the same production mathematical implementation as historical PR #8; consumer Cargo.toml and Cargo.lock have been deliberately retargeted from frozen PR #8 to combined PR #14. Test-only `fuzz/Cargo.toml` remains historically pinned to PR #8 for provenance.

**Direct same-revision verification:** Windows and WSL2 Linux Rust 1.99.0 all/default and no-default test suites pass, and the 579-case semantic fingerprint was rerun against this exact combined SHA on both hosts: `435349aa9a62498a`. Warning-denied Clippy and Rustdoc pass. Four Rust 1.99.0 `no_std` target checks pass on the combined source (AArch64 Linux, WebAssembly, Thumb, RISC-V), **compile-only**.

**Bounded resource review:** 23 exact operation/input shapes × 3 Windows + 3 Linux release-process runs = 138 samples with independent mathematical checks outside the measurement window, documented in `M1-COMPOSED-CANDIDATE-REVIEW-20261009.md` and `resource-evidence/m1-pathologies-composed/`. No material arithmetic defect reproduced; expected dense output and quadratic exact work persist, private Rational LCM bound works as designed, and PRS guard fallbacks remain exact on tested cases. Infallible dense API lacks a recoverable OOM guarantee / global untrusted-input work budget; this is an explicit resource contract limitation requiring separate API authorization if changed, not a silent M1 correctness failure.

**Formal M1 finding:** Engineering evidence supports REQ-CORE-0001/0003, SEM-0001/0002, ERR-0001, DEP-0001 and SEC-0001 within sampled and reviewed M1 behavior. REQ-PORT-0001 has Linux/Windows native x86-64 tests but lacks release-blocking Class-A physical Linux ARM64 execution; its exact M1 milestone signoff treatment must be decided by the authorized reviewer. No independent approval obtained. Packaging at M4/Q6 remains blocked by unpublished registry dependencies; license/publication are owner-deferred. No overall or formal qualification percentage promoted.
