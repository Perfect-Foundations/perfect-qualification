# Independent Uncertainty qualification evidence — 2026-10-11

Source: `Perfect-Foundations/perfect-uncertainty` Git SHA
`d9e82a6e4f510903fe0125cae91fa0d8fa00d4e9`.
Production dependency exclusively on this pinned exact Git revision;
only additional transitive dependency `libm=0.2.15`. The independent
consumer retains a lockfile and disallows an implicit path override.
Independent Python standard-library Fraction reference computes the
quadratic form via diagonal sum plus 2×off-diagonal pairs, unlike
production row-major compensated binary64 accumulation.

Frozen corpora (ASCII LF exact SHA-256):
- `gum2_v1.tsv`: 63 pairs; `c6e23aaeaa5a6d20ac53a47ce32201d8f2544921b429edb07589593b312dd5c4`.
- `gum3_v1.tsv`: 40 triples; `dae756ec8b8578ff89300cd6246b87a4d3c8702ea190201b19db33aee8e8668a`.

103 exact mathematical reference vectors; Rust first-order variance and
square-root uncertainty checked within absolute/relative
`5e-12 * max(1, expected)`, with caller-selected PSD tolerance `1e-12`.
Additional source-exact consumer tests exercise singular/perfect correlation,
negative PSD, symmetry/diagonal/finite violations, source size limits,
nonfinite propagation, scalar standard uncertainty and explicit expansion
factor without implied probability.

Observed Windows native x86-64 Rust 1.99 and WSL2 Linux x86-64:
both `--no-default-features` and `--all-features` used
`cargo test --locked --offline` and
`cargo clippy --locked --offline --all-targets -- -D warnings`.
3 integration tests per feature/host configuration passed, 12 integration
test executions total. Python independently regenerated corpus hashes on both
hosts and source/lock verifier passed. No ARM64 hardware, formal acceptance,
published GUM profile, or release claim. Windows and WSL2 are distinct hosts
but the same physical x86-64 hardware.
