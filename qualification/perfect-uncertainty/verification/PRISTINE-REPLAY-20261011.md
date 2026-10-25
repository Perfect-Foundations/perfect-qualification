# Source-exact pristine checkout replay — 2026-10-11

## Frozen source identities

- Qualification detached Git checkout: `a0580b1d7232acda66ed0bccf45a9810d2461193`.
- Source: `Perfect-Foundations/perfect-uncertainty@1ca644f946cb9adea9f817675a464a5800713b25`.
- Qualification tracked lockfile SHA-256:
  `18423d1e5697ff56d350f2246a0aeb1b59ddfb901e974f3467e22ce6210c0552`.
- Explicit dependency set: exact Git source and `libm=0.2.15`.
- No path overrides, fabricated registry identities or dependency substitutions.

## Method

Created a separate detached Git worktree from the exact remote Qualification
PR #13 commit at a previously nonexistent local path. Its
`git status --porcelain=v1` was empty after all verification commands.

A reused external Cargo target directory provided build-cache reuse only:
no generated target files were placed in the pristine source checkout.
The exact source manifest, lockfile and files were compiled.

Both independent Python Fraction oracle generators and the root graph
verifier were run directly from the pristine checkout; the three frozen
LF vectors verified byte-for-byte with their previously recorded SHA-256.

## Executed results

**Windows x86-64 and Ubuntu WSL2 Linux x86-64**:

- `cargo test --locked --offline --all-features`: six integration tests PASS.
- `cargo test --locked --offline --no-default-features`: six PASS.
- `cargo clippy --locked --offline --all-targets --all-features -- -D warnings`: PASS.
- `cargo clippy --locked --offline --all-targets --no-default-features -- -D warnings`: PASS.
- Python independent Fraction mathematical reference: 263 exact cases PASS.
- Source Git SHA / Cargo.lock / SHA-256 vector verifier: PASS.

Total: **24/24 independent Rust test executions** across four
host-feature configurations; zero failed. The reference vectors are
263 *mathematical input cases*, not 263 Rust test functions.

The same Git object had been imported into the private WSL2 Cargo Git cache
because authenticated live Git fetch was unavailable; offline checkout was
performed using the exact full Git revision without path overrides.

This is a source-exact clean-checkout replay, not hardware-independent
qualification: Windows and WSL2 run on one physical x86-64 computer.
No physical ARM64, hosted CI, independent security signoff, formal M1/Q6
promotion, protected merge, library release or Perfectπ change is asserted.
