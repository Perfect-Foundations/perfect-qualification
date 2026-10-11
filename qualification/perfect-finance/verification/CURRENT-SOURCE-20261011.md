# Independent Perfect Finance qualification — October 11, 2026

**Exact pinned source**: Finance `6f7540a13d29ab97387e2b0305fd3c9c2739d9c6`, Decimal `f048affde6c5ea037b793580d295254695f7e97a`, Rational `97cce0ae8ff7ef206929d273e1d4a5774f6a2a34`, Arithmetic `1a54d3c7cbbae4e73325cc70fd2777a2427b1504`, Numeric `19b6747cd852a47a694a020b97ba70b6b3ef259b`; named `time = 0.3.44`. One reviewed revision per Perfect crate, no path override. Cargo.lock tracked by exact exemption.

**Independent reference**: `scripts/reference.py` uses only Python stdlib datetime/date, Decimal and fractions.Fraction. Frozen exact corpora are regenerated in memory and byte-checked without using the Rust implementation:
- 36 leap/reversed/month-end/same-date day-count vectors for Actual360, Actual365Fixed, 30/360 US and 30E/360: SHA256 `0a113ac24472f45ed92707c43157f79c20cd1981d074f6c89c0975b82832ddc0`.
- 48 exact signed/higher-magnitude decimal quantization vectors with half-even, half-away and toward-zero rounding: SHA256 `d754eff8cefc686087dcda8429a03d4aab693e4bdf337b7b5e50d8dc27e3b76f`.

Three independent public consumer tests check all 84 vectors, quantized signed zero/quantum preservation, explicit currency-mismatch refusal, and deterministic date/currency aggregation under a resource bound. An initial draft reference-generator convention label erroneously selected US handling for the 30E/360 fixture. Corrected the **uncommitted** generator and regenerated only its not-yet-accepted fixture; underlying Finance production code was already consistent with the independent corrected values. Never overwrite accepted reference data.

**Executed**: Windows native x86-64 and WSL2 Linux x86-64, Rust 1.99.0, `cargo test --locked --offline` and warning-denied `cargo clippy --all-targets` in both feature modes, plus Python generator recomputation: 3 integration tests/configuration ×4 configurations =12 passing test executions, zero remaining failure. No current external QuantLib/date-convention equivalence assertion, physical ARM64, formal M1 or distro release claim.
