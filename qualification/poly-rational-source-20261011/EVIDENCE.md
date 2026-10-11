# Source-exact Arithmetic M4 / Rational M5 / Polynomial integration

Executed 2026-10-10/11 America/New_York; final Qualification evidence for the authorized draft experimental source graph. This current consumer was updated from the previous graph in Git history (parent `817649d`); the independent historical `fused-git-graph-20261010` consumer is unchanged.

## Exact source revisions

- Numeric: `19b6747cd852a47a694a020b97ba70b6b3ef259b`
- Arithmetic: `65d8f55641739d5d6a11c17fe7d68c8898d984d7`
- Rational: `8bd86f328850c8153537842947b161ed7e3f0d24`
- Polynomial: `815f6a2ae3fb58407b28d6a84c781a82add9433c`

`source-exact.json` and `verify_graph.py` require the exact source Git identities, normalized Cargo.lock SHA-256, ten unchanged imported independent test-source SHA-256 values, SymPy fixture SHA-256, and current cross-operation test-source SHA-256. Cargo graph is `--locked --offline`; `cargo tree -d` shows no duplicate source identity. No `[patch]`, source override, new clone, worktree or replacement consumer.

## Final source-exact integration test matrix

| Execution environment | No default features | All features |
|---|---:|---:|
| Windows x86-64 | 30/30 passed | 30/30 passed |
| Ubuntu WSL2 Linux x86-64 | 30/30 passed | 30/30 passed |
| QEMU user-mode Linux AArch64 | 30/30 passed | 30/30 passed |

All six `cargo test --locked --offline` executions returned zero. Each runs 25 frozen independent mathematical checks and five current cross-operation tests. The added cross-operation regression has independent CPython Fraction/integer results for signed unit Rational multiplication, integer-valued multiplication, Horner evaluation at 5/3, quotient/remainder by a signed constant and complete polynomial coefficients. The other two check exact mixed-denominator cross-operations, wide canonical zero normalization, polynomial GCD and fused Integer cancellation.

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
