# Wave 24 — Error Correction / Finance source-exact qualification
Date: 2026-10-13. No milestone approval or release.

## Exact source pins
- Perfect Error Correction PR #2: `ca6fd8437a3bb024ebe199cf6a6f6c3d0427bde5`.
- Perfect Finance PR #2: `7d0e44ad33d33313c671348605d5562e7bbed478`.
- Perfect Algebra remains `20b2f10ae3390808d412be84c7af76ffbcb2766d`, unchanged.
- Manifests and Cargo.lock files pin the exact production commits; the source graph verifier validates every transitive Git identity and the original frozen mathematical vector checksums.
- `python verification-wave3-graphs.py`: PASS, including previously frozen BW GF7, Hamming, and Finance oracle regenerators.
- `verification-wave21-daycount.py` remains preserved; Wave 24 does not rebrand earlier 96 day-count reference cases.

## New bounded mathematical/state assurance
- Error Correction: independent primitive-integer evaluation oracle for GF(5) RS(4,2) and GF(7) RS(5,2). Exhaustive guaranteed-radius cases: 2,800 GF5 and 5,880 GF7, including mixed errors/erasures and atomic invalid-input refusal. 8,680 is the number of tested symbol-damage cases, not unique theorems.
- Finance: three distinct deterministic seed families, 600 owned-ledger operations each. Independent reference rows, stable IDs, transactional append/replace/remove, duplicate and retired ID refusal, Decimal admission, bounded repeated indexed date-currency and currency queries, and two-range output preservation. 1,800 are state steps, not mathematical vector counts.
- These behaviors were added as permanent tests. No production algorithm or public API was changed.
- Bounded mutation evidence: isolated Finance `OwnedCashFlowLedger::reset` two-mutant campaign: one caught, one unviable; isolated FEC session decoding two-mutant campaign: one caught, one unviable. Treat unviable mutants separately from meaningful kills.

## Independent consumer matrix
Rust 1.99; `--locked --offline --all-targets`.
| Platform | FEC no-default/all | Finance no-default/all |
|---|---|---|
| Windows Native x86-64 | PASS/PASS | PASS/PASS |
| Ubuntu WSL2 x86-64 | PASS/PASS | PASS/PASS |
| Local AArch64/QEMU Linux | PASS/PASS | PASS/PASS |

The two QEMU FEC consumer runs exceeded short-command transport timeouts, then were completed using persistent sessions; both final process exit codes were 0. No physical ARM64 evidence is claimed.
Windows warning-denied all-target Clippy passed for both crates in both feature modes. Rustfmt --check passed.

## New canonical numerical transcripts
Canonical source: `qualification/perfect-error-correction/tests/wave24_canonical.rs`, `qualification/perfect-finance/tests/wave24_canonical.rs`, and the Polynomial Wave 24 consumer in Qualification PR #10. Each emits 128 ASCII `W24|` records (384 independent ordered records, 1152 platform executions).
- Polynomial SHA256: `7120fd33313cee79447d11fbac5c49e84c06cb6f921c78670308adcf1c5a9ff2`
- RS SHA256: `33a256ce217e95ae00cbd1db245f03e193cdb88372d6fd98a1aaf0e9dae6b98a`
- Finance SHA256: `be0e7b9060345cb7c7bb5cabe9fad38f0fa63263fd28acb014f6eb671440183b`
- Combined SHA256: `d20d1e6e8e778092561e40d780523dc9c144fb0802280844f6aa51c41d6767c2`
- Identical across Windows Native, WSL2 x86-64 and local AArch64/QEMU; no divergence.
- `verification-wave24-canonical.py --log-dir C:\\Scratch --output reports/WAVE24-CANONICAL-20261013.json` reproduces these checks. Raw execution logs are retained at `C:\\Scratch\wave24-canonical-{POLY,FEC,FIN}-{win,wsl,qemu}-20261013.txt` on Zen; they must not be called source-controlled evidence until actually committed.

## Remaining verification gates
Formal M1/M2/Q6 acceptance, untested physical ARM64 hardware, holistic fuzz/mutation, long-running resource soak, independent performance regression metrics, formal license/security review and publication remain OPEN. No claimed speedup. No hosting, CI, billing, or paid services.

## Final Polynomial source follow-up
Polynomial PR #18 advanced to `ae45f4d386dff6d525424b70e3115074dc7f8f51` solely through new mutation-derived tests. The prior 384-record corpus at `7774e361c8a86c1ad3f5358d8df7a353371713d9` remains frozen and is not rewritten. The expanded canonical verifier now accepts an explicit Polynomial source and immutable log tag; the new final-source corpus is retained separately, with exact new hashes in `WAVE24-CANONICAL-FINAL-20261013.json`. Error Correction and Finance source pins are unchanged.
