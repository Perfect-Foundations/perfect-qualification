# Source-exact Arithmetic M4 / Rational M5 / Polynomial integration

Executed 2026-10-10/11 America/New_York; final Qualification evidence for the authorized draft experimental source graph. This current consumer was updated from the previous graph in Git history (parent `817649d`); the independent historical `fused-git-graph-20261010` consumer is unchanged.

## Exact source revisions

- Numeric: `19b6747cd852a47a694a020b97ba70b6b3ef259b`
- Arithmetic: `65d8f55641739d5d6a11c17fe7d68c8898d984d7`
- Rational: `812d66757ce145a7a70fc33818c7b3059ecd3401`
- Polynomial: `d1208299eaea7c0dd924a19178689c67753fe3cc`

`source-exact.json` and `verify_graph.py` require the exact source Git identities, normalized Cargo.lock SHA-256, ten unchanged imported independent test-source SHA-256 values, SymPy fixture SHA-256, and current cross-operation test-source SHA-256. Cargo graph is `--locked --offline`; `cargo tree -d` shows no duplicate source identity. No `[patch]`, source override, new clone, worktree or replacement consumer.

## Final source-exact integration test matrix

| Execution environment | No default features | All features |
|---|---:|---:|
| Windows x86-64 | 34/34 passed | 34/34 passed |
| Ubuntu WSL2 Linux x86-64 | 34/34 passed | 34/34 passed |
| QEMU user-mode Linux AArch64 | 34/34 passed | 34/34 passed |

All six `cargo test --locked --offline` executions returned zero. Each runs 25 frozen independent mathematical checks and nine current cross-operation tests. The added cross-operation regression has independent CPython Fraction/integer results for signed unit Rational multiplication, integer-valued multiplication, Horner evaluation at 5/3, quotient/remainder by a signed constant and complete polynomial coefficients. The other two check exact mixed-denominator cross-operations, wide canonical zero normalization, polynomial GCD and fused Integer cancellation.

Windows all-target `cargo clippy -- -D warnings` passed both feature modes for the final Qualification consumer; Windows full Arithmetic, Rational and Polynomial source test/lint matrices also passed, and Rustfmt checked. Ubuntu WSL2 complete Rational and Polynomial source suites passed both feature modes at the final pinned revisions; Arithmetic M4 complete WSL2 source suites were previously executed against unchanged `65d8f55`. New changed-operation focused tests passed on QEMU in both feature modes when separately executed prior to this source graph; this current 33-test consumer additionally passed QEMU both modes.

Independent `verify_m4_m5_python.py` exact reference script passed 14 Integer/Fraction vectors on Windows and Ubuntu WSL2, both with SHA-256 `87ab8e114247fbb3df5489708821e8fe985693e461ef7ee1f4e7dd9c59b204b6`.

## Production scope and performance evidence

Arithmetic `65d8f55` optimized exact fused subtract from a zero accumulator. Rational `8bd86f3` includes shared-denominator addition, unit denominator construction, signed-unit multiplication/division, integral Rational multiplication and the exact Arithmetic dependency pin. Polynomial `815f6a2` includes exact derivative exponent conversion, bounded LCM/sparse multiplication optimizations, Horner leading-coefficient initialization and final Rational/Arithmetic source pins.

Matched selected release-mode baseline/candidate trials: Rational 256-bit multiply by +1 ~359 ns to ~48 ns, 8192-bit divide by +1 ~562 ns to ~86 ns; integral Rational 256-bit multiplication ~476 ns to ~136 ns, while a 256-bit mixed-denominator workload regressed ~1622 ns to ~2006 ns. Horner constant Integer evaluation ~42 ns to ~7 ns, degree-one Integer evaluation ~36 ns to ~24 ns; long Rational evaluations were comparable with some small regressions. These measurements are workload-specific; no universal acceleration, memory-allocation savings or formal statistical distribution is claimed. Unprofitable unrelated generic field-division and LCM-scaling candidates were not retained.



## Additional exact source changes and matched performance

Rational `8bd86f3` specializes direct subtraction with equal canonical denominators and zero operands, avoiding the large negated Rational intermediate for those cases. Polynomial `be76feb` specializes division by `c*x^d`, using exact coefficient slices for the remainder and corresponding shifted quotient with proper signed/nonunit scale; the public GCD quotient-free path receives the same exact remainder. Polynomial `815f6a2` reuses Arithmetic M4's existing mutable fused product/subtraction in Karatsuba schoolbook leaves with a negated multiplicand; no new Arithmetic public interface. Original PRS skip-unit-scaling was benchmarked and rejected due to matched complete-GCD regressions, so its source remains unchanged.

Matched Windows release-mode four-run A/B/B/A full public operation results, selected workloads, in nanoseconds:

| Case | Baseline | Accepted candidate |
|---|---:|---:|
| Rational sub equal denominator, 1,024-bit | 9,849 | 8,874 |
| Rational sub exact cancellation, 1,024-bit | 50 | 10 |
| Rational sub shared factor, 8,192-bit | 134,731 | 144,103 (regression) |
| Polynomial divide by (-1)*x^32, numerator degree 128 | 42,155 | 1,250 |
| Polynomial divide by (-3/5)*x^32, numerator degree 128 | 43,925 | 17,495 |
| Polynomial divide by wide-fraction*x^16, numerator degree 64 | 158,862 | 175,098 (regression) |
| Integer Polynomial 64x64 dense, 128-bit coefficients | 235,225 | 178,750 |
| Integer Polynomial 128x128 dense, 128-bit coefficients | 751,338 | 622,712 |

The two added current consumer tests use independent Fraction quotient/remainder reference coefficients, reduced subtraction, and a 64x64 dense Integer convolution with independently specified triangular output coefficients. New source-specific regressions directly passed on Windows and emulated Linux AArch64 in both feature modes. Complete Rational and Polynomial source tests passed Windows and WSL2 in both feature modes; Windows warning-denied Clippy and rustdoc checks passed. These benchmarks do not establish general speedups, allocations, or formal acceptance; effects outside tested input shapes remain open.

## Qualification boundary

This is verified exact behavior for the executed configurations, not physical ARM64, formally accepted M1/M2/M4/M5 milestones, exhaustive proof, security/API approval, hosted CI success, or release. Hosted Actions are externally billing-blocked. Perfectπ and unrelated source trees were not accessed or altered.

## Subsequent source-exact sparse field-division qualification — 2026-10-11

- Updated Polynomial `9a073971392c6caae1238337fb6c10afef11cbf0` (experimental PR #18) retains all prior exact public behavior. The only new production path caches the nonzero lower terms of a high-degree sufficiently sparse nonmonomial Rational divisor during general field division, rather than rescanning every zero coefficient at each quotient step.
- Arithmetic `65d8f55641739d5d6a11c17fe7d68c8898d984d7`, Rational `8bd86f328850c8153537842947b161ed7e3f0d24`, and Numeric `19b6747cd852a47a694a020b97ba70b6b3ef259b` are unchanged.
- New source-owned `tests/m2_sparse_field_division_20261011.rs` checks exact Fraction-expanded quotient/remainder coefficients for a degree-256 nonmonomial divisor and quotient-free GCD behavior. It passed Windows no-default and all-features, followed by full Polynomial suites, Rustfmt, and warning-denied all-target Clippy in both configurations.
- New Qualification-owned `high_degree_sparse_field_divisor_matches_independent_fraction_vectors` test in the existing `new_graph_cross_operations.rs` independently specifies all seven nonzero dividend terms and complete expected quotient and remainder.
- Final exact locked consumer: **25 imported independent + 6 current cross-operation = 31 tests**, zero failures per configuration: Windows native x86-64 no-default/all-features; WSL2 Linux native x86-64 no-default/all-features; QEMU user-emulated Linux AArch64 no-default/all-features. All six Cargo test commands exited zero. Windows warning-denied all-target Clippy also passed in both configurations. The exact graph/lockfile/imported-oracle hash verifier passed.
- Measured matched Windows release-mode A/B/B/A **public division** performance on selected sparse divisor/quotient degree pairs: (128,200) 52,058→12,982 ns; (256,200) 68,822→9,162 ns; (512,200) 146,230→20,075 ns. Dense controls (256,40) 1,222,316→1,107,699 ns and (512,40) 2,647,283→2,358,966 ns. These are workload-specific short measurements, **not** a universal performance claim.
- The earlier graph at `dc61c0fe3690ce59f7c878da4a4e41b1ca2b490c` is retained in Git history, and the separate `fused-git-graph-20261010/` corpus remains untouched. QEMU results are not physical ARM64 hardware acceptance. Hosted Actions remain account billing-blocked; formal M1/M2/M4/M5, independent API/security review, licenses, and releases are not approved.


## Current revision — integer monomial multiplication and mixed Rational product (2026-10-11)

The previous sparse Rational field division Qualification (Polynomial `9a07397`, Qualification `5348a78`) remains historical in Git history. This update advances only the existing current consumer, not the historical `fused-git-graph-20261010` corpus.

**Exact tested production graph:** Numeric `19b6747`, Arithmetic `65d8f55`, Rational `812d66757ce145a7a70fc33818c7b3059ecd3401`, Polynomial `29274e1edd5db2b23fb5ee2465987f11cd9d0efa`. Rational `812d667` specializes `mul` when exactly one operand has denominator one, performing only the potentially relevant cross-cancellation. Polynomial `f595b5e` recognizes canonical signed/nonunit integer monomial multipliers from a cheap first-coefficient gate, then performs exact coefficient scaling and shifting. Polynomial `29274e1` pins Rational `812d667` in manifest and lockfile.

Matched four-run A/B/B/A release-mode *public-operation* measurements (nanoseconds; baseline→candidate), selected operands only:

| Operation | Baseline | Candidate |
|---|---:|---:|
| IntegerPolynomial 128 coefficients, 256-bit × x^64 | 20,882 | 4,650 |
| IntegerPolynomial 512 coefficients, 256-bit × x^64 | 72,972 | 17,485 |
| IntegerPolynomial 128 coefficients, 256-bit × 17*x^64 | 20,830 | 5,430 |
| IntegerPolynomial 512 coefficients, 256-bit × dense degree-3 control | 174,020 | 184,628 (regression) |
| Rational integral × fractional, 256-bit | 342 | 222 |
| Rational fractional × integral, 1,024-bit | 400 | 273 |
| Rational exact integral/fractional cancellation, 4,096-bit | 302 | 206 |
| Rational fractional × fractional control, 1,024-bit | 15,533 | 14,928 |

These are short, matched workload observations, not universal performance guarantees, memory measurements or statistically qualified cross-platform speedups.

New permanent source-owned regressions check exact wide signed monomial coefficients, swapped operand order, canonical zero, sparse/dense reference convolution, single-integral Rational multiplication, full cancellation, sign, canonical reduction and fractional control. Two new independent current-consumer tests cover 4096-bit Fraction product identities and a 96-coefficient monomial scaling reference. The existing frozen 25 imported independent oracle tests were not changed.

**Final locked Qualification result:** 25 imported + 8 current cross-operation tests = **33/33 passing in six configurations**: Windows native x86-64, Ubuntu WSL2 x86-64 and QEMU user-mode Linux AArch64, each with no-default and all-features. All six test commands exited zero. Warning-denied Windows Qualification Clippy passed both modes. The updated immutable graph, lockfile digest, current cross-operation hash and unchanged imported SymPy/source hashes passed the exact verifier. Focused changed-operation source tests directly passed QEMU both modes; full affected Rust source tests passed Windows and WSL2 both modes; warning-denied Windows source Clippy and rustdoc passed both modes.

**Boundaries:** No physical ARM64 evidence or formal M1/M2/M4/M5 acceptance. Hosted Actions remain externally blocked by account billing. Independent API/security approval, licensing and releases remain pending. No PerfectPi files were accessed or modified.

## 2026-10-11 sparse Integer exact-division production follow-up

- Current pinned source is Polynomial `d1208299eaea7c0dd924a19178689c67753fe3cc`; all other four-crate source revisions are preserved from the prior graph. The production change reuses cached nonzero lower divisor coefficient references in sparse long Integer division, while avoiding the sparse-index allocation for ordinary small/dense divisors.
- The owning production regression `tests/m2_sparse_integer_division_20261011.rs` passed no-default/all-features; independent integer-convolution coefficient vectors, nonmonomial negative-leading/nonunit divisors, and exact `NonIntegralQuotient`/`NonExactDivision` errors are checked. Both complete Windows Polynomial source suites, rustfmt and warning-denied all-target Clippy passed for the source revision.
- New consumer-owned `sparse_integer_nonmonomial_division_matches_independent_exact_vectors` verifies complete expected quotient coefficients and typed error precedence without relying on the production regression. Final locked Qualification executed **34/34 checks in each of six configurations**, zero failures: Windows x86-64, WSL2 Linux x86-64, QEMU user-emulated AArch64 Linux, each with no-default and all-features. Windows Qualification warning-denied all-target Clippy passed both modes. Total: 204 executed independent test cases.
- The source-exact manifest/lockfile/SymPy and imported fixture SHA-256 verifier passed. The upstream versions were not replaced by local path patches, and no duplicate Arithmetic/Rational identities were introduced.
- A candidate-only release-mode performance probe, **not a matched before/after comparison**, measured selected sparse (divisor degree, quotient degree) operation times of (128,200) 6,557 ns; (256,200) 6,733 ns; (512,200) 7,530 ns. Dense controls: (256,40) 108,270 ns; (512,40) 223,900 ns. **No claimed relative speedup or universal performance benefit** follows from these measurements alone.
- The 2026-10-11 parent Qualification commit `0c203bd38889ba4f3305ae3f0b3bc284c94945a4` remains in Git history, as does the independent historical `fused-git-graph-20261010` consumer. Native physical ARM64, hosted Actions (known billing block), independent API/security approval, formal milestones, licensing and release remain pending. PerfectPi was not accessed, built, tested or modified.

## Matched sparse Integer division benchmark follow-up (2026-10-11)

The previously documented `d120829` optimization has now been compared against its direct source predecessor `29274e1edd5db2b23fb5ee2465987f11cd9d0efa`. This is **new performance evidence for the same already-qualified production revision**, not a new source revision or an additional Qualification test run.

The reproducibility fixture is `benchmarks/sparse_integer_division_20261011.rs` (SHA-256 of the fixture on Windows: `BA68A47F465EFDEDB357E607F0FA8762394EB233BD32CF2B084131F56166700B`). It is a standalone Cargo example source and is intentionally outside the consumer's `examples/` directory so that Qualification's 34-test execution matrix and all-target Clippy scope are unchanged. To rerun, copy it to a temporary example in an authorized experimental Polynomial checkout and execute that example with `cargo run --release --locked --offline --example <name>` at each specified exact source revision; do not alter protected branches or historical consumers.

The fixture constructs the exact dividend as `divisor.mul(&quotient)` before timing, measures only the public `IntegerPolynomial::div_exact(&divisor)`, and exercises divisor `x^d + 3` and quotient coefficients `(i % 9) + 1` for 201 terms. Dense controls use divisor coefficients `(i % 7) + 1` and quotient degree 20. The selected sparse quotient degree is 200. Repetition counts are 48 (sparse degree 64/128), 24 (sparse degree 256/512), and 4 (dense controls). Timed results use Rust `Instant` and `black_box`. Windows release-mode compile flags: `--locked --offline`, `CARGO_BUILD_JOBS=2`, `CARGO_INCREMENTAL=0`, same Cargo manifest/dependency lock and host. An automated four-run A/B/B/A execution substituted *only* `src/division.rs` from the parent commit, rebuilt the example for each variant, and restored the current source byte-for-byte in a `finally` block; its normalized content matched Git HEAD.

| Integer public division workload | Baseline median (ns) | Candidate median (ns) | Ratio |
|---|---:|---:|---:|
| Sparse divisor degree 64, quotient degree 200 | 25,655 | 12,988 | 1.98x |
| Sparse divisor degree 128, quotient degree 200 | 41,880 | 8,337 | 5.02x |
| Sparse divisor degree 256, quotient degree 200 | 76,100 | 8,406 | 9.05x |
| Sparse divisor degree 512, quotient degree 200 | 145,989 | 9,470 | 15.42x |
| Dense divisor degree 128, quotient degree 20 | 49,600 | 47,762 | 1.04x |
| Dense divisor degree 256, quotient degree 20 | 103,175 | 97,562 | 1.06x |

These are short, paired workload measurements with two observations per variant, not a statistical performance qualification, portable speedup guarantee, allocation comparison, or worst-case bound. In contrast to the earlier candidate-only data, these establish measured before/after results for the listed workloads.

**Storage cleanup after benchmarking:** A scoped `cargo clean` of the experimental Polynomial Windows target removed 7,203 generated files (943.3 MiB as reported by Cargo). All 24 root-level scripts and logs were copied, SHA-256 checked, restored, and independently rechecked before the duplicate backup was deleted. C: free at this execution's start was 23,056,355,328 bytes; measured free space after cleanup/backup removal was 24,051,347,456 bytes (increase 994,992,128 bytes). WSL2 VHDX was not compacted, unrelated caches and untracked evidence were not touched.

The exact tested and immutable source graph remains Numeric `19b6747`, Arithmetic `65d8f55`, Rational `812d667`, Polynomial `d120829`, already qualified by commit `2b84ceb` with 34/34 checks in each of six platform/feature executions. No new acceptance, CI, physical ARM64, or release evidence is implied by this benchmark-only record.

## 2026-10-11 exact nonzero-constant Polynomial GCD source and benchmark

**New exact Polynomial source SHA:** `0ea415f459c911b090c25ca25c62797bd2082fbc` (existing experimental PR #18; direct predecessor `d1208299eaea7c0dd924a19178689c67753fe3cc`). Numeric `19b6747`, Arithmetic `65d8f55`, and Rational `812d667` are unchanged. Production change in `src/division.rs` recognizes nonzero constant operands *after* the existing zero-GCD cases. For Q[x], every nonzero constant is invertible, so the canonical monic GCD is 1. For Z[x], the positive constant GCD is `gcd(content(left),content(right))`. This bypasses both PRS and rational Euclidean division for this exact case. Normal nonconstant paths remain unchanged.

**Independent source-owned regression:** `tests/m1_m2_constant_gcd_exact_20261011.rs` verifies signed coefficient contents, opposite order, two constants, zero semantics, 8192-bit exact magnitudes, fractional nonzero unit constants, and a nonconstant shared-factor control. All three tests passed in both Windows feature modes, complete Windows Polynomial test suites and warning-denied all-target Clippy passed both modes, and direct QEMU AArch64 execution of these three tests passed both modes.

**Independent Qualification regression:** `constant_polynomial_gcd_uses_exact_content_and_field_units` uses the separate SymPy-derived equation `gcd_Z(6*x^128+18*x^64-12,-18)=6`, both operand orders, exact rational unit GCD, and zero. Frozen 25 imported tests and prior nine cross-operation tests are unchanged. The current consumer includes 25 imported + 10 cross-operation tests = **35 tests per feature/platform configuration**.

**Matched release-mode public GCD benchmark:** `benchmarks/constant_gcd_20261011.rs` (canonical SHA-256 `b36e6f776ad8eb3a2a4da03cb73a6c956bbb33eb6b283c88714c4cbbf6a7b5d3`). Its source fixture is outside this consumer's Cargo examples so test/Clippy scope is unchanged. In the authorized Polynomial experiment, four ordered A/B/B/A builds substituted *only* the Git parent `src/division.rs` against the candidate file, rebuilding the same release-mode Cargo example each time with `--locked --offline`, `CARGO_BUILD_JOBS=2`, and `CARGO_INCREMENTAL=0`; a `finally` handler restored and byte-verified the candidate. Coefficients, polynomial degree, benchmark repetition counts and dense controls are frozen in the checked-in benchmark fixture. Nanoseconds below are medians of two iterations per source variant:

| Public GCD operation | Previous (ns) | Optimized (ns) | Ratio |
|---|---:|---:|---:|
| Z[x], degree 16 versus nonzero constant | 1,280 | 296 | 4.32x |
| Z[x], degree 64 versus nonzero constant | 1,711 | 361 | 4.74x |
| Z[x], degree 128 versus nonzero constant | 2,774 | 436 | 6.36x |
| Z[x], degree 256 versus nonzero constant | 4,726 | 661 | 7.15x |
| Q[x], degree 16 versus nonzero constant | 400 | 38 | 10.53x |
| Q[x], degree 64 versus nonzero constant | 662 | 36 | 18.39x |
| Q[x], degree 128 versus nonzero constant | 1,041 | 34 | 30.62x |
| Q[x], degree 256 versus nonzero constant | 1,780 | 35 | 50.86x |
| Z[x], degree 16 versus nonconstant degree-one control | 2,044 | 2,349 | 0.87x (regression) |
| Z[x], degree 256 versus nonconstant degree-one control | 11,392 | 11,359 | ~1.00x |
| Q[x], degree 256 versus nonconstant degree-one control | 8,636 | 8,428 | 1.02x |

These are short matched measurements of the specified exact workloads, not universal speedups, a formal statistical qualification, or an improvement to general nonconstant PRS. No physical AArch64 hardware testing, hosted CI, formal M1/M2 gate acceptance, independent API/security approval, licensing or release is implied. Historical Qualified consumers and PerfectPi were not modified.

**Final source-exact Qualification execution (current pinned graph):**

| Environment | No default features | All features |
|---|---:|---:|
| Windows x86-64 | 35/35 passed | 35/35 passed |
| Ubuntu WSL2 x86-64 | 35/35 passed | 35/35 passed |
| QEMU user-mode Linux AArch64 | 35/35 passed | 35/35 passed |

All six `cargo test --locked --offline` configurations exited zero: **210 total successful test executions**. Final Windows Qualification `cargo clippy --all-targets -- -D warnings` passed in both modes, as did source-owned Polynomial Clippy. Complete native Windows and WSL2 Polynomial source suites passed both modes. The new three-test source regression independently passed QEMU Linux AArch64 in both modes. Source-owned warning-denied rustdoc passed both modes. Windows and WSL2 `verify_graph.py` checked the identical committed source graph, Cargo.lock, original independent mathematical vectors and original SymPy oracle; the new consumer test and benchmark fixture SHA-256 values also matched. Independent CPython 14-vector checks passed on Windows and WSL2 with identical results. `cargo tree --locked --offline -d` reported no duplicate packages.

The acceptance boundary remains unchanged: local execution evidence for these exact configurations only; neither physical ARM64 validation, externally billing-blocked hosted CI, formal milestone approval nor release is established.
