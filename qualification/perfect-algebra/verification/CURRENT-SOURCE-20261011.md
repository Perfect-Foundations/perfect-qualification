# Independent Perfect Algebra M1 consumer — October 11, 2026

## Exact source graph

- `perfect-algebra@5e24b4f4c685ef5dd26e1976cda5518b06795fe3`
- `perfect-number-theory@2bdd2eee53615acb5beac8720c1d2be62120c080`
- `perfect-arithmetic@9ec660f9abfd7fe93e441e77dbb1158c371603f5`
- `perfect-numeric@19b6747cd852a47a694a020b97ba70b6b3ef259b`

Every production/qualification dependency is a real exact pinned Git revision. Nothing impersonates a published registry crate. Qualification is never a production dependency.

## Independent mathematical reference

`scripts/reference.py` independently uses Python integers and polynomial-reduction arithmetic to produce and subsequently verify the immutable checked-in mathematical vectors, with no production output as an oracle.

- `vectors/prime_v1.tsv`: 208 prime-field add/subtract/multiply vectors, SHA-256 `9f4eef8bb5ec280787f6b5857a0584c88edce3feb89d5120ad97e7ebca77e7b8`.
- `vectors/extension_v1.tsv`: 161 polynomial-basis extension vectors, SHA-256 `f89308e9aaf99bf5cb70e7129a3025a6cf9854bcb40ee3072343f352b961f102`.
- Public API consumer: four tests including exact field tables, independent construction and proof-class separation, structurally equal parents, distinct-field incompatibility, degree-4 reducibility, degree/coeff input guards, and zero inverse refusal.

## Executed local verification

Rust 1.99.0, native Windows x86-64 and WSL2 Linux x86-64, both `--no-default-features` and `--all-features`: `cargo test --locked --offline` and `cargo clippy --locked --offline --all-targets -- -D warnings` returned 0 (four tests per host/feature; 16/16 independent test executions). Python reference recomputation and SHA checks passed both hosts. Windows and WSL private Git caches resolved **exact identical Git commits**; no path overrides or modified dependency manifests. No expected-result regeneration after production execution.

M1 gate remains unearned. Native physical ARM64, independent security review, all formal Qualification gates, distro-release, and hosted CI are **not claimed**. Historical Qualification data in other branches was not altered.
