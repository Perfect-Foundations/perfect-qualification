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
