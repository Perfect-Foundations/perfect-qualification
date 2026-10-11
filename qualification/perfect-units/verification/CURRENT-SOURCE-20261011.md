# Independent Perfect Units M1 consumer — October 11, 2026

## Exact source graph

- `perfect-units@2fead7599f0f44dbc3ea7612c9b1db32330d8852`
- `perfect-rational@97cce0ae8ff7ef206929d273e1d4a5774f6a2a34`
- `perfect-arithmetic@1a54d3c7cbbae4e73325cc70fd2777a2427b1504`
- `perfect-numeric@19b6747cd852a47a694a020b97ba70b6b3ef259b`
- `typenum=1.19.0` is the explicit production type-level dimension dependency, **not** a second rational-number implementation.

## Independent mathematical oracle

`scripts/reference.py` uses only Python standard-library `fractions.Fraction` to derive exact arbitrary rational conversions; checked-in reference corpus is immutable after first generation, and each normal invocation recomputes and verifies exact bytes.

- `vectors/linear_v1.tsv`: 40 conversion cases for km/m and ms/s, SHA-256 `5664dd7570ad1c8e3359ebbe4e4098d40b73301ebaa6e127cdca3de75a3317c1`.
- `vectors/affine_v1.tsv`: 20 Celsius/Fahrenheit point cases at exact fractional inputs, SHA-256 `6463880c269a36320502acdcdbb5228f954ae506dce052a530ca03f3ed90b755`.
- Public consumer tests: exact conversion/roundtrips, derived force/velocity, runtime dimensional mismatch, zero divisor and temperature difference with **scale but no point offset**.
- `verification/COMPILE-FAIL-M1.md` is executable rustdoc `compile_fail` test input, included from `src/lib.rs`; it proves the Rust type checker rejects length+time, point+point, point multiplication in this exact Git consumer.

## Executed local verification

Native Windows x86-64 and WSL2 Linux x86-64, Rust 1.99.0, no-default/all-features `cargo test --locked --offline` and `cargo clippy --locked --offline --all-targets -- -D warnings` passed (three integration tests/configuration, 12/12 independent integration test executions). `cargo test --locked --offline --doc` executed and passed all three compile-negative cases on both systems (6/6 executions). Standard-library Fraction oracle scripts and hashes passed both hosts. Private Git sources were the exact same pinned commits; WSL Git objects were loaded into cache without modifying the original Git-manifest dependency graph.

Primary standards provenance: BIPM SI Brochure 9th ed. version 4.01 (June 2026) for SI; NIST SP 811 Appendix B.8 for temperature conversions. No formal M1/Q6 qualification, physical ARM64, package-release, or host CI claims.
