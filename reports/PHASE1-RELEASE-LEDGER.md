# Phase 1 — source-exact private release ledger (2026-10-23)

**Release decision: NOT READY.** This is the single consolidated Phase 1 report within the existing Qualification repository. Per-crate `project-status.toml` remains the lifecycle authority; historical qualification applies only to its exact source SHA. No release/progress status is promoted by this report.

## Current source graph

| Crate | GitHub main | Exact selected dependency identity |
| --- | --- | --- |
| Numeric | `b43e68d4fe5bb59e7682e0a687b0a1f899b146e5` | `19b6747cd852a47a694a020b97ba70b6b3ef259b` (last Q0-Q6 qualified) |
| Arithmetic | `95e8499f458d9cb681ce0d452cade1cd17c9f6ad` | same |
| Rational | `21a7fba5f22ab7f5db12b480a1c910edd1797d77` | same |
| Float | `e46b68ed20ec232be08e6c9f968ab2bc85db0736` | same |
| Decimal | `0b4766a015706bcb77c1c54d2ed3a552e3e8f933` | same |
| Qualification main | `f37fec893f3f9a3f587966d865a599bb6d036455` | independent test-only harness |

Do not repin Numeric alone: downstream direct/transitive users still resolve the historical qualified Numeric revision. No duplicated Cargo identities appeared in the Windows integration graph.

## Verification actually executed

Windows x86-64, Cargo 1.99.1, 2026-10-23:
- All five current main sources: offline locked no-default and all-feature test suites, warning-denied Clippy, rustdoc, rustfmt **PASS**. Float's all-feature suite retained one **IGNORED** release performance probe.
- Existing Qualification five-crate public integration: no-default **8/8 PASS**, all-features **8/8 PASS**, warning-denied Clippy, rustfmt, `cargo tree -d` **PASS**. Includes 6,627 independently bounded Decimal arithmetic cases and 3,800 cross-crate signed values.
- Arithmetic Q5 test-only consumer repinned to current main: no-default/default/all-feature **9/9 PASS per mode**; warning-denied Clippy, rustfmt and 72 retained Python Natural/Integer/GCD vectors **PASS**. New independent `Natural::bit` test checks 528,384 primitive bit observations plus sparse 4,096-bit/extreme-index cases. See draft PR #16.
- `no_std` **15/15 compile-only PASS**: Numeric, Arithmetic, Rational, Float, Decimal each checked on `wasm32v1-none`, `thumbv6m-none-eabi`, `riscv32imac-unknown-none-elf`. These are **not** cross-platform runtime tests.
- Numeric current main `cargo package --offline` **PASS**: 80 files, 280.3 KiB unpacked, extracted package compiled. Its absent license is intentionally unresolved.
- Arithmetic/Rational/Float/Decimal package content lists **PASS**, but actual `cargo package --offline` **BLOCKED** because their required Perfect upstream crates have no matching crates.io registry entries.

## Competitive results (measured, not universal)

Raw, unchanged source-exact CSVs reside in [phase1-raw](phase1-raw/). Each benchmark uses a deterministic seed, release build, 7 samples per case, two sequential process runs on the same Windows host. Runs were **not** fully interleaved A/B/B/A; exact performance qualification, peak RSS and confidence intervals remain open. Values below are Perfect latency/reference latency.

| Matched public operation | Size | Observed latency ratio, run 1 / run 2 |
| --- | --- | --- |
| Arithmetic division/remainder vs Malachite 0.12.0 | 16,384-bit | 2.71x / 2.61x slower |
| Arithmetic multiplication vs ibig 0.3.6 | 2,048-bit | 1.55x / 1.60x slower |
| Rational balanced comparison vs num-rational 0.4.2 | 2,048-bit | 2.49x / 2.48x slower |
| Rational common-factor normalization vs num-rational 0.4.2 | 16,384-bit | 1.44x / 1.47x slower |
| Rational balanced comparison vs num-rational 0.4.2 | 16,384-bit | 0.93x / 0.93x (slightly faster) |

Existing optional Arithmetic `high-throughput` dispatch improved 16,384-bit multiplication ~15% in these sequential campaigns, while 16,384-bit division/remainder changed only ~2%; not a sufficient fix for the largest deficit. Rational 2,048-bit comparison used 2 allocations / 1,040 growth bytes vs num-rational 10 / 2,328, highlighting a deliberate latency/allocation tradeoff. Raw CSV byte hashes should be used for subsequent source-exact review.

## Unfinished mandatory gates

| Gate | Current disposition |
| --- | --- |
| Q0 repository integrity | BLOCKED — complete five-crate historical reuse/integrity signoff not reissued |
| Q1 MSRV/platform/feature matrix | BLOCKED — Linux source authentication, native ARM64 runtime, exact Rust 1.99.0 matrix |
| Q2 mathematical integration | BLOCKED — partial public operations pass; complete contracts unqualified |
| Q3 reproducibility | BLOCKED — independent cross-host outputs incomplete |
| Q4 robustness | BLOCKED — exact-head mutation/fuzz/sanitizer suite incomplete |
| Q5 independent oracle | BLOCKED — tested Arithmetic and selected primitives; complete Float/Decimal/Rational evidence pending |
| Q6 release candidate | BLOCKED — above gates and registry-equivalent package/approval prerequisites |
| R0–R7A private release | BLOCKED — package/bench/API/security/independent review prerequisites |
| R7B public publication | NOT APPLICABLE — not authorized |

WSL2 cannot authenticate to private Arithmetic Git. An attempted authorized source transfer was refused and cancelled. No Linux or ARM64 runtime pass, universal performance superiority, release, license decision, merge or paid CI success is claimed. Perfectπ remains untouched.

**Next executable priorities:** source-exact independent arithmetic division and Rational comparison investigation with matched allocations/footprint, qualification-owned large-case oracles, and controlled cross-platform verification when authorized; follow with review and repinning of every affected downstream consumer. Do not advance readiness until its exact mandatory gates actually pass.
## Exact raw SHA-256 digests

| Raw CSV | SHA-256 |
| --- | --- |
| phase1-arithmetic-competitor-main-20261023-run1.csv | d110f44312e39667d572ce931758be0fdfda30e39d4f77332763a7de41d41086 |
| phase1-arithmetic-competitor-main-20261023-run2.csv | 32ec0b5b1e4d84a0c67359b6139d734b121c9d93be2b8f597cbd587610a15fac |
| phase1-arithmetic-high-throughput-main-20261023-run1.csv | 6d8ff5aaee1bdd502a77348848ac42189110a8101990402d501b57a04da8c60d |
| phase1-arithmetic-high-throughput-main-20261023-run2.csv | ed0a82ca233b55a1765395fdfae9fcf380227d94bc747deb904210dce44fb7fd |
| phase1-rational-allocation-main-20261023.csv | 2b365a3f9a8672cdece050f938e1d2aefd5b1f6b362221a0a8d4b1e2a6c4905b |
| phase1-rational-competitor-main-20261023-run1.csv | facffe328beb12b1cefe90fa2032a22ca5c548360f5dab1df30c9a194eebfe2d |
| phase1-rational-competitor-main-20261023-run2.csv | 4b42794dde511f555d1d0c0b152428a6f3cacc0cb36fe84ba97f72915cfc37b4 |

## Float rounded multiplication current-source recheck

The existing `benchmarks/phase64-rounded-multiplication` consumer could not run with `--locked` on Float main `e46b68ed` because its lockfile pinned Rational `9d6ac480` instead of the production-required `21a7fba5`. This was fixed separately as draft Float PR #4 at `7b22c936ebd147bf2b1a68c4e22fad5c7e3bcc84` (only Cargo.lock changed). The source-exact locked/offline benchmark then ran successfully twice with its original code and Astro Float Num 0.3.7.

| Precision | Perfect Float latency / Astro latency: run 1 / run 2 |
| --- | --- |
| 64 bits | 0.268x / 0.271x |
| 128 bits | 0.986x / 0.999x |
| 256 bits | 1.629x / 1.644x |
| 1,024 bits | 1.292x / 1.311x |
| 4,096 bits | 1.992x / 1.948x |

Numbers are seven-round median multiplication latency for the harness's exact input patterns, not a comprehensive correctly-rounded MPFR oracle or general Float performance qualification. No production Float change is justified by these two sequential campaigns alone. Raw CSVs use byte-preserving `.gitattributes` and their Git blob SHA-256 digests follow.

| Raw Float CSV | SHA-256 |
| --- | --- |
| phase1-float-astro-main-20261023-run1.csv | f8ccee98a31d2b12fd1cc30649c18069b521bb51830e2805119e50b50f022984 |
| phase1-float-astro-main-20261023-run2.csv | 0713d861033b0d08add02682250a1be561ca4b5e05f4e312deacef95aaa6a5cc |

## Exact Rust 1.99.0 MSRV and bounded Decimal Q5 coverage (2026-10-23)

New execution on authorized Windows MSVC source-exact revisions in the source graph above:

- `cargo +1.99.0 test --offline --no-default-features --all-targets` and `cargo +1.99.0 test --offline --all-features --all-targets` **PASS for all five foundation mains**, using a shared Cargo target directory. Float retained one explicitly ignored performance probe. This qualifies the Windows test run under exact declared MSRV (not the full multi-OS MSRV matrix).
- `cargo +1.99.0 check --offline --no-default-features --target TARGET` with `RUSTFLAGS="-D warnings"`: **15/15 PASS**, five foundations times `wasm32v1-none`, `thumbv7em-none-eabihf`, `riscv64imac-unknown-none-elf`. Cross-target **compilation only**, not hardware/emulated execution.
- Existing pinned `qualification/perfect-arithmetic`: Rust 1.99.0 offline locked no-default and all-feature suites **9/9 PASS each**, warning-denied Clippy **PASS**.
- Existing five-foundation integration: added `tests/phase1_quantize_oracle.rs`, a separate bounded primitive integer division/remainder oracle for signed Decimal quantization. Independent expected coefficient and rounding direction are computed without Decimal conversion/reference helpers: **31,605 cases** across five rounding modes, seven source exponents, seven destination exponents and signed coefficients -64..64. An additional test verifies signed-zero preservation across source/target quanta. Rust 1.99.0 offline locked no-default/all-feature integration suites **10/10 PASS each**, warning-denied all-target Clippy and formatting **PASS**.
- This is new Windows source-exact Q1/Q5 evidence; **Q0–Q6 and R7A remain BLOCKED** by the broader unresolved release gates listed above. No source revision, production crate, license, package publication, readiness percentage or release approval changed.
## Rational exact-order candidate PR #7 (not selected into release graph)

Separate **draft** branch `perf/phase1-rational-bounded-prefix-20261023`,
exact SHA `4fbe3977a53e3f3bceb6329c0804c2c3b0e9a506`, based on
Rational `main` `21a7fba5f22ab7f5db12b480a1c910edd1797d77`.
Public `Rational::cmp` API and exact semantics unchanged. A provably
disjoint leading-product interval may certify ordering without wide
cross-products; overlapping intervals retain exact fallback.

Rust **1.99.0** Windows MSVC no-default/all-features full source tests,
160,000 bounded internal primitive comparisons, 1,152 external
independent large-fraction comparisons, Fibonacci-prefix tests,
Clippy/rustdoc warnings denied, and fmt: **PASS**.
No-default warning-denied compile-only checks on `wasm32v1-none`,
`thumbv7em-none-eabihf`, `riscv64imac-unknown-none-elf`: **3/3 PASS**.

The existing seven-sample quick release harness, two separate sequential
source campaigns, measured 2,048-bit balanced comparison
`1,979/1,983 ns` baseline vs `31.57/24.05 ns` candidate, and
16,384-bit `4,192/4,221 ns` vs `12.58/15.77 ns`.
Selected candidate comparison cases allocated **zero** counted bytes,
vs baseline 1,040 B (2,048-bit) and 61,456 B (16,384-bit).
Five adversarial Fibonacci-prefix comparisons measured
`1.015/1.075/1.020/1.024/1.026` candidate/baseline latency ratios
for 512/2,048/4,096/8,192/16,384-bit inputs. Initial 16-bit variant
regressed adversarial cases more severely and was rejected in favor of
8-bit interval bounds with a 1,024-bit admission threshold.

**Limitations:** baseline/candidate process sequences are not ABBA
interleaved, other operations show host noise, resource and linked-code
size matrices remain incomplete, and Linux/native ARM64, full mutation
and downstream exact-source qualification have not passed. Candidate
**NOT MERGED**, all source graph pins remain on current `main`.
M5, Q0-Q6 and R7A **NOT READY**.

| Candidate raw CSV | SHA-256 |
| --- | --- |
| phase1-rational-prefix8-candidate-20261023-run1.csv | 86c5eab82574161eeaf5ce5861f860e3d9fb9e89324ea0f83b345b89dcc259ba |
| phase1-rational-prefix8-candidate-20261023-run2.csv | 063eb582189a54c2896f0bf3b39a17c583a837602fc040526f70f30bf20daae6 |
| phase1-rational-prefix8-pathological-20261023.csv | a9353f22b57f473e20a39507c5108ffb39723fb10d859b0e9254aeb124eeaf47 |
| phase1-rational-prefix8-allocations-20261023.csv | 48b0a62c63fed7532bf835f3813376e0270a440edc94690c2334b088671a91e9 |

The allocation CSV was normalized losslessly from PowerShell UTF-16 redirected text to UTF-8 with LF line endings for review; numeric values and rows are unchanged.

## Decimal independent rounded-division oracle (Rust 1.99.0)

The existing five-crate test-only consumer now includes 73,200 additional bounded signed Decimal `div_round` cases, checked against an independent i128 integer-quotient/remainder significant-digit oracle. Source exponents are zero, source integers run -30..30 with nonzero divisors, four precisions (1/2/3/5) and five explicit rounding modes; tests independently verify exact resulting Rational value and the rounding error direction. Division by zero and extreme exponent/resource semantics are covered elsewhere and are **not** certified by this bounded test.

Windows Rust 1.99.0 offline locked source-exact no-default/all-feature full integration suites: **11/11 PASS in each mode**. All-feature all-target Clippy with `-D warnings` and formatting checks: **PASS**. This extends Q2/Q5 evidence but leaves Decimal M1, comprehensive numerical coverage, Linux/ARM64 runtime and independent Q0-Q6/R7A **NOT READY**.

## Independent Rational Q5 references on the unchanged five-foundation graph

The existing five-foundation integration consumer now owns
`qualification/perfect-four-crate-integration/tests/phase1_rational_reference.rs`,
adapted with explicit provenance from Perfect Rational main
`21a7fba5f22ab7f5db12b480a1c910edd1797d77`,
original `tests/phase65_bounded_cmp.rs`. The independent mathematical
oracle is `num-rational 0.4.2` plus `num-bigint 0.4.8`, test-only.
The existing Arithmetic backend may separately resolve `num-bigint 0.5.1`;
this is a **dev/reference** version duplication, not a duplicate
production Perfect crate identity or new runtime dependency.

Tests cover 1,152 forward/reverse wide signed comparisons, 48
common-numerator/denominator and ordered-boundary cases, and four
long shared continued-fraction-prefix adversarial cases. Existing
Rational public APIs and source graph pins were not modified.
Exact Windows MSRV Rust 1.99.0, locked offline no-default and
all-feature integration suites: **14/14 PASS per configuration**.
All-target all-feature Clippy with warnings denied and formatting
checks: **PASS**. Production-only `cargo tree --edges normal -d`
reports no duplicated package identities.

This is Qualification-owned, repeatable evidence for Rational
**main**, not proof that unmerged Rational PR #7 passes on the
candidate SHA. Candidate-specific independent verification remains
uncompleted; no repinning of Float/Decimal to that candidate, no
gate advancement or R7A readiness claim. Linux/ARM64 native
runtime, exact-head mutation, resource/performance breadth and
registry package proof remain BLOCKED/OPEN.

## Bounded Float binary rounding reference

Rust 1.99.0 Windows Qualification adds 18,800 integer-oracle rounding observations across five modes and four precisions in tests/phase1_float_rounding_oracle.rs. MSRV no-default and all-feature integration pass 15/15 in each mode, Clippy -D warnings and fmt pass. Expected dyadic values use Float exact conversion and therefore do not form a fully independent MPFR oracle. Q0-Q6 and R7A remain NOT READY.


## Large Arithmetic division independent reference

A new Qualification-owned phase1_arithmetic_division_oracle.rs checks 240 large Natural div_rem inputs at widths 64, 256, 1024, 4096, 8192 and 16384 bits against test-only num-bigint 0.4.8 (Arithmetic internally uses 0.5.1). It independently verifies quotient/remainder bits, reconstruction a = q*d + r, remainder bounds, division by one, a smaller than divisor, and division-by-zero error. This tests exact Arithmetic main 95e8499f458d9cb681ce0d452cade1cd17c9f6ad, not an optimized candidate.

Windows Rust 1.99.0 locked offline no-default and all-features five-foundation integration: 17/17 PASS in each configuration. Warning-denied Clippy, formatting, and normal-edge dependency duplicate check PASS. The previous 16,384-bit performance deficit remains open; correctness vectors do not imply competitive latency. Q0-Q6/R7A remain NOT READY.


## MSRV optimized release-mode integration

Exact Qualification branch source 2c0e3c8a5453c12b6f12c00854a55138d03c6b79: cargo +1.99.0 test --locked --offline --release --all-features --all-targets passes 17/17 Windows tests. This supplements, but does not replace, the MSRV debug, portability and independent reference evidence above. Native Linux/ARM64, mutations, fuzzing and Q6/R7A remain blocked.

## Independent Rational PR #7 candidate verification — 2026-10-24

Current Rational PR #7 head: 0f8ccd07b1f3c29b1d725ef79fb6dfa48274f9b2.
Production Arithmetic selected: 95e8499f458d9cb681ce0d452cade1cd17c9f6ad.
Numeric remains 19b6747cd852a47a694a020b97ba70b6b3ef259b.

The isolated test-only Qualification consumer at
qualification/perfect-rational-candidate/Cargo.toml pins that exact
Rational candidate Git SHA; it reuses, without duplicating, the independent
Rational reference cases owned by the existing five-foundation integration
consumer. Cargo.lock was generated by Cargo 1.99.0 after an authorized Git
fetch, then used with --locked --offline for all verification.

Verified on Zen Windows x86-64 at Rust 1.99.0:
- Candidate no-default, default, and optimized release feature suites:
  3/3 tests PASS per configuration.
- Independent num-rational 0.4.2 / num-bigint 0.4.8 signed large-fraction
  reference, boundary, and long-prefix oracle exercised in each suite.
- Candidate warning-denied Clippy and rustfmt: PASS.
- Candidate normal-edge Cargo duplicate graph: none found.
- Original pinned five-crate integration, unaffected by candidate:
  17/17 no-default and 17/17 all-features tests PASS; warning-denied Clippy PASS.

Rational CI and benchmark references were independently repaired in PR #7
without changing public source APIs. A benchmark-only transitive
num-modular 0.6.6 lock entry was yanked; Cargo generated a lock using
0.6.5, followed by successful locked offline benchmark compilation.
Float PR #4 received coherent Arithmetic/Rational CI, MPFR and M3
performance references, Cargo-generated lock and successful Windows
Rust 1.99.0 source tests, lint, formatting and locked reference resolution.

Candidate-only PASS is not downstream Float/Decimal integration
qualification. Linux native and physical ARM64 runtime, mutation/fuzz
replay, comprehensive reference/benchmark qualification, registry package
proof and Q0–Q6 / private R7A remain OPEN or BLOCKED. No publication,
paid hosted CI, merge or PerfectPi change was performed.
## Post-correction no_std class-B compile checks — 2026-10-24

Using the authorized Zen Windows target, exact Rust 1.99.0 with
CARGO_NET_OFFLINE=true and RUSTFLAGS=-D warnings, run Cargo check
--no-default-features independently for each of the following exact
candidate revisions:

- Rational PR #7: 0f8ccd07b1f3c29b1d725ef79fb6dfa48274f9b2
- Float PR #4: 9e8d7c608ec0c5faadb46c427a82d1df84185a01

Targets wasm32v1-none, thumbv7em-none-eabihf and
riscv64imac-unknown-none-elf: all six checks PASS.
This is compile-only evidence; no WASM/Thumb/RISC-V program ran.
Windows source suites and Qualification exact-candidate oracle results
are documented above. Native Linux remains BLOCKED because WSL2 lacks
credentials for private Arithmetic Git (the target process is usable).
Hosted CI was not run; GitHub workflow lookup shows no new runs for
these skip-ci candidate commits. Physical ARM64, full mutation/fuzz,
downstream Rational-candidate integration, performance gaps and
Q0-Q6/R7A release gates remain OPEN or BLOCKED.
## Rational candidate downstream five-foundation graph (2026-10-24)

The existing test-only `qualification/perfect-rational-candidate`
consumer was extended *in place* with direct exact Git pins for Float
and Decimal candidate branches and path references to the already-owned
five-foundation independent test oracles. No new benchmark framework,
production runtime dependency, or duplicate fixture source was added.

- Numeric: `19b6747cd852a47a694a020b97ba70b6b3ef259b`
- Arithmetic: `95e8499f458d9cb681ce0d452cade1cd17c9f6ad`
- Rational PR #7: `0f8ccd07b1f3c29b1d725ef79fb6dfa48274f9b2`
- Float downstream branch: `798ef3bd88d3effc916f6cb4a7ba2bb03e884764`
- Decimal downstream branch: `eee4322047c07fba45c0f6644f5835d04448d4d9`

The Float branch is derived from draft Float PR #4 and changes only the
selected Rational revision in CI/source/reference manifests and locks;
the Decimal branch derives from Decimal main and changes only Rational's
revision in source manifest and Cargo lock. Cargo generated all lockfiles.
Both downstream source branches passed Windows Rust 1.99.0 locked offline
no-default and all-features source tests, warning-denied Clippy, rustfmt,
and normal-edge Cargo duplicate-package checks.

**Candidate Qualification evidence, Zen Windows x86-64 Rust 1.99.0:**
Cargo-generated exact Git lock resolution **PASS**. Normal-edge Cargo
tree contains exactly one source identity for each of the five Perfect
production crates; `cargo tree --locked --offline -e normal -d` reports
no duplicates. Both `cargo test --locked --offline --no-default-features
--all-targets` and `--all-features --all-targets` pass **12/12** tests
each (9 reused cross-family tests plus 3 independent Rational external
reference tests). Warning-denied all-feature Clippy and rustfmt check
**PASS**. An earlier trial using a `[patch]` to the same Git repository
was rejected by Cargo and removed before this coherent graph was made.

This evidence is **candidate-specific** and does not advance the five-main
release graph. Linux Git access remains blocked, and source-exact native
ARM64, mutation/fuzz, comprehensive MPFR and resource/performance
qualification, packaging and private R7A remain OPEN / NOT READY.
All downstream branches and PRs must remain unmerged until requirements
and affected-consumer qualification are satisfied. No paid CI, publication,
license selection, or PerfectPi write was performed.

## Rational candidate downstream post-qualification verification (2026-10-24)

Candidate branch PRs are separately preserved and remain draft/unmerged:
- Float <https://github.com/Perfect-Foundations/perfect-float/pull/5>,
  SHA `798ef3bd88d3effc916f6cb4a7ba2bb03e884764`,
  based on Float infrastructure PR #4 at `9e8d7c608ec0c5faadb46c427a82d1df84185a01`.
- Decimal <https://github.com/Perfect-Foundations/perfect-decimal/pull/3>,
  SHA `eee4322047c07fba45c0f6644f5835d04448d4d9`,
  based on Decimal main `0b4766a015706bcb77c1c54d2ed3a552e3e8f933`.
- Rational candidate <https://github.com/Perfect-Foundations/perfect-rational/pull/7>,
  SHA `0f8ccd07b1f3c29b1d725ef79fb6dfa48274f9b2`.

**Zen Windows x86-64, Rust 1.99.0, exact candidates, offline locked:**

| Check | Float | Decimal | Independent Qualification candidate |
| --- | --- | --- | --- |
| `cargo test --locked --offline --no-default-features --all-targets` | PASS | PASS | PASS 12/12 |
| `cargo test --locked --offline --all-features --all-targets` | PASS (one performance probe ignored) | PASS | PASS 12/12 |
| `cargo test --release --locked --offline --all-features --all-targets` | NOT RUN | NOT RUN | PASS 12/12 |
| `cargo clippy --locked --offline --all-features --all-targets -- -D warnings` | PASS | PASS | PASS |
| `cargo fmt --all -- --check` | PASS | PASS | PASS |
| `cargo doc --locked --offline --all-features --no-deps` with `RUSTDOCFLAGS=-D warnings` | PASS | PASS | NOT RUN |
| `cargo check --locked --offline --no-default-features --target TARGET` with `RUSTFLAGS=-D warnings` | 3/3 PASS | 3/3 PASS | NOT RUN |

TARGET above denotes `wasm32v1-none`, `thumbv7em-none-eabihf`,
and `riscv64imac-unknown-none-elf`. Cross-target results establish
compilation only. All source tests and independent Qualification tests
were conducted on Windows native, not on Linux or physical ARM64.
These results apply only to the pinned candidate dependency graph and
**do not** upgrade production main, qualification completeness or
private R7A readiness. No public release, merge, paid hosted CI or
PerfectPi modification occurred. Source-specific Linux/ARM64 runtime,
mutation/fuzz, full MPFR mathematical oracles, competitive performance,
supply-chain and package release gates remain OPEN/BLOCKED.
