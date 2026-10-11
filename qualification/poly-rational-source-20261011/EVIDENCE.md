# Source-exact Arithmetic M4 / Rational M5 / Polynomial integration

Executed 2026-10-10/11 America/New_York; final Qualification evidence for the authorized draft experimental source graph. This current consumer was updated from the previous graph in Git history (parent `817649d`); the independent historical `fused-git-graph-20261010` consumer is unchanged.

## Exact source revisions

- Numeric: `19b6747cd852a47a694a020b97ba70b6b3ef259b`
- Arithmetic: `65d8f55641739d5d6a11c17fe7d68c8898d984d7`
- Rational: `8bd86f328850c8153537842947b161ed7e3f0d24`
- Polynomial: `9a073971392c6caae1238337fb6c10afef11cbf0`

`source-exact.json` and `verify_graph.py` require the exact source Git identities, normalized Cargo.lock SHA-256, ten unchanged imported independent test-source SHA-256 values, SymPy fixture SHA-256, and current cross-operation test-source SHA-256. Cargo graph is `--locked --offline`; `cargo tree -d` shows no duplicate source identity. No `[patch]`, source override, new clone, worktree or replacement consumer.

## Final source-exact integration test matrix

| Execution environment | No default features | All features |
|---|---:|---:|
| Windows x86-64 | 31/31 passed | 31/31 passed |
| Ubuntu WSL2 Linux x86-64 | 31/31 passed | 31/31 passed |
| QEMU user-mode Linux AArch64 | 31/31 passed | 31/31 passed |

All six `cargo test --locked --offline` executions returned zero. Each runs 25 frozen independent mathematical checks and six current cross-operation tests. The added cross-operation regression has independent CPython Fraction/integer results for signed unit Rational multiplication, integer-valued multiplication, Horner evaluation at 5/3, quotient/remainder by a signed constant and complete polynomial coefficients. The other two check exact mixed-denominator cross-operations, wide canonical zero normalization, polynomial GCD and fused Integer cancellation.

Windows all-target `cargo clippy -- -D warnings` passed both feature modes for the final Qualification consumer; Windows full Arithmetic, Rational and Polynomial source test/lint matrices also passed, and Rustfmt checked. Ubuntu WSL2 complete Rational and Polynomial source suites passed both feature modes at the final pinned revisions; Arithmetic M4 complete WSL2 source suites were previously executed against unchanged `65d8f55`. New changed-operation focused tests passed on QEMU in both feature modes when separately executed prior to this source graph; this final 30-test consumer additionally passed QEMU both modes.

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
