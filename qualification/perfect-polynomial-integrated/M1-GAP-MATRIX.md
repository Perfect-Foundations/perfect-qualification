# Perfect Polynomial — authoritative M1 acceptance gap matrix (2026-10-09)

**Controlling inputs:** `perfect-family/docs/CRATES-IO-READINESS.md` at locally inspected family `6801f4ebd9870d0342d579604c32f2e0bc03c14a`; Polynomial approved REQ files, ADRs, `docs/TRACEABILITY.md`, `project-status.toml` at **integrated PR #8 exact source SHA** `d2365a0fc5af81877e64598301064ea2910e67f9`; `perfect-qualification/QUALIFICATION-PLAN.md` Q0–Q6. This record is qualification **evidence for a subset**, not gate signoff. Source dependencies: Arithmetic `1a54d3c7cbbae4e73325cc70fd2777a2427b1504`; Rational `35a8e9cc629ee578fe7b624e2134929ce7eeff8a`; transitive Numeric `19b6747cd852a47a694a020b97ba70b6b3ef259b`.

| Requirement | Acceptance criterion and current evidence | Missing evidence / classification | Executable now? | M1-blocking? | Next action |
|---|---|---|---|---|---|
| REQ-CORE-0001 | Canonical coefficient order, zero/degree, construction verified via source-independent public consumer on Windows/Linux and 79-case oracle. **PARTIAL PASS** | Complete cross-host semantic fingerprint and explicit reviewer acceptance **NOT-RUN** | Yes | Yes | Add cross-host deterministic fingerprint and sign off separately |
| REQ-CORE-0003 | Add/sub/mul, Horner, derivative, Q division, Z exact division, canonical exact GCD validated by public consumer and independent SymPy. **PARTIAL PASS** | Bounded high-degree/pathological property/fuzz/resource data and final M1 review **NOT-RUN** | Yes | Yes | Retain seeded external oracles, mutation assessment and resource guards |
| REQ-SEM-0001 | Exact integer/rational operations and oracle expected coefficients, no approximate API. **PARTIAL PASS** | Broader adversarial mutation/properties and formal semantics acceptance **NOT-RUN** | Yes | Yes | Source-independent randomized vectors and typed-error tests |
| REQ-SEM-0002 | Canonical zero and Z/Q GCD sign/monic contracts independently checked. **PARTIAL PASS** | Published/stable contract and full consistency review **NOT-RUN** | Yes | Yes | Reviewer-check docs/trait semantics and public contract |
| REQ-ERR-0001 | DivisionByZero/NonExactDivision/NonIntegralQuotient public typed errors and guard fallback tests. **PARTIAL PASS** | Broader malformed inputs, resource refusal/integer overflows and mutation coverage **NOT-RUN** | Yes | Yes | Targeted mutation/negative-case verification |
| REQ-DEP-0001 | Source pins and minimal exact Arithmetic+Rational production dependencies validated from Cargo.lock/Cargo tree. **PARTIAL PASS** | Final dependency/feature/supply-chain review and live advisory/license issue resolution **NOT COMPLETE** | Yes | Yes | Audit, record exact lockfile and no build/network behavior |
| REQ-PORT-0001 | Windows x64 + Ubuntu WSL2 x64 Rust 1.99.0 both feature modes PASS; AArch64/wasm32v1/Thumb/RISC-V no-default cross-check PASS. **PARTIAL PASS** | **Native physical Linux ARM64 runtime NOT RUN** (Class-A declared release blocking), full portable runtime and declared MSRV proof **PARTIAL** | Partially | M1 acceptance unresolved, Class-A release blocking | Preserve compile-only proof; obtain zero-cost native ARM64 when available, no fabricated result |
| REQ-SEC-0001 | Polynomial production source `#![forbid(unsafe_code)]`, no build.rs and no foreign CAS/runtime in Cargo manifest; offline tests PASS. **PARTIAL PASS** | Complete transitive unsafe/native/FFI/build-script/security audit **NOT COMPLETE** | Yes | Yes | Record tool-based audit and full source boundary review |
| REQ-PERF-0001 | Retained matched operation/allocator evidence for integrated PR #8; regression facts documented. **PARTIAL** | Full M3 representative acceptance later | Yes | No, **M3** | Avoid speculative M1 performance rewrites |
| REQ-PKG-0001 | `cargo package --list` source inventory PASS. Actual `cargo package --offline --locked --allow-dirty` and `--no-verify` **FAIL**: registry index lacks unpublished `perfect-arithmetic`. | Exact registry-resolution package construction/verification **BLOCKED**; no fabricated package PASS. Version+Git pin policy intact. | Diagnostic checks yes | **M4/release**, private readiness work now | Retain failure, check generated registry mapping in disposable staging, defer registry-dependent package acceptance |
| REQ-DOC-0001 | Source-level canonical exact semantics already documented. **PARTIAL** | Full release-level documentation review M4/M6 | Yes | No, M4/M6 | Complete package metadata now; no invented license |
| REQ-DET-0001 | Exact source outputs observed equal on Windows/Linux. **PARTIAL** | Formal repeated semantic fingerprint Q3, M2/M5/M6 | Yes | No formal M1 block | Add reproducible hash fingerprints |
| REQ-VER-0001/0002 | SymPy finite-domain reference coverage available; no M1 certified roots API. **PARTIAL / NOT APPLICABLE roots at M1** | Later M2–M6 independent references/root certificate gates | Later | No | Do not claim certified roots |
| REQ-CORE-0002, REQ-INT-0001/0002 | Dense/sparse and algorithm ownership policy; later M4/M6 architecture semantics | Later milestones | Later | No | Retain domain ownership rules |

### Qualification gates (no entire gate signed off)

| Gate | Evidence now | Still missing |
|---|---|---|
| Q0 | Exact SHA/Cargo.lock, clean source check, manifest/DAG inventory | Complete reviewer/reuse/metadata signoff |
| Q1 | Two Class-A host tests + four portable cross-checks | Physical ARM64 execution and other declared matrix coverage |
| Q2 | Independent public Arithmetic/Rational→Polynomial consumers and typed errors | Formal domain compatibility acceptance |
| Q3 | Deterministic SymPy fixtures/lockfiles | Repeated portable semantic fingerprints/signoff |
| Q4 | Negative cases and newly seeded exact oracle tests | Mutation/fuzz/sanitizer report and acceptance |
| Q5 | 79 independent structured SymPy cases; further generated oracles | Formal external reference scope review |
| Q6 | Candidate not yet released | Registry package + offline built package, licensing/public release fields, audits/full release matrices |

### Publication distinction

The family policy expressly permits **private development and qualification without selecting a license, opening repositories or removing `publish=false`**. It nevertheless expects successful private package preparation. Registry-dependent package verification currently fails due to unpublished Perfect dependencies, which is a packaging/Q6 obstruction and must not be silently recast as a failure of the independently source-pinned M1 algebraic tests. The owner's license decision remains **TBD**. Authoritative `project-status.toml` remains implementation/M1, overall 20%, qualification 0%, release readiness 0% until formally accepted.
