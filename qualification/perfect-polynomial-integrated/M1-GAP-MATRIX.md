# Perfect Polynomial — authoritative M1 acceptance gap matrix (2026-10-09)

**Controlling inputs:** `perfect-family/docs/CRATES-IO-READINESS.md` at locally inspected family `6801f4ebd9870d0342d579604c32f2e0bc03c14a`; Polynomial approved REQ files, ADRs, `docs/TRACEABILITY.md`, `project-status.toml` at **integrated PR #8 exact source SHA** `d2365a0fc5af81877e64598301064ea2910e67f9`; `perfect-qualification/QUALIFICATION-PLAN.md` Q0–Q6. This record is qualification **evidence for a subset**, not gate signoff. Source dependencies: Arithmetic `1a54d3c7cbbae4e73325cc70fd2777a2427b1504`; Rational `35a8e9cc629ee578fe7b624e2134929ce7eeff8a`; transitive Numeric `19b6747cd852a47a694a020b97ba70b6b3ef259b`.

| Requirement | Acceptance criterion and current evidence | Missing evidence / classification | Executable now? | M1-blocking? | Next action |
|---|---|---|---|---|---|
| REQ-CORE-0001 | Canonical coefficient order, zero/degree, construction verified via source-independent public consumer on Windows/Linux and 79-case oracle. **PARTIAL PASS** | 579-case Windows/Linux fingerprint matches; independent reviewer acceptance **NOT-RUN** | Yes | Yes | Retain fingerprint 435349aa9a62498a and obtain separate semantic signoff |
| REQ-CORE-0003 | Add/sub/mul, Horner, derivative, Q division, Z exact division, canonical exact GCD validated by public consumer and independent SymPy. **PARTIAL PASS** | Long-duration/pathological fuzz/resource acceptance and final M1 review **PARTIAL / NOT-RUN** | Yes | Yes | Retain retained 500 seeded SymPy references, 10k native fuzz runs, targeted mutation analysis and allocator census |
| REQ-SEM-0001 | Exact integer/rational operations and oracle expected coefficients, no approximate API. **PARTIAL PASS** | Broader adversarial mutation/properties and formal semantics acceptance **NOT-RUN** | Yes | Yes | 500 seeded SymPy and 10k bounded real libFuzzer cases now retained; expand large-degree and abnormal-resource paths |
| REQ-SEM-0002 | Canonical zero and Z/Q GCD sign/monic contracts independently checked. **PARTIAL PASS** | Published/stable contract and full consistency review **NOT-RUN** | Yes | Yes | Reviewer-check docs/trait semantics and public contract |
| REQ-ERR-0001 | DivisionByZero/NonExactDivision/NonIntegralQuotient public typed errors and guard fallback tests. **PARTIAL PASS** | Broader malformed/resource-refusal cases and mutation adequacy **PARTIAL** (11 mutants: 1 caught / 6 missed / 4 unviable) | Yes | Yes | Address survived bounded-LCM optimization mutants with independent path-observable tests; do not assume mathematical failure |
| REQ-DEP-0001 | Source pins and minimal exact Arithmetic+Rational production dependencies validated from Cargo.lock/Cargo tree. **PARTIAL PASS** | Final dependency/feature/supply-chain review and live advisory/license issue resolution **NOT COMPLETE** | Yes | Yes | Audit, record exact lockfile and no build/network behavior |
| REQ-PORT-0001 | Windows x64 + Ubuntu WSL2 x64 Rust 1.99.0 both feature modes PASS; AArch64/wasm32v1/Thumb/RISC-V no-default cross-check PASS. **PARTIAL PASS** | **Native physical Linux ARM64 runtime NOT RUN** (Class-A declared release blocking), full portable runtime and declared MSRV proof **PARTIAL** | Partially | M1 acceptance unresolved, Class-A release blocking | Preserve compile-only proof; obtain zero-cost native ARM64 when available, no fabricated result |
| REQ-SEC-0001 | Polynomial production source `#![forbid(unsafe_code)]`, no build.rs and no foreign CAS/runtime in Cargo manifest; offline tests PASS. **PARTIAL PASS** | Complete transitive unsafe/native/FFI/build-script/security audit **NOT COMPLETE** | Yes | Yes | Record tool-based audit and full source boundary review |
| REQ-PERF-0001 | Retained matched operation/allocator evidence for integrated PR #8; regression facts documented. **PARTIAL** | Full M3 representative acceptance later | Yes | No, **M3** | Avoid speculative M1 performance rewrites |
| REQ-PKG-0001 | `cargo package --list` source inventory PASS. Actual `cargo package --offline --locked --allow-dirty` and `--no-verify` **FAIL**: registry index lacks unpublished `perfect-arithmetic`. | Exact registry-resolution package construction/verification **BLOCKED**; no fabricated package PASS. Version+Git pin policy intact. | Diagnostic checks yes | **M4/release**, private readiness work now | Retain failure, check generated registry mapping in disposable staging, defer registry-dependent package acceptance |
| REQ-DOC-0001 | Source-level canonical exact semantics already documented. **PARTIAL** | Full release-level documentation review M4/M6 | Yes | No, M4/M6 | Complete package metadata now; no invented license |
| REQ-DET-0001 | Exact source outputs observed equal on Windows/Linux. **PARTIAL** | Additional repeatability and supported-platform evidence Q3, M2/M5/M6 | Yes | No formal M1 block | Add reproducible hash fingerprints |
| REQ-VER-0001/0002 | SymPy finite-domain reference coverage available; no M1 certified roots API. **PARTIAL / NOT APPLICABLE roots at M1** | Later M2–M6 independent references/root certificate gates | Later | No | Do not claim certified roots |
| REQ-CORE-0002, REQ-INT-0001/0002 | Dense/sparse and algorithm ownership policy; later M4/M6 architecture semantics | Later milestones | Later | No | Retain domain ownership rules |

### Qualification gates (no entire gate signed off)

| Gate | Evidence now | Still missing |
|---|---|---|
| Q0 | Exact SHA/Cargo.lock, clean source check, manifest/DAG inventory | Complete reviewer/reuse/metadata signoff |
| Q1 | Two Class-A host tests + four portable cross-checks | Physical ARM64 execution and other declared matrix coverage |
| Q2 | Independent public Arithmetic/Rational→Polynomial consumers and typed errors | Formal domain compatibility acceptance |
| Q3 | Deterministic SymPy fixtures/lockfiles | 579-case matching Windows/Linux digest available; formal broader-target signoff |
| Q4 | Negative cases and newly seeded exact oracle tests | 10k bounded native libFuzzer runs PASS; 6 bounded-LCM mutants MISSED; sanitizer/Miri/long-duration acceptance still open |
| Q5 | 79 independent structured SymPy cases; further generated oracles | Formal external reference scope review |
| Q6 | Candidate not yet released | Registry package + offline built package, licensing/public release fields, audits/full release matrices |

### Publication distinction

The family policy expressly permits **private development and qualification without selecting a license, opening repositories or removing `publish=false`**. It nevertheless expects successful private package preparation. Registry-dependent package verification currently fails due to unpublished Perfect dependencies, which is a packaging/Q6 obstruction and must not be silently recast as a failure of the independently source-pinned M1 algebraic tests. The owner's license decision remains **TBD**. Authoritative `project-status.toml` remains implementation/M1, overall 20%, qualification 0%, release readiness 0% until formally accepted.

## Focused bounded-LCM qualification update (test-only SHA 643c13b15205ae94a583e64588c0c0b48a9a6bd2)

- **REQ-CORE-0003 / REQ-SEM-0001 (partial PASS):** exact bounded denominator LCM and signed integer scaling verified directly; private optimized 4x4 Rational convolution matches independent Python Fraction coefficients. Exact fallback at a 513-bit LCM is verified.
- **REQ-ERR-0001 (partial PASS):** mathematical LCM threshold uses strict >512 width; 1/64/128/256/511/512 eligible, 513/514/768 declined. A 512-bit plus coprime-3 accumulated LCM declines. Not a general memory exhaustion guarantee.
- **Q4 (focused scope PASS, full gate incomplete):** cargo-mutants 27.1.0 replay of the same 11 bounded-LCM mutations: **7 caught, 0 missed, 4 unviable**, versus prior 1/6/4. Formerly missed optimization disabling and erroneous work bound mutants now detected by private tests. See mutation-evidence/lcm-targeted-20261009/README.md and structured outcomes.
- **Q5/Q2 (additional partial PASS):** previously untracked public-source-exact independent i128 oracle recovered and tested, 1,536 bounded arithmetic/evaluation cases plus 192 factor/GCD invariant cases, Windows and Linux x86-64 Rust 1.99.0 PASS. This does not independently establish expected GCDs; prior SymPy vectors still supply that reference.
- **REQ-DEP-0001 / REQ-SEC-0001 (additional partial review):** cargo metadata/tree inspect 8 locked packages, four registry crates expose SPDX MIT/Apache metadata; Perfect-family licenses remain undecided. Polynomial source forbids unsafe, has no own build.rs/FFI or network code; transitive num-traits requires a Rust build script with autocfg. Full transitive unsafe/security and resource audits are not signed off.
- **REQ-PORT-0001:** Windows and WSL2 Linux affected suites PASS both feature modes; physical ARM64 runtime remains NOT RUN. Portable-target compile-only prior evidence applies to unchanged production code and is not native runtime evidence.

**Formal status:** no full Q0–Q6 gate signed, no overall/qualification/release percentage updated. The cargo package registry precondition is unchanged (unpublished private Perfect dependencies), so previous exit-101 evidence remains authoritative. No unauthorized publication/license change.
