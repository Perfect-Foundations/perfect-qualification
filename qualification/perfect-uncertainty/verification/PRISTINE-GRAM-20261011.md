# Pristine arbitrary Gram-correlation Qualification replay — 2026-10-11

## Exact artifact checkpoint

- Tested Git source candidate: `Perfect-Foundations/perfect-uncertainty@1450a418f5473b3e173a0cc79c65f0ce55d29427` (PR #2, draft).
- Tested Qualification commit: `Perfect-Foundations/perfect-qualification@a9bd8f6b0172c1cebf713900f19c5574a73ff13e` (PR #13, draft).
- A detached Git worktree was recreated from **exactly** `a9bd8f6b0172c1cebf713900f19c5574a73ff13e`. Git status was clean. No source-file copies, uncommitted tests or local Cargo path substitutions were used.
- Root source-identity checker verified the single exact Git dependency, `libm=0.2.15`, immutable lockfile and six normalized-LF corpus SHA-256 identities.
- Checked-in Cargo.lock SHA-256: `60004d8fa0004c180dfb1a667a6628c170191941c2bce8614234344b7db85525`.

## Mathematical reference and tests

- Original GUM reference: 63 two-input + 40 three-input = 103.
- Previously independently generated adversarial: 160.
- Previously independently generated binary64 near-singular: 234.
- Previously independently generated conditioned equicorrelation 4D/6D/8D: 900.
- **New arbitrary PSD sign-Gram and perturbed Gram corpus:** 1,601 in dimensions 3, 4, 6, 8, 12, 16; SHA-256 `d10b42da25b5746390c1b1cd47677a6b5704d7e9f03eeaab483898b933560aaf`.
- **Grand total = 2,998 independently generated exact mathematical references.** Input and expected values are frozen binary64 bit patterns. Each new Gram vector is tested in normal and reversed coordinate order (3,202 propagation computations) with explicit `1e-10` relative variance tolerance on finite positive expectations.
- **Invalid PSD refusal:** 49 cyclic placements of a 3x3 indefinite source block with exact witness variance `-2^-31`, caller PSD tolerance `1e-12`; six additional exact rank-one singular PSD acceptance controls. These are structurally reasoned controls, not additional reference rows.

## Actually executed, exact PR revision

- Windows native x86-64, Rust 1.99: `cargo test --locked --offline` and `cargo clippy --locked --offline --all-targets -- -D warnings` passed for both `--no-default-features` and `--all-features`.
- WSL2 Ubuntu Linux x86-64, Rust 1.99: same complete matrix passed.
- Each of those four host/feature configurations executed 12 Rust integration test functions: **48/48 passing test executions**.
- All five independent Python reference generators recomputed and byte-verified all six corpora from pristine source on **both hosts**.
- The source Git SHA, Cargo.lock contents, and six mathematical corpus SHA-256 values passed the root verifier independently on **both hosts**.
- Hosts share one physical x86-64 machine. The signed/dyadic Gram construction guarantees PSD *for the exact input coefficient values*; this test corpus does **not** prove that every arbitrary PSD binary64 input is accepted or has an accuracy guarantee.
- Neither source PR nor Qualification PR is formally accepted/released. No hardware ARM64 run, hosted CI, official GUM conformance or new performance benchmark is claimed. Perfectπ remains read-only.
