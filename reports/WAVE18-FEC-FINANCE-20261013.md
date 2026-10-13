# Wave 18 — Exact FEC and Finance consumer qualification

Production source graph:
- Perfect Algebra `20b2f10ae3390808d412be84c7af76ffbcb2766d` (unchanged).
- Perfect Error Correction `f08e8a5435d9a067558fb69009bb618bfa948fb3`.
- Perfect Finance `82c6c71ae7b94942ceb47259dcc1f995b4147ba3`.
- Lower-level dependencies remain pinned as in `verification-wave3-graphs.py`.

Error Correction public consumer test `tests/wave18_direct.rs` uses
the existing **independent frozen** GF(17) n16/k12 polynomial-evaluation
and GF(16) reference codewords to verify reusable transactional decode
buffers, owned decoding sessions, clean/unknown-error/erasure/mixed
recovery, corrected positions, repeated successful calls, bound admission
refusal, invalid-input refusal and next-call recovery. It also verifies
bounded direct batch decoding, prior-output preservation and per-word
outcomes. Existing Hamming, profile and field tests remain.

Finance public consumer test `tests/wave18_owned.rs` reuses the existing
14 independent frozen date-range mathematical reference rows to test
the owned stable-ID ledger and bounded multi-range exact cash-flow
aggregation. It verifies grouped output equivalence with independent
standalone consumers, preserved arithmetic/quantum, leap dates,
explicit range boundaries, repeated query buffers, strict resource
admission, ID continuity and transactional invalid updates.

No new frozen mathematical reference rows are claimed; the tests
reuse existing independent oracles with new public API contracts.
Original corpus files and their recorded SHA256 fingerprints are
unchanged. The graph script also independently regenerates GF(7)
Berlekamp-Welch, packed Hamming, currency and date-range corpora.

Windows Native Rust 1.99 source worktree verification passed for both
consumer crates: locked/offline tests and -D warnings Clippy under both
no-default and all-feature configurations, rustfmt, exact graph/lock
verification and independent regenerators. Replays on final clean
detached Windows and Ubuntu WSL2 checkouts are separate gates that
must be recorded after the final Qualification commit.

Measurement evidence for the FEC implementation at source
`089a1158b74ca8f3b8b0e2195cc192dfbc3bad76` is preserved
in the production repository report (production code is unchanged
at the subsequent `f08e8a5` commit). Fifteen alternating release
samples versus `df3f06e` reduced GF(16) two-error scoped active peak
from 32240 to 28912 requested bytes and GF(17) from 4816 to 2944,
with small latency improvements. Scoped allocator counters are
not operating-system RSS.

Formal M1/Q6 approval, physical ARM64 hardware runtime, hosted CI,
crates.io release and protected branch merge remain unearned.
