# Wave 12 source-exact downstream qualification — 2026-10-12

This is evidence for **draft, unmerged** development candidates, not M1/Q6 formal acceptance or release authorization.

## Exact production source graph

- Perfect Algebra PR #3: `6d4eac0ebaab2417720304ba2739089120db8fa4`.
- Perfect Error Correction PR #2: `14f0d11ac3424c68158812511d6c4cac13a90ef1`, consuming the exact Algebra revision above.
- Perfect Finance PR #2: `1752f18f9d17fc59a8440dcb48abb9f3d9f986f3`.
- Upstream Algebra/Units independent Qualification PR #11: `99f8a08e9b0317476cab239bc27f3f895257f1d0`.
- This downstream Qualification PR #12 repins **both** production crates to the exact full Git SHAs, with tracked Cargo.lock files. `verification-wave3-graphs.py` checks source identities, lockfiles and immutable frozen vectors.

The optimized Algebra parent shares immutable extension moduli on targets supporting pointer atomics and retains the owned representation elsewhere. No second finite-field authority or path-substituted dependency graph is introduced.

## Public independent consumer coverage

- `tests/rs_batch.rs` uses the existing independently calculated GF(17) 16-symbol reference corpus and the committed RS batch API to test clean, two-error, erasure-only, mixed, workspace reuse, batch count/input/result limits and per-word malformed-data isolation. It does **not** count repeat batches as new mathematical reference rows.
- `qualification/perfect-finance/tests/batch.rs` retains its existing standalone reference checks and adds 14 previously frozen independent date-range oracle comparisons against the new caller-owned batch result methods. It tests output atomicity on mathematical failure and continued use after a validated reset.
- All prior independent mathematical corpora remain unmodified. No artificial vector-count promotion is claimed.
- A separate untracked historical `qualification/wave6-performance/` folder was intentionally **excluded** from this commit, including its generated `target/` directory.

## Executed acceptance on the exact source pins

Windows Native x86-64, Rust 1.99:
- `cargo fmt --all -- --check` passed for both consumers.
- `cargo test --locked --offline --no-default-features` passed for both.
- `cargo test --locked --offline --all-features` passed for both.
- `cargo clippy --locked --offline --all-targets --no-default-features -- -D warnings` passed for both.
- `cargo clippy --locked --offline --all-targets --all-features -- -D warnings` passed for both.
- `python verification-wave3-graphs.py`: exact source, Cargo.lock and frozen vector checks passed.

Ubuntu WSL2 x86-64, Rust 1.99:
- Identical locked/offline test and warning-denied Clippy matrices passed for both consumers with **separate Linux** target directories.
- Independent oracle generation and exact source/lock/vector graph checks passed.
- The private WSL2 Cargo Git object cache initially lacked the three newly pinned revisions. The exact committed Git objects were fetched from the matching verified local repositories into the private cache with `git fetch --no-tags <local Git repository> <full SHA>`. No Cargo.toml dependency, Cargo.lock or source path substitutions were used.

This verifies two operating systems on **one physical computer**; no physical ARM64 runtime, hosted CI, formal independent review or release approval is claimed. Production cross-target checks reported in Wave 12 are compile-only.

The final detached-worktree replay of the pushed Qualification SHA is recorded separately from this precommit matrix.
