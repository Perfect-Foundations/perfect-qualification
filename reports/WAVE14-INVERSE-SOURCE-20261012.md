# Wave 14 — Exact downstream inversion source qualification

`perfect-algebra` pinned:
`6a2d4924466ebeab79b849f56e6d9a8b9acfdf24`.
`perfect-error-correction` pinned:
`55a5fd368f008dd8b8bbbea1751829d5e1dc99aa`.
`perfect-finance` pinned unchanged:
`1752f18f9d17fc59a8440dcb48abb9f3d9f986f3`.

Upstream extended-Euclid Bézout polynomial update now performs a
single-buffer exact subtraction/product. Error Correction production
only updates its exact source dependency: decoder algorithm, RS limits,
workspace, Hamming/SECDED behavior and public interfaces are unchanged.

The existing independent GF(16) evaluation corpus is exercised
through correctable double errors and mixed errors/erasures, repeatedly
with a reusable decoder workspace. Independent GF(17), Hamming,
Finance exact day-count/Decimal and cross-date aggregation consumers
are unchanged, as are the **325 frozen independent Finance/FEC reference
rows**. Test injections are not additional frozen oracle rows.

Wave 13 pinned source (15 Windows release samples) vs Wave 14
(15 samples), GF(16), n8/k4 2 errors/reused workspace:
allocations 1571 -> 1541; allocator requested bytes
501416 -> 488312; median elapsed 543800 -> 523300 ns.
Scoped peak requested 24544 bytes unchanged.
GF17 n16/k12 two-error allocations 9 -> 9, median
458500 -> 459900 ns (small latency regression).
No RSS/stack claim; benchmark measurements are sequential
and are not general hardware performance guarantees.

M1/Q6 qualification applies only to replayed full SHA graph and
tested configurations; release and physical ARM64 runtime remain open.
