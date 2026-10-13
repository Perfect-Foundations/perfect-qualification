# Wave 19 — Final pristine Qualification replay (2026-10-13)

## Frozen inputs
- Independent Qualification source commit: `06f765d220aa51c78d3e293d6b7629243f02d4d4` (prior to this evidence-only commit).
- Production Error Correction: `0c8b40568545fee98b5ebae8edc36b23e4463ab0`.
- Production Finance: `f91e5ae9b9d297721afe2d507d74ad22a5e689d0`.
- Exact dependencies and mathematical-vector hashes: `verification-wave3-graphs.py` and committed Cargo.lock files.
- Clean detached checkout created at the Qualification commit shown above. Its tracked status was empty before verification. The older dirty Qualification worktree remained unchanged.

## Actual clean-checkout results
The following tests **executed**, each with `cargo +1.99.0 test --locked --offline --all-targets` under `--no-default-features` and `--all-features`:

| Execution host | FEC consumer | Finance consumer |
|---|---|---|
| Windows 11 Native x86_64 | 19 passed each mode | 12 passed each mode |
| Ubuntu 24.04.5 WSL2 x86_64 | 19 passed each mode | 12 passed each mode |
| AArch64 Linux user-mode QEMU on Zen x86_64 | 19 passed each mode | 12 passed each mode |

Each of the 12 complete consumer commands exited 0. Across platform/configuration repetitions this is **186 passing test executions, 0 failing and 0 ignored**; those executions are not 186 independent mathematical vectors. WSL2 and AArch64 executions share the x86_64 host, and user-mode QEMU does **not** establish native physical ARM64.

The clean checkout's `python verification-wave3-graphs.py` also exited 0: pinned Cargo.lock/Git-source identities, all frozen vector SHA256 checks, independent GF(7) BW, Hamming packed, Finance currency, Finance date-range regenerated references and prior exact dual-revision benchmark provenance passed. No mathematical reference corpus was changed.

The source implementation and targeted checks were already verified on the same pinned production revisions (see `WAVE19-FEC-FINANCE-20261013.md`). No CI surrogate, paid infrastructure, source override, or external runner was used. This report is documentary, so the additional Qualification commit does not change code, manifests, pins, test definitions, or mathematical references.

## Remaining formal gates
M1 formal review/acceptance, physical ARM64 hardware evidence, M6/Q6 acceptance, release qualification, crate packaging/licensing and publication remain open. No quantified new performance or allocation claim was accepted, and no existing progress percentage changes.
