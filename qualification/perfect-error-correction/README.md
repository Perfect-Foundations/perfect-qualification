# perfect-error-correction — independent pinned Git M1 consumer

This separate Rust crate does not ship as a production dependency.
Its Cargo.toml and tracked Cargo.lock pin actual immutable Git source revisions,
never path overrides or nonexistent registry publications.

`python scripts/reference.py` and `python scripts/bw_gf7_reference.py`
recompute the applicable frozen mathematical references using the Python
standard library, without accepting Rust output as expected results. The
new GF(7) reference generator refuses overwriting a preexisting vector.
Frozen hashes and source provenance are recorded in
`verification/CURRENT-SOURCE-20261011.md` (historical) and
`verification/WAVE8-BW-20261012.md` (new FEC source revision).

Verification:
```sh
python scripts/reference.py
cargo test --locked --offline --no-default-features
cargo test --locked --offline --all-features
cargo clippy --locked --offline --all-targets --no-default-features -- -D warnings
cargo clippy --locked --offline --all-targets --all-features -- -D warnings
```

Recorded evidence covers Windows x86-64 and WSL2 Linux x86-64 with
Rust 1.99.0; it does not imply formal M1 completion or native ARM64.
