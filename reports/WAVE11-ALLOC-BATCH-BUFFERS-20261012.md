# Wave 11 — allocator-guided source exact M1 engineering qualification

**Pinned production source revisions**

- Error Correction: 985176401494dda6973da05122d946b4ff3004e7
- Finance: cacc0eb40a735788245033d0679c9d8364f73545
- Starting Qualification: 23d05a5dcb3bb369f06d6a5ca949450c0b1d7848

The **325 independent frozen mathematical reference rows remain intact**.
No new mathematical reference rows are claimed: all new caller-output and
sorted-index query adapters are checked against existing independently
computed GF(17), GF(16), packed Hamming, and Python Decimal/date-range
references in qualifying consumers. Malformed buffers, rejected resets,
correction injections and host replay are test cases, not new reference rows.

New Error Correction qualification exercises exact packed bytes, noalloc
caller destination buffers, all Hamming parity sizes 3–8, SEC/SECDED,
MSB/LSB, corrected positions, and zero invalid-padding acceptance.
Existing independent RS consumer verifies reusable workspace after
successful and failed decoding and field mismatches. Production maintains
the historical error-radius and trial-budget behavior.

New Finance qualification replays the 14 frozen Python Decimal/date-range
rows through CashFlowBatch, including leap day, explicit boundaries,
deterministic grouped output and reuse after rejected reset.

**Measured source-level allocation evidence** is recorded in production
Wave 11 documents and retained test-only GlobalAlloc harnesses.
No heap allocator dependency was introduced into the libraries. For
GF(17), n16/k12 two errors, source baseline 15 allocations/9264 bytes/
7152 scoped peak vs updated workspace 9 allocations/7200 bytes/5184
scoped peak; elapsed 471.9 us baseline vs 492.6 us candidate in a
single Windows optimized test, a measured latency tradeoff.

Finance 1024 one-date/one-currency filtered aggregation baseline:
10231 allocations, 80198 bytes, 16808 scoped peak, 1.138 ms.
Indexed batch setup: 2053 allocations and 24576 retained requested
index/scratch bytes, 0.176 ms. Warm query: 8173 allocations, 53062
bytes, 8192 scoped peak, 0.987 ms. First query and setup are accounted
for separately; benchmark iteration and OS environment are limitations.
Thread-local scoped allocator accounting excludes stack and RSS; a
negative retained delta can reflect freeing previously allocated
workspace elements, and must not be used as a process memory metric.

Complete source-exact final Windows/WSL2 detached-checkout replay,
format, rustdoc, no_std, warning-denied Clippy and exact transitive graph
outcomes are recorded after the final committed Qualification SHA.
Formal M1/Q6 acceptance, physical ARM64 runtime, release and publication
are not claimed.
