# Perfect Uncertainty — independent pinned M1 consumer

This is an assurance-only Rust consumer, **not** a production dependency.
It builds `perfect-uncertainty` directly from exact immutable commit
`66af44422c38f5c86d4e4b3c44f8ae2a37c1a59c`, with an audited
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

Near-singular binary64 input verification additionally uses
`scripts/conditioning_reference.py` and frozen
`vectors/conditioning_v1.tsv` (234 exact Fraction-derived cases).
The full four-corpus mathematical reference count is **497**.
The complete evidence and numerical limits are documented in
`verification/CURRENT-SOURCE-20261011.md`.

## Arbitrary PSD Gram qualification extension (2026-10-11)

Run `python scripts/gram_reference.py` to recompute 1,601 additional exact rational variance references from general sign-Gram PSD correlation matrices (3 to 16 dimensions), with a frozen SHA-256. The Rust consumer tests all of them in both normal and reversed coordinate order and checks 49 independently constructed invalid-source PSD rejection cases and six singular controls. Total mathematical reference count is now 2,998 independent rows, not counting the permutation repeats. Details and limitations: `verification/GRAM-ARBITRARY-20261011.md`.
