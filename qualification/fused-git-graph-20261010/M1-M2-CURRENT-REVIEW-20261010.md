# Current-source M1/M2 technical decision packet — 2026-10-10

**Purpose:** concise independent-review handoff; **not** reviewer approval,
formal milestone acceptance or a release. Historical review packets
and fingerprint/source checkpoints remain unchanged and retain their
original SHAs. All 17 existing requirements remain **Approved**.
The owning Polynomial `project-status.toml` remains M1, 20% overall.

## Exact four-crate revision

| Crate | Immutable source revision |
|---|---|
| Perfect Numeric | `19b6747cd852a47a694a020b97ba70b6b3ef259b` |
| Perfect Arithmetic | `d99aef2a9ef230eb725adaef664b0cc0214dc4bd` |
| Perfect Rational | `b45de7e7ad63b0850e1d3bbc67e38163b3c8fa7c` |
| Perfect Polynomial | **`59fa967bdf2c20dbde7a93acb22979c8625441af`** |

The separate consumer `Cargo.toml`, `Cargo.lock`,
`source-exact.json` and `verify_graph.py` prove a single Git-sourced
Arithmetic identity, correct Rational transitive dependency, exact
Numeric source and no local path/`[patch]` substitution.

## Implemented and mathematically checked

- Existing exact Z/Q dense arithmetic, Horner evaluation, derivative,
  signed/content GCD, typed integral-exact Z division and field Q
  quotient/remainder, with bounded fraction-free division and primitive
  PRS that refuse privately and fall back exactly, remain intact.
- New Polynomial `4efb2716f9a09fceb3882d396928f5b836245054`
  specializes GCD with one zero operand directly: Z returns
  positive-leading **original-content** polynomial and Q returns
  monic normalization; two-zero remains canonical zero.
  Independent signed/wide Rust math tests passed.
- New Polynomial `1e584a8f842e0f81725982c0896a1ce19d302c41`
  computes exact division by `c*x^d` without Rational lifting:
  checks all lower-coefficient remainders **first**, then performs
  signed integral division of higher coefficients.
  `NonExactDivision` wins over `NonIntegralQuotient`.
  Independent `verification/check_monomial_division_fraction.py`
  proves twenty signed degree/leading classes and a 4,098-bit typed
  error-priority case; the existing 15,625-case grid and 38 wide
  division references pass.
- Polynomial `59fa967bdf2c20dbde7a93acb22979c8625441af`
  appends a **current, source-exact** `docs/TRACEABILITY.md` section
  using accepted IDs `REQ-CORE-0001/0003`, `REQ-SEM-0001/0002`,
  `REQ-ERR-0001`, `REQ-DEP-0001`, `REQ-PORT-0001`,
  `REQ-SEC-0001`, `REQ-DET-0001`, `REQ-VER-0001`,
  `REQ-PERF-0001`, and later-capability IDs without reclassifying
  requirements or changing project status.
- Qualification-owned `tests/m2_quotient_free_and_fallback_integration.rs`
  adds two current-source zero-GCD and monomial typed-error cross-crate
  checks, making **24** tests per feature mode. Source has **94** tests
  per feature mode. All pass on both hosts with Rust 1.99.0.
  Earlier independently computed SymPy/Fraction oracles and PRS
  resource-guard paths remain pinned; Qualification checked them.

## Current-source exact cross-host determinism

Reused the **existing** versioned and independently specified V1/V2
canonical programs (no duplicated new oracle corpus). Re-executed
the final pinned Polynomial commit on Windows x86-64 and Ubuntu WSL2
Linux x86-64, with all and no-default features — eight complete raw
streams, all checked byte-for-byte and by SHA-256.

| Stream | Records | Bytes per execution | SHA-256 (all four identical) |
|---|---:|---:|---|
| V1 canonical math | 37 | 7,225 | `4bd81775891439f48cee151c08a4a58293b4d1bd1e9d3c1a28d4e9b4da4c388d` |
| V2 bound/dispatch semantics | 27 | 4,165 | `87c8a080eeedca85482b486e47c03226a7266a3d3898efeb6ac5fcc3fd27b3fc` |

`determinism-evidence/current-59fa967/` contains source-exact
bytes/manifest and per-feature test logs;
`references/verify_current_source_determinism.py` verifies all eight
raw files against the manifest and exact graph SHA.
Windows separately executed the fully independent SymPy 1.14.0
and Python Fraction V1/V2 oracles on **current bytes**; V2 also
was independently checked on all four current outputs from Windows.
A matching historical V2 digest is **not** substituted for current
execution: the new streams were actually regenerated.

## Complete-operation performance, not microbenchmarks

Three alternating baseline/candidate Rust 1.99.0 locked Windows
release trials, identical input per comparison, count allocator
and selected complete API operations:

| Optimization | Baseline calls | Candidate calls | Baseline bytes | Candidate bytes |
|---|---:|---:|---:|---:|
| `gcd_Z(0,p)` for 100 dense wide-degree operations | 57,100 | 200 | 6,512,000 | 420,000 |
| `gcd_Q(p,0)` for 150 negative-leading operations | 38,700 | 19,350 | 3,660,000 | 1,830,000 |
| 80 `div_exact` nonintegral `2*x` operations | 72,960 | 160 | 9,693,440 | 338,560 |

Sampled latencies and peak incremental live bytes, exact baseline SHA,
inputs and limitations are in the owning source files
`verification/M1-ZERO-GCD-DIRECT-20261010.md` and
`verification/M1-MONOMIAL-EXACT-DIVISION-20261010.md`.
Do **not** generalize these large improvements to nonzero/nonmonomial
paths. Sparse nonmonomial control allocation totals were unchanged.

Two new sequential scoped mutations were **killed**: replacing
the monomial nonzero-remainder predicate `any` with `all`
and reversing zero-GCD leading-sign logic. Source SHA256 restored
byte-for-byte and tracked diffs remained clean; no mutant committed.

## Technical readiness and external decision boundary

**Implemented:** M1 canonical exact initial Z/Q core; verified
selected M2 acceleration/fallback mathematics, public typed errors,
signed width/normalization, new zero-GCD and monomial specialization.

**Engineering tested:** 94 Polynomial and 24 Qualification tests
per feature mode on Windows and WSL2 Linux x86-64, all pass;
warning-denied all-target Clippy both hosts; canonical byte parity,
graph verifier, independent exact oracles, warning-denied Windows
rustdoc/rustfmt. Earlier PRS width/fractional refusal/continuation
tests and independent source-specific evidence remain retained.

**Independent math checks:** Python Fraction, SymPy and Qualification
owned expected vectors; sufficient to submit this candidate for a
human mathematical/source/API review, **not** a universal proof.

**Not externally approved or formally accepted:** independent public
Arithmetic API and Polynomial API/source/security reviews, physical
Linux ARM64 **runtime**, GitHub-hosted job execution and CI gate,
owner license/advisory/packaging approvals, M1/M2 milestones,
cross-family release qualification, crates.io publication. No
separate physical ARM64 behavior is claimed from WSL2 or compile-only
targets; status percentages and protected branches remain unchanged.

**Highest-value reviewer questions:** (1) public dependency APIs
`Integer::sub_mul_assign`, `scale_sub_mul_assign` and
`unsigned_bit_length`; (2) field/PRS exact resource semantics and
error precedence; (3) invariants of canonical coefficients and
leading content; (4) Class-A native ARM64 and hosted CI options;
(5) source exact dependency licenses/security and release policy.

**Fresh no_std portability evidence:** At exact Polynomial `59fa967b`,
Windows Rust 1.99.0 `cargo check --locked --no-default-features`
passed `wasm32v1-none`, `thumbv7em-none-eabihf` and
`riscv64imac-unknown-none-elf` compile-only targets. This does not
constitute physical execution or all-configuration verification on
those embedded architectures. Windows warning-denied rustdoc passed
for source and consumer.
