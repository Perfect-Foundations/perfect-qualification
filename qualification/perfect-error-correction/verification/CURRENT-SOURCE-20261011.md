# Independent Perfect Error Correction qualification — October 11, 2026

**Pinned Git production source**: Error Correction `02594e730c156ed0e5f585ba0e5d348d6240e6fd`, Algebra `8c8c0794f6098db1545e4ca18a6bfb38e2e822f2`, Number Theory `2bdd2eee53615acb5beac8720c1d2be62120c080`, Arithmetic `9ec660f9abfd7fe93e441e77dbb1158c371603f5`, Numeric `19b6747cd852a47a694a020b97ba70b6b3ef259b`. No branch revisions or path overrides in production dependencies. Cargo.lock tracked intentionally, byte-checked.

**Independent reference**: `scripts/reference.py` Python stdlib integers (not Rust production APIs) produces immutable ASCII little-endian logical Hamming(7,4)/SECDED(8,4) bit strings for all 16 messages, 25 GF(5) polynomial-evaluation RS(4,2) vectors, and four GF(16) RS(8,4) vectors using Python GF(16) x^4+x+1 reduction and exact polynomial evaluation. Fixed corpora:
- Hamming 16, SHA256 `5ddef5b87b2b994ced47a86a2f3c4b88db09f17603b54254866aa05c6c24cdf6`.
- GF(5) RS 25, SHA256 `07b62970fc79b24fddc9a2494228c180f0a4600b75fe791db53db807683408e0`.
- GF(16) RS 4, SHA256 `b16458ba91078c41d6c00d617dba5399014d49168322cd0303deba802b1e3ddb`.

The public consumer tests exact codewords, exhaustive SECDED one/two-bit injection and correct status, GF(5) hard-error correction, and GF(16) double-hard-error and one-error/one-erasure patterns. The initial draft tested 2 hard errors + one erasure with four parity symbols, violating `2e+v<=4`; this fixture was corrected to a within-guarantee pattern before acceptance, not used to relax the production contract.

**Executed**: Windows x86-64 and WSL2 Linux x86-64, Rust 1.99.0: both `--no-default-features` and `--all-features`, source-exact consumer `cargo test --locked --offline`, warning-denied all-target `cargo clippy`, recomputed Python mathematical corpus. Three integration tests per host/feature = 12 passing test executions, zero failures after harness correction. Four configurations for fixed production revision; absent physical ARM64/hosted CI. No formal release/M1 promotion.
