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
