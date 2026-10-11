# Perfect Uncertainty — independent pinned M1 consumer

This is an assurance-only Rust consumer, **not** a production dependency.
It builds `perfect-uncertainty` directly from exact immutable commit
`cb5bf156705d69374aa781fd1b214a85e5d4d15d`, with an audited
`Cargo.lock`. It does not use Cargo path patches or unpublished registry
substitutes.

`scripts/reference.py` independently computes 103 exact GUM first-order
variance cases using Python `Fraction` and diagonal-plus-paired-offdiagonal
arithmetic. The Rust integration test uses only the published public API,
checks each expected variance/standard uncertainty under an explicit binary64
tolerance, tests PSD/adversarial rejection, and verifies coverage metadata
does not manufacture confidence probabilities.

```sh
python scripts/reference.py
cargo test --locked --offline --no-default-features
cargo test --locked --offline --all-features
cargo clippy --locked --offline --all-targets --no-default-features -- -D warnings
cargo clippy --locked --offline --all-targets --all-features -- -D warnings
```

Only the committed SHA, feature/configuration/host combinations, numerical
tolerance and named examples are supported by this evidence. Do not infer
native ARM64 execution, general GUM standards conformance, M1 closure,
interval enclosure, distribution shape, coverage probability or release.
