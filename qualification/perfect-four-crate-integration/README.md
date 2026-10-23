# Phase 1 ? five-foundation exact Git integration

This is the **single existing integration harness**, retained at its historical
perfect-four-crate-integration path rather than duplicating framework infrastructure.
It now consumes all five published Phase 1 foundations exclusively through exact Git
revisions, with no source path overrides:

- Numeric: 19b6747cd852a47a694a020b97ba70b6b3ef259b (private qualified revision).
- Arithmetic: 95e8499f458d9cb681ce0d452cade1cd17c9f6ad.
- Rational: 21a7fba5f22ab7f5db12b480a1c910edd1797d77.
- Float: e46b68ed20ec232be08e6c9f968ab2bc85db0736.
- Decimal: 0b4766a015706bcb77c1c54d2ed3a552e3e8f933.

The test-only crate is never a production dependency. It verifies selected public
API contracts, canonical rational/decimal value composition, finite Float conversion,
rounding and the absence of duplicate Arithmetic package identities.

From this directory, run:

    cargo tree --locked --offline -i perfect-arithmetic
    cargo test --locked --offline --no-default-features --all-targets
    cargo test --locked --offline --all-features --all-targets
    cargo clippy --locked --offline --all-features --all-targets -- -D warnings
    cargo fmt --all -- --check

Both feature suites passed locally on Zen Windows MSVC with Rust 1.99.1.
This is **not** source-exact Q0-Q6 release qualification for five crates.
It does not replace fuzz/mutation, MPFR/competitor benchmarks,
memory/size, Linux or ARM64 native runtime, independent validators,
packaging, licensing, or publication gates. Historical qualification
evidence is not automatically transferred to the new Git graph.
