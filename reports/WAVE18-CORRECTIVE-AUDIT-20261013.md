# Wave 18 corrective audit — 2026-10-13

## Exact source revisions
- Error Correction PR #2: `3a57c79d6d34c1e0a975c70e9076e023a45eec87` — one bounded-batch semantic bug fix.
- Finance PR #2: `be084cb969b9ab7b4c4c7696422a2174d8bc85e4` — **documentation/status-only** CI-health correction; Finance production source unchanged from `82c6c71ae7b94942ceb47259dcc1f995b4147ba3`.
- Qualification PR #12 previous HEAD: `7424d4348464428d8a34edfc240993ff6f03a255`. This new exact source graph supersedes the previous live pins; frozen mathematical-reference corpora and their SHA256 fingerprints remain unchanged.
- Algebra remains `20b2f10ae3390808d412be84c7af76ffbcb2766d`, with unchanged lower-level pins.

## Reproduced bug and fix
At Error Correction `f08e8a5435d9a067558fb69009bb618bfa948fb3`, `decode_batch_into_with_workspace` retained stale surplus output slots when an admitted batch was shorter than the previous one. A regression on Zen (Rust 1.99, WSL2 Linux x86_64, locked/offline) failed: length 4 observed versus 1 required. Source `3a57c79d6d34c1e0a975c70e9076e023a45eec87` truncates outputs to the admitted word count *after* global admission; a rejected batch still preserves caller state. Re-running the previously failing production regression passed.

Independent `qualification/perfect-error-correction/tests/wave18_direct.rs` uses the existing frozen GF(17) reference codewords and checks four-/eight-word batch shrinkage to one and then zero. The newly added Qualification regression **failed** with the old pinned source (8 versus 1), then **passed** on Windows Native when repinned to `3a57c79d6d34c1e0a975c70e9076e023a45eec87`.

## Executed host verification
- Error Correction production source `3a57c79d6d34c1e0a975c70e9076e023a45eec87`: Windows Native x86_64 and Ubuntu WSL2 x86_64 Rust 1.99 locked/offline all-target tests in both no-default and all-feature modes, warning-denied Clippy, format; Windows rustdoc passed.
- Updated independent Error Correction and Finance consumer worktrees: Windows Native x86_64 Rust 1.99 locked/offline all-target tests, warning-denied Clippy and format in both feature modes **passed** after both Git-pin and lockfile updates.
- **Not yet passed on the new pins**: Ubuntu WSL2 independent Qualification consumer rerun. Its cargo fetch failed before compilation because the new private Git revisions were missing from the Linux Cargo cache and WSL2 Git had no working GitHub credential provider. That failed setup is not a Rust test failure and cannot be reported as passing. The prior `7424d4348464428d8a34edfc240993ff6f03a255` Windows/WSL2 Qualification evidence remains valid only for its older source graph.
- Hosted GitHub Actions remains **blocked**, not green: Finance source run 38346831973 failed within two seconds with no executed steps and GitHub's explicit failed-payment/spending-limit annotation. Error Correction and Finance live-status push jobs reported the same restriction. Draft PR jobs may be skipped. This is not source failure and no CI success is claimed.

## Open gates and retained boundaries
M1 and Q6 acceptance, hardware-native ARM64, hosted CI, packaging/licensing, release and protected merges are still open. QEMU-emulated AArch64 runtime results from the prior exact Wave 18 graph are historical source-specific evidence; do not automatically transfer them to the changed FEC SHA. Perfectπ was not modified. No progress percentage or readiness gate is awarded by this corrective report.
