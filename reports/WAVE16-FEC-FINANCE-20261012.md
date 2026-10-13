# Wave 16 — FEC and Finance exact qualification

Pins: Algebra 20b2f10ae3390808d412be84c7af76ffbcb2766d;
FEC f1d43b155c70c9d30484038f11b211a34d9ee1d4;
Finance 1fa0eefd1dc2e940a8a5afbe79360d24f8e93c69.

Existing 325 frozen independent mathematical rows remain unchanged.
A GF(17) polynomial-evaluation corpus now also exercises caller-owned
RS encode/syndrome buffers and transactional malformed-input refusal.
A 1024-flow exact Finance narrow-range corpus is checked against an
independently regenerated Python Decimal oracle:
EUR -7999999.936, GBP -8999999.928,
JPY 9000000.063, USD 9000000.063.

Windows both-feature exact-source tests, warning-denied Clippy and
source/lock/math checks passed. Final detached-checkout verification
is separately gated.

FEC 15 alternating Windows source comparisons, two errors GF(16):
1541 to 1070 allocations, median 465100 to 394500 ns;
scoped peak increased 24544 to 32240 requested bytes.
Retained local evidence: _wave16_fec_inplace_interleaved_stats.json.

No M1/Q6 milestone promotion, hosted CI, release or branch merge.

## Finance source correction after final benchmark

The earlier Finance source `b451e1968713d4076f322caae714cce9c2bee686`
passed mathematical tests but was **rejected**, then reverted using
forward-only Git history. The exact restored production source is now
`1fa0eefd1dc2e940a8a5afbe79360d24f8e93c69`.
The accepted Finance implementation retains the already-qualified
transactional caller-owned ledger buffers; no new range-index selection
algorithm remains in production.

The identical 1024-flow harness, 15 alternating samples per revision,
found 2.3% and 0.5% lower medians for repeated aggregation and currency
queries but no allocation reduction. The 16384-flow harness, 10
alternating samples, found just 0.6% lower medians for both warm queries,
with more allocations and requested bytes for currency totals and an
approximately 6% slower cold query. This did not justify the additional
index/state complexity. The original tests still verify that the
restored source returns exact frozen and independently generated
results; tests are not evidence that the rejected optimization survives.
The full rejection rationale and sample paths are recorded in
Perfect Finance's `docs/M1-WAVE16-RANGE-INDEX-REJECTION-20261012.md`.

Both Qualification consumers must be rechecked at the final exact source
revisions on clean Windows and WSL2 checkouts. No M1/Q6 promotion,
hosted CI, release, or hardware-runtime acceptance is claimed.
