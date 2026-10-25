# Wave 23 — canonical integer-polynomial output determinism

Production: `Perfect-Foundations/perfect-polynomial@d3094041c544913da56eabe5b8db35cc5a695b32`.
Independent Qualification PR #10 extends source `dd2f97f3e71709f36e86bdff07867badfa5241a3` without changing its dependency pins, Cargo.lock, or mathematical reference fixtures.

## Workload and digest
`qualification/poly-rational-source-20261011/tests/wave23_canonical.rs` produces 48 ordered ASCII records beginning `CANONICAL|POLY|`. It computes public integer-polynomial multiplication, derivative, and evaluation at x=2 using a fixed xorshift seed. All coefficient values are converted through checked `Integer::try_to_i64().into_value()` rather than Rust Debug or platform-dependent storage.

SHA-256 of the exact 48 UTF-8 lines joined with LF and a final LF:

`2308861b9d829052b4c4713570c6702c8a8fe7c23aa723a89a330df50affa933`

Windows native x86-64, Ubuntu WSL2 Linux x86-64, and QEMU AArch64 Linux user-mode all produced **the identical 48-line digest** using Rust 1.99, exact pinned dependencies, `--locked --offline --all-features`. All three also passed this public consumer's no-default test configuration; Windows/WSL passed warning-denied Clippy in both feature configurations and formatting checks.

This is limited fixed-input cross-target output equality, not an API-level canonical serialization format, proof of every input, physical ARM64 hardware, separate machine hardware, formal M1/Q6 acceptance, hosted CI, merge or publication. All work used existing resources with $0 additional spending.
