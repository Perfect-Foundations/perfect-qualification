# Higher-dimensional conditioned covariance qualification — October 11, 2026

## Fixed Git source identities

- Production: `Perfect-Foundations/perfect-uncertainty@1450a418f5473b3e173a0cc79c65f0ce55d29427` (draft PR #2).
- This Qualification branch: `qualify-wave3-uncertainty-20261011` (draft PR #13).
- Exact Cargo.lock SHA-256: `3b5d923062f52b80fe98cea38f9e500576397dbac2a319c9ec644e744aafeba8`.
- Dependents: solely the Git-pinned production Uncertainty crate and `libm=0.2.15`. No path overrides or published-crate spoofing.

## Reproduced numerical defect and correction

In previously pinned production `66af44422c38f5c86d4e4b3c44f8ae2a37c1a59c`,
the valid four-dimensional equicorrelation matrix with
`rho=1-2^-53` and vector `[10^8,-10^8,-1,0]` produced variance
`3.2204460603525433` rather than the independently computed exact
binary64-input variance `3.220446049250313`; relative error ~3.45e-9.
The cause was collapsing the compensated matrix-vector row to a single
`f64` before outer multiplication. Source commit `1450a418...`
retains both high and low row components and uses separate FMA residual
products in the compensated outer sum. A permanent source-level regression
tests that exact case.

## Independent mathematical reference

`scripts/conditioned_reference.py` uses Python standard-library
`fractions.Fraction.from_float`, **not** the Rust implementation,
to compute the exact quadratic form for binary64 source inputs:

`u_c^2 = (1-rho) * sum(g_i^2) + rho * (sum(g_i))^2`

The tested positive-semidefinite equicorrelation matrices have
eigenvalues `1-rho` (multiplicity n-1) and
`1+(n-1)*rho`. Source rho values include perfect and near-perfect
correlation, moderate correlation, independence, and permitted negative
correlation. Tested dimensions are 4, 6 and 8, with mixed-sign and
cancelling contributions at scales from `10^8` to `10^140`.
Data are encoded as exact f64 bit patterns, not rounded text approximations.

- 900 frozen independent exact rational variance references:
  `vectors/conditioned_4_6_8_v1.tsv`.
- Retained SHA-256:
  `30541264282bfcb43ada3e8349e8ac8e4c8fe1b90eb61ffcb068f0f6e9f37cc3`.
- Public Rust consumer tests each of the 900 reference vectors against
  real production APIs, then repeats them with input order reversed
  (equicorrelation permutation invariance). Numerical relative tolerance
  is explicit `1e-10` for nonzero reference values; exact zero is
  required for the chosen algebraic zero-variance vectors.

The previous immutable reference families, their generator code and
fingerprints are unchanged: 103 original + 160 adversarial + 234
near-singular reference cases. **Total frozen independent mathematical
reference cases: 1397**, plus repeated permutation checks on the new 900.
The 900 reversals are metamorphic executions, *not* additional independent
mathematical reference rows.

## Executed verification

Rust 1.99, native Windows x86-64 and Ubuntu WSL2 Linux x86-64:

- `cargo test --locked --offline --all-features` passed.
- `cargo test --locked --offline --no-default-features` passed.
- `cargo clippy --locked --offline --all-targets --all-features -- -D warnings` passed.
- `cargo clippy --locked --offline --all-targets --no-default-features -- -D warnings` passed.
- Independent Python high-dimensional oracle regenerated all 900 cases
  and checked frozen bytes and SHA-256.
- Exact pinned Cargo.lock, Git source, and **all five** vector fingerprints
  verified on both hosts.

Nine executable Rust Qualification test functions per host/feature
configuration, **36/36 total passing test executions**, zero failed.
Windows and WSL2 share one physical x86-64 host. WSL's private Cargo
Git cache received the same exact Git object from the local verified
source clone, without dependency identity substitution.

No formal M1/Q6 acceptance, physical ARM64, hosted CI or release claim;
no Perfectπ modification and no protected merge.
