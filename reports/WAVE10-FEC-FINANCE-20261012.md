# Wave 10 — exact new M1 source and independent reference evidence

This report records implementation/qualification evidence, NOT formal M1/Q6
acceptance, merge, release, standard wire compatibility, or independent
physical hardware qualification.

| Consumer | Exact production revision |
| --- | --- |
| Perfect Error Correction PR #2 | c25e906ebbd608391706b25307d0bedfaab80a3d |
| Perfect Finance PR #2 | 63f704c68e1567991979fff1061266946285d153 |

The previous Qualification base was
7f5608ee9bf4e948e427ec446c5ff6fdb2283a46. The commit containing this
record is the new Qualification source revision. Cargo.toml and Cargo.lock
pin production source IDs and the independent root verifier locks their
transitive Perfect dependencies. No accepted corpus was overwritten.

## New implementation

Error Correction: immutable profile-bound typed/byte operations reject
incompatible bit orders and RS field/code parameters; packed Hamming
encoding, syndrome, and decoding operate directly on bytes without a logical
Vec<bool> intermediate; reusable RsDecoderWorkspace retains immutable
evaluation powers and matrix-row capacity between decodes, with state reset
after failures. The existing 16-symbol RS bound, max_trials semantics,
field algebra and MDS correction guarantee remain.

Finance: exact inclusive/exclusive Gregorian DateRange, rejecting reversed
dates; additive filtered dated aggregation and filtered currency netting,
using the pre-existing bounded exact Decimal reduction with borrowed flows.
Input max_flows bounds the whole input, including excluded rows. No FX,
rounding, market calendar or date-count accrual is introduced.

## Mathematical sources and counts

The existing **311** frozen independent reference rows remain unchanged.
New Python-stdlib Decimal/datetime oracle
qualification/perfect-finance/scripts/date_range_reference.py generated
**14** exact date-range currency reference rows in date_range_v1.tsv.
SHA-256:
0529ed78ea046396b8f7ca80c541711a810179b909a30be926567496cff9896e.

Total frozen independent rows = **325** (311 + 14). Existing 48 packed
Hamming rows and eight GF(17) n16 rows are reused to validate the new
profile/workspace APIs. The profile compatibility refusals, Hamming error
injections, permutations, codeword recovery checks, and Windows/Linux test
executions are NOT counted as new independent mathematical rows.

## Measured resource and runtime observations

Windows 11 Native Rust 1.99.0 release-mode benchmark sources are retained in
the corresponding production crate tests. For GF(17) n=16,k=12,
RsDecoderWorkspace retained 512 Vec field-element capacity slots
(powers + matrix rows), not a byte-accurate heap total.

Source-level same-binary warm benchmark examples:
- clean codeword: 132,640 ns normal decode vs 120,187 ns reused workspace;
- two unknown errors: 528,772 ns normal vs 514,918 ns reused;
- four erasures: 124,407 ns normal vs 111,672 ns reused.
These are single-run, not statistically certified improvements.

Hamming(255,247) reference comparisons: packed encode 4,115 ns versus
logical-bool encode 1,074 ns; packed syndrome 119 ns versus logical 48 ns;
packed decode 319 ns versus logical 306 ns. The bool operation presupposes
already unpacked input; these numbers do not establish end-to-end packed
transport speedup. Direct packed operations reduce intermediate Vec<bool>
allocation but exact allocation counts/bytes were not instrumented.

Finance release-mode scaling covered 16,64,256,512,1024 flows with
same-date and distributed date/currency workloads, including cancellation.
For 1024 same-date/currency flows, approximately 1.58–1.60 ms for original
and filtered grouped aggregation in one run. No statistically established
performance improvement or byte-accurate peak heap claim is made.

Both production sources passed Windows and Linux x86-64 locked/offline
Rust 1.99 tests, both features, formatting and warning-denied Clippy.
Windows warning-denied rustdoc and compile-only checks on
wasm32v1-none, thumbv7em-none-eabihf and riscv64imac-unknown-none-elf
passed. Those do not prove physical runtime compatibility.
Final pinned Qualification clean-checkout results are recorded on PR #12
after executing that exact committed source. Windows and WSL2 share Zen
hardware. No lifecycle gate percentages were promoted.
