# Wave 7 — historical Finance/FEC source-exact checkpoint

This record preserves the verification of the **prior** source revisions.
It is not evidence for subsequent Wave 8 production changes, nor a formal
M1/Q6 acceptance decision.

| Source | Exact revision |
| --- | --- |
| Perfect Finance PR #2 | f867494370f714e9cec962dd6202d9f546e397fd |
| Perfect Error Correction PR #2 | 1316c8b0d0db3d21b67b456bf17f2da8359fc57d |
| Perfect Qualification PR #12 | c17e2284a117b6b209079ce5e882acd24342f12d |

Wave 7 local recorded outcomes (2026-10-12):

- Rust 1.99 Windows Native x86-64 production Finance and FEC: both
  feature configurations, locked/offline tests and warning-denied Clippy
  passed; formatting checks passed.
- Windows Qualification consumers of both pinned production commits:
  both feature configurations, locked/offline tests, formatting and
  warning-denied Clippy passed (QUAL_WINDOWS_MATRIX_EXIT=0).
- Pristine detached Ubuntu WSL2 x86-64 checkout of Qualification
  c17e228: both consumers, both feature configurations, tests,
  formatting, warning-denied Clippy passed (LINUX_QUAL_MATRIX_EXIT=0).
- Python independent source/math regeneration, exact dependency graphs,
  frozen reference hashes and Finance dual-revision performance manifest
  passed (ORACLES_EXIT=0). Both committed mathematical reference
  corpora and lockfiles matched their declared hashes.
- The two OS executions share the **same physical Zen computer**;
  this is independent OS compilation, not independent physical hardware.
- All three revisions remained draft PRs; no protected merge, formal M1
  acceptance, release or crates.io publication occurred.

CI exception: Finance job 114649278596 (run 38199422971) did not
start. GitHub returned this account-level annotation:
"The job was not started because recent account payments have failed or
your spending limit needs to be increased."
FEC run 38199510712 and Qualification run 38202490620 had skipped
jobs. This is **not** a green hosted CI result and does not authorize
spending, billing changes, or repeated hosted retries.

The later Berlekamp–Welch change supersedes this FEC source pin; the
Wave 7 evidence remains exclusively historical.
