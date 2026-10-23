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

## Additional primitive differential matrix (2026-10-23)

The existing test-only five-crate consumer now includes tests/phase1_primitive_differential.rs. An independent bounded-i128 fractional oracle supplies 6,627 expected signed Decimal add/subtract/multiply results across mixed base-10 exponents. Another 3,800 signed Decimal/Rational/Float consistency cases use all five common rounding modes. The latter are cross-crate invariants, not an independent MPFR correctly-rounded Float proof.

Windows Rust 1.99.1 exact-source locked/offline full no-default and all-features test suites, warning-denied Clippy in both configurations, Rustfmt and offline no-default documentation passed. Earlier cross-target checks were compile-only. A fresh WSL2 source-exact Qualification checkout could not fetch the private Arithmetic source because GitHub authentication was unavailable to Cargo there; no Linux test result was obtained.

This is additional integration evidence only. It does not advance Rational M5, Float M4, Decimal M1, Arithmetic Q6, Q0-Q6 family qualification, or the private R7A/public R7B release gates.
