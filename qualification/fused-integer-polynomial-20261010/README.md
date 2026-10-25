# Independent experimental Arithmetic × Polynomial integration

This fixture is intentionally **separate from** `qualification/perfect-polynomial-integrated`, whose conservative M1 evidence and exact source pins remain untouched. It is an independent Qualification-owned source consumer for draft Arithmetic PR #15 and draft Polynomial PR #18. **No formal M1/M2 acceptance, production dependency promotion, or release is claimed.**

## Exact source graph

See [source-exact.json](source-exact.json) for all immutable Git revisions, Rust 1.99.0, provenance blobs, feature configuration, and limitations. The current source pair is:

- Arithmetic: `73c3e6f30278556cda090b8d5cce2ed2d2a422c1`
- Polynomial: `ce89bad2b80a2c887f91a912a36a78c80303ba1c`
- Rational: `35a8e9cc629ee578fe7b624e2134929ce7eeff8a`
- Numeric: `19b6747cd852a47a694a020b97ba70b6b3ef259b`

The source-exact replay clones Arithmetic and Polynomial in an existing empty scratch directory with known relative names. The committed `consumer/Cargo.toml` uses only those **relative** adjacent directories and a qualification-only Cargo path patch for the original Arithmetic Git URL. The independent consumer has a **retained `consumer/Cargo.lock`**, and Polynomial's lockfile Git blob is checked against the qualification manifest. `cargo metadata` and `cargo tree` assert exactly one Arithmetic package, consumed through three paths: Qualification → Arithmetic, Qualification → Polynomial → Arithmetic, and Qualification → Rational → Arithmetic. Invalid same-source Git patches are not used.

## Reproduce

From this directory, with Python 3.9+, Git read access to the private repos, Cargo/Rust 1.99.0 and an **existing empty** destination directory:

```sh
python3 run_fixture.py --workdir /path/to/empty/scratch
```

The wrapper verifies its vendored replay script's Git blob provenance and both source SHAs, runs Polynomial's locked feature matrix, independently checks Cargo.lock provenance, then materializes **this** Qualification-owned consumer and runs its all-feature/no-default tests, rustfmt, warning-denied Clippy, and warning-denied rustdoc. This fixture refuses implicit local source overrides, duplicate Arithmetic package identities, or Cargo.lock drift.

`consumer/tests/independent_math.rs` contains **separate** exact Python Integer/Fraction and SymPy GCD reference coefficients: full signed Z[x] quotient and three typed errors; full signed Q[x] quotient and nonzero remainder; 128–8192-bit exact Integer carry/cancellation; and high-degree Q/Z[x] GCD with exact integer content. The source crates' own M2 suites also run through the replay. All tests use the same source revision and no network-dependent mathematical oracle.

## Evidence and acceptance

Windows x86-64 fresh network clone of the exact source pair passed Polynomial all-feature/no-default, warning-denied Clippy, rustfmt and rustdoc **plus** this separate Qualification consumer's four independent mathematical tests, both features, Clippy, rustfmt, rustdoc and unchanged lockfiles. Ubuntu WSL2 x86-64 uses fresh independent local-filesystem clones sourced from the SHA-verified Windows GitHub checkout; it independently runs the consumer test matrix. This is cross-OS execution, **not** an independent WSL2 network authentication test or physical ARM64 execution.

Polynomial PR #18's own exact source evidence records Integer exact division 64/64 (128-bit) allocator calls **8,385 → 4,163**, requested bytes **211,880 → 110,600**, Windows three-run mean **491.3 → 289.7 μs**. This fixture does not treat benchmark-only activity as mathematical acceptance. The Polynomial source also contains SymPy exact GCD and rational division references; no qualification percentage is increased.

The current Arithmetic hosted CI runner failed before executing any job steps; see [CI-RUNNER.md](CI-RUNNER.md). Independent reviewer approval, portable *production* dependency contract, native ARM64 runtime, complete family qualification, licensing/publication and formal milestone gates remain open.
