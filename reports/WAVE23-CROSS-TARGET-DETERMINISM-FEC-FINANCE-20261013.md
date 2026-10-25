# Wave 23 — canonical FEC / Finance cross-target numerical transcripts

Production: Error Correction `1b5977267c19fa58f7ab4d2fefac25c75882a012`; Finance `2bd884a9d406dd1036504127dc768c6777790e13`. Independent Qualification PR #12 extends exact source `94ac9d5d74d6fccd21b586e5dcb2d72f4762d81a`. The source pins, Cargo.lock, and all frozen mathematical fixtures remain unchanged.

## Fixed-input numerical computations
`qualification/perfect-error-correction/tests/wave23_canonical.rs` constructs a GF(7), n=6/k=3 Reed–Solomon code, independently checks 48 evaluation codewords, corrupts each at one known coordinate, verifies exact reconstructed codewords and messages, and emits 48 `CANONICAL|RS|` ASCII lines with actual decoded ordinals. SHA-256 of the LF-separated lines with final LF:

`3668cfb5f1a9654d2d7c66a9a21dab579dfe2f9427ad21abf93c4474b77cf335`

`qualification/perfect-finance/tests/wave23_canonical.rs` constructs 48 exact EUR/USD dated ledgers, validates exact currency totals against independently calculated integer sums, and emits 48 `CANONICAL|FIN|` ASCII lines with actual Decimal amount values. SHA-256 under the same rule:

`debc613b9805c8fedbbb13a6d232e12b8aa934f2a316b5147e5b1e86ae7fb6fc`

Both digests matched **byte-for-byte** on Windows Native x86-64, Ubuntu WSL2 Linux x86-64, and local AArch64 Linux user-mode QEMU using Rust 1.99 exact Git dependencies and `--locked --offline --all-features`. All three platforms also passed the no-default feature tests. Windows and WSL2 warning-denied Clippy in both feature modes and Windows formatting passed.

The checks do not establish any wire-protocol serialization, physical ARM64 hardware validation, universal mathematical correctness, formal M1/Q6, paid hosted CI, or release readiness. All execution remained local with $0 additional spending.
