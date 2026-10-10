# Independent immutable Git-pinned Arithmetic → Rational → Polynomial qualification

**This is a separate experimental consumer, not the conservative M1 consumer.** Its `Cargo.toml` contains *only* ordinary exact-revision Git dependencies; no `[patch]`, local source directory, config override, or sibling worktree is required. Cargo.lock is deliberately retained even though this repository normally ignores lockfiles. The consumer is `publish=false`.

## Frozen source identities

| Package | Exact Git commit |
|---|---|
| Perfect Arithmetic | `73c3e6f30278556cda090b8d5cce2ed2d2a422c1` |
| Perfect Rational | `53816615d28d47cbc8210efb239f05f632ecd854` |
| Perfect Polynomial | `7633fc00e96cf37b8baa731011efaa95ada8b385` |
| Perfect Numeric | `19b6747cd852a47a694a020b97ba70b6b3ef259b` |

The `source-exact.json` manifest separately records the four immutable SHAs, SHA-256 of the LF-normalized retained lockfile (Git may check it out as CRLF on Windows), independent mathematical oracles, and acceptance limitations. `verify_graph.py` inspects live Cargo metadata, asserts one Git package identity for Arithmetic, checks direct and Rational-transitive resolution, checks the Numeric source, rejects any Cargo path/patch override, and checks retained lockfile bytes. `cargo tree --locked -d` reports no duplicate packages on Windows and Ubuntu WSL2.

## Reproduction

From a clean checkout of this Qualification branch, in this directory, with Python 3.9+, Rust/Cargo 1.99.0 and permitted private GitHub repository access:

```sh
python3 verify_graph.py
cargo +1.99.0 tree --locked -d
cargo +1.99.0 tree --locked -i perfect-arithmetic
cargo +1.99.0 fmt --all -- --check
cargo +1.99.0 test --locked --all-features
cargo +1.99.0 test --locked --no-default-features
cargo +1.99.0 clippy --all-targets --all-features --locked -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo +1.99.0 doc --no-deps --locked --all-features
cargo +1.99.0 check --locked --no-default-features --target wasm32v1-none
```

The first line ensures every subsequent Cargo operation is against the intended immutable source identity. It must not be bypassed if metadata is incomplete or a local Cargo source override is present. Under WSL2, the only private Git fetch restriction was credentials. Exact Git commit objects were imported into Cargo's Git object cache from Windows SHA-verified complete Git bundles for Arithmetic, Rational and Polynomial. Cargo's declared dependencies remain immutable Git sources, not path patches. Linux was executed on x86-64, not physical ARM64.

## Independent qualification tests

`tests/independent_math.rs` contains fixed Python arbitrary-precision Z[x] quotient vectors, Python Fraction Q[x] quotient/remainder vectors, SymPy high-degree GCD and Integer content, separate primitive-PRS exact GCD and division coefficients, signed exact BigInt carry/cancellation, and typed errors. Its expected polynomial coefficients are not produced by the implementation under test. Windows and Linux WSL2 Rust 1.99.0 all/no-default tests pass; rustfmt, warning-denied Clippy/rustdoc also pass. Windows `wasm32v1-none` no-default **compile-only** succeeded.

The source PRs' broader original M2 and guard continuation tests independently pass. The fused primitive PRS reduces allocations in late-refusal GCD but **does not demonstrate a latency speedup**; see Polynomial source `docs/FUSED-PRIMITIVE-PRS-20261010.md`. Retained original conservative and fraction-free baselines remain unchanged, as does the earlier path-patched experiment.

## Qualification boundaries

This establishes a reviewable, unpatched Git dependency graph for the **experimental** source only. Independent API reviewers have not approved `Integer::sub_mul_assign` or `Integer::scale_sub_mul_assign`. GitHub Actions hosted Arithmetic jobs previously failed **before runner assignment**, with zero steps executed; no passing hosted CI is asserted. Native physical ARM64, cross-family formal gates, owner license selection, release and crates.io publication remain outstanding. No accepted M1/M2 readiness percentage or protected production pin was changed.
