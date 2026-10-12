# Wave 16 — FEC and Finance exact qualification

Pins: Algebra 20b2f10ae3390808d412be84c7af76ffbcb2766d;
FEC f1d43b155c70c9d30484038f11b211a34d9ee1d4;
Finance b451e1968713d4076f322caae714cce9c2bee686.

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
