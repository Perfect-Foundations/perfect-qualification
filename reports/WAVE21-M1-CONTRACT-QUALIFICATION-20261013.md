# Wave 21"È›y¯ßy‘ M1 Requirement Evidence Reconciliation and Independent Qualification
Date: 2026-10-13. Draft PRs; evidence only, not formal M1/Q6 acceptance.

## Exact production and Qualification source graph
- Error Correction #2: 58ccad8644287d91d5f82f903cdade12b1c63244
- Finance #2: dc632f1635597ab4238474871665d893ab6abcbb
- Algebra #3 unchanged: 20b2f10ae3390808d412be84c7af76ffbcb2766d
- Qualification previous HEAD: 5f335140ec9cbc79f1d0b083d5f488e875a4b617.
- Cargo manifests and locks in this evidence revision pin the exact new source commits. All prior frozen mathematical vector files and reference checksums remain unmodified.

## Requirement classifications ∫w^~)ﬁt Error Correction
| Requirement | Implementation | New/existing verification | Technical state |
|---|---|---|---|
| REQ-CORE-0001 | src/hamming.rs, src/packed.rs, src/rs.rs | hamming_exhaustive, rs_reference, wave21_hamming_full | M1 implementation exists; formal gate open |
| REQ-CORE-0002 | src/rs.rs and src/rs/batch.rs; src/profile.rs | direct_decode, wave20_profiles, Qualification wave20_profile | Structured results implemented; profile/guarantee review open |
| REQ-SEM-0001 | src/profile.rs; packed.rs; rs.rs | packed_profile, rs_reference, Wave 21 independent parity oracle | Explicit current M1 mapping; later standard profiles separate |
| REQ-SEM-0002 | src/rs.rs bounded-distance model | rs_reference, rs_high_degree, direct_decode | Within-region evidence; outside region has no original provenance guarantee |
| REQ-ERR-0001 | src/lib.rs typed errors; rs/batch.rs transactional admission | direct_decode, packed_buffers, wave20_profiles | Implemented; fuzz/mutation gate open |
| REQ-DEP-0001 | Cargo.toml exact Perfect Algebra field parent | rs_reference field-parent rejection, exact source graph | Source-integrated; formal Algebra Verified gate separate |
| REQ-DEP-0002 | src/rs/bw.rs code-local BW solver; no general polynomial public API | Source ownership review and graph | Conditional Polynomial ownership/Verified review OPEN |
| REQ-PORT-0001 | Rust 1.99 and no_std+alloc in Cargo.toml/lib.rs | Windows/WSL2/QEMU and three Class-B compile targets | Tested configurations only; physical ARM64 OPEN |
| REQ-SEC-0001 | forbid unsafe in src/lib.rs, reviewed Cargo.toml | Source/manifest and offline builds | Broader package/license/advisory audit OPEN |
| REQ-DET-0001 | scalar Hamming/RS and ordered RS batches | frozen vectors, exhaustive SECDED pairs, both feature modes | Tested determinism; formal cross-target fingerprints OPEN |

## Requirement classifications+ßuÁ‚ùÁT Finance
| Requirement | Implementation | New/existing verification | Technical state |
|---|---|---|---|
| REQ-CORE-0001 | src/lib.rs Money/CurrencyCode/Decimal | finance_core, multioperand_exact, Qualification exact ledger | Existing exact Money implementation |
| REQ-CORE-0002 | src/lib.rs DayCount/time::Date; ledger.rs/owned.rs | finance_core, wave21_daycount_independent, exact_range consumers | Four M1 conventions implemented; formal gate OPEN |
| REQ-SEM-0001 | CurrencyCode three uppercase syntax only | finance_core including syntactically valid ZZZ | No current ISO/rounding metadata implied |
| REQ-SEM-0002 | Money::quantize with explicit quantum/mode; currency mismatch | finance_core and exact monetary tests | Explicit quantization, no implicit FX |
| REQ-DATA-0002 | No market/calendar/catalog authority in M1 source | API and manifest inspection | Source constraint met; formal package audit OPEN |
| REQ-ERR-0001 | FinanceError, owned.rs atomic transaction, ledger.rs bounded query | owned_ledger, multi_range, wave20_transactions | Typed refusals tested; fuzz/mutation OPEN |
| REQ-DEP-0001 | Cargo.toml exact Decimal/Rational and time dependency | Source graph verifier and feature tests | Exact pins checked; formal license/advisory review OPEN |
| REQ-PORT-0001 | Rust 1.99 std/no_std+alloc core | Windows/WSL2/QEMU, three Class-B compilation targets | Physical Linux ARM64 OPEN |
| REQ-SEC-0001 | forbid unsafe, offline finance core, no build script | Source/manifest review; locked offline Cargo | Broader package/supply-chain audit OPEN |

Blank historical traceability cells have been filled only with grounded source/evidence, including explicit pending work. Existing Approved requirement statuses remain unchanged. This is not a progress metric.

## New mathematical and state-machine evidence
- Perfect Error Correction tests/wave21_hamming_full.rs is an independent binary even-parity encoder oracle for every supported Hamming r=3..8 size, both transport orders, all single-bit corruption positions, all two-bit extended Hamming/SECDED positions, and malformed padding with caller-buffer preservation. Both the production crate and an independently built public Qualification consumer passed it. Double errors are **detected**, never reported as corrected.
- Perfect Finance tests/wave21_daycount_independent.rs retains 24 external-Python-derived civil-date boundary pairs, each evaluated under the four approved conventions = 96 exact mathematical reference values. Cases include 1900/2000 leap-century behavior, February month-end, reversed date order and same-day periods. Independent Qualification verification-wave21-daycount.py regenerates the results using Python datetime.date and fractions.Fraction and checks the frozen Rust reference table. SHA256 of the regenerated normalized exact fraction rows: 67cf09a84b9a1f3910af65dc1fb154334c1e2b6450d952447ccc3957f102523a.

## Exact-source execution matrix
Rust 1.99, --locked --offline, both no-default and all-feature configurations:
| Platform | FEC Qualification consumer | Finance Qualification consumer | Warning-denied Clippy |
|---|---:|---:|---|
| Windows Native x86_64 | 20 passed/config | 14 passed/config | PASS both configurations |
| Ubuntu WSL2 x86_64 | 20 passed/config | 14 passed/config | PASS both configurations |
| AArch64 Linux user-mode QEMU | 20 passed/config | 14 passed/config | Not run for AArch64; executable tests PASS |

Twelve complete Qualification test commands yielded 204 passing executions, zero failures, zero ignored. Repetitions are NOT new mathematical reference cases. Production Windows all-target tests in both configurations passed at the exact source SHAs (FEC 23/19 and Finance 24/24 passed; deliberately ignored tests not counted). Changed production tests additionally passed Ubuntu WSL2 in both configurations. Both production crates passed no-default compile-only tests on wasm32v1-none, thumbv7em-none-eabihf and riscv64imac-unknown-none-elf. None of QEMU or compile-only tests count as physical hardware verification.

The unchanged preexisting frozen sources still pass verification-wave3-graphs.py and their independent BW GF7/Hamming/Finance reference regeneration at the new exact pins. The Linux private Git cache was populated by local bundles from exact Windows HEAD commits, with no path-substituted dependencies and no paid services.

## Further authorized gates
M1 implementation evidence is materially stronger. Formal M1 acceptance, Algebra/Polynomial Verified ownership, source-wide fuzz/mutation, exact cross-target fingerprints, physical ARM64, full package/license/advisory review, M5 and M6/Q6 acceptance remain separate. No release, merge, publication, feature-scope expansion, code-level optimization or percentage increase is claimed.

The previous Wave 20 source-isolated 180-row Finance performance evidence is retained unchanged. Wave 21 produced no new performance comparison; repeated benchmarks without an optimization were intentionally avoided.

No changes to PerfectPi, MachinaMesh, Algebra, protected branches or unrelated work. Original dirty Qualification worktree remains untouched. No billing action, paid resources, or hosted CI use.
