# perfect-algebra independent M1 consumer

This consumer is **not a production dependency**. Cargo.toml pins exact Git revisions. The tests exercise only the public API of the subject crate; expected mathematical values are generated independently in Python, retained in versioned immutable `vectors/*_v1.tsv`, and verified by `python scripts/reference.py`.

Run from this directory with Rust 1.99.0:

```shell
python scripts/reference.py
cargo test --locked --offline --no-default-features
cargo test --locked --offline --all-features
cargo clippy --locked --offline --all-targets --no-default-features -- -D warnings
cargo clippy --locked --offline --all-targets --all-features -- -D warnings
```

The exact revisions, mathematical corpus SHA256, host/feature evidence and unearned release gates are in `verification/CURRENT-SOURCE-20261011.md`. Revisions and vector corpus must not be silently regenerated to agree with implementation.
