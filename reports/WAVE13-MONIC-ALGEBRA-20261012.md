# Wave 13 — exact monic extension-multiplication graph qualification

M1 candidate pins in this Qualification revision:

- Perfect Algebra: `16831b3cde94f29f18b3e62304fb6e050d7cfcc5`.
- Perfect Error Correction: `a9f6189d0dfcc29631216ee403d4439b164120e0`.
- Perfect Finance (unchanged): `1752f18f9d17fc59a8440dcb48abb9f3d9f986f3`.

All are full remote Git source revisions; tracked consumer lockfiles must
resolve the same single transitive Algebra source. No path overrides.

The independent GF(16) Reed–Solomon polynomial-evaluation corpus is
consumed again in the batch-workspace interface: each of the four frozen
GF(16) rows is checked for exact encoding and repeated batch decoding
under two unknown errors or a mixed error/erasure pattern.
The independent GF(17) n16/k12 rows and complete Finance reference
suite remain tracked and unchanged. Across these two Qualification
consumers the **325 immutable frozen mathematical reference rows** are
unchanged; injected cases and workload repetitions do not count as
new independent mathematical rows.

Independent Algebra/Units Qualification PR #11 is repinned separately,
using existing 369 frozen Algebra rows plus new bit-polynomial
reference tests for GF(16) and GF(256).

Previously measured 13-run baseline vs 15-run downstream candidate:
GF(16), n8/k4, two-error reused workspace: allocations 1717 to
1571; total requested heap 524088 to 501416 bytes; median latency
642200 to 543800 ns; MAD 22300 to 9100 ns. Scoped peak stays
24544 requested bytes. The full sample data is retained locally
in the named Wave 12/13 benchmark logs, not claimed as physical RSS.

Qualification tests/Clippy, exact SHA/lock checks, independent
mathematical regeneration, clean detached replay and release gates
must remain separately evidenced; implementation and benchmark
improvements alone confer no M1/Q6 promotion.
