# Source-exact AArch64 Linux QEMU execution — 2026-10-10

This is executed **emulated AArch64 Linux user-mode** evidence for the existing independent Polynomial Qualification consumer, **not** a physical ARM64 qualification or CI acceptance.

## Frozen source

- Qualification branch parent: `87a25a24c51b8945ddfd0e9c46d268e80822ad36` (PR #10, draft).
- `qualification/fused-git-graph-20261010/Cargo.toml`, `Cargo.lock`, `source-exact.json` unchanged.
- Polynomial: `59fa967bdf2c20dbde7a93acb22979c8625441af`.
- Arithmetic: `d99aef2a9ef230eb725adaef664b0cc0214dc4bd`.
- Rational: `b45de7e7ad63b0850e1d3bbc67e38163b3c8fa7c`.
- Numeric: `19b6747cd852a47a694a020b97ba70b6b3ef259b`.

## Execution environment

- Zen Windows x86-64 host; Ubuntu WSL2 Linux x86-64 guest.
- Rust 1.99.0 and installed Rust target `aarch64-unknown-linux-gnu`.
- Ubuntu QEMU user emulator `qemu-aarch64 8.2.2` and GNU AArch64 cross compiler `aarch64-linux-gnu-gcc`; sysroot `/usr/aarch64-linux-gnu`.
- Installed Ubuntu cross compiler, cross libc development files, and QEMU using `apt-get install --no-install-recommends`; package transaction reported 23 new packages, 54.8 MB downloads and approximately 233 MB installed footprint.
- Used the **existing** Qualification worktree at `C:\Dev\PerfectFoundations\_qual_polynomial_m1_20261009\qualification\fused-git-graph-20261010` read through WSL, without a new clone or reset. That checkout already had unrelated **untracked** files; they were preserved.
- All cross-build output confined to `/home/ubuntu/pf-arm64-qualification-target`, reusing one target directory for both feature modes, with two Cargo jobs, incremental disabled and debug info disabled.

## Commands and observed outcomes

Both invocations use these environment assignments:

```sh
CARGO_TARGET_DIR=/home/ubuntu/pf-arm64-qualification-target
CARGO_NET_OFFLINE=true
CARGO_BUILD_JOBS=2
CARGO_PROFILE_DEV_DEBUG=0
CARGO_PROFILE_TEST_DEBUG=0
CARGO_INCREMENTAL=0
CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc
CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_RUNNER=qemu-aarch64
QEMU_LD_PREFIX=/usr/aarch64-linux-gnu
```

Under `env`, the exact command for each feature mode was:

```sh
cargo test --manifest-path /mnt/c/Dev/PerfectFoundations/_qual_polynomial_m1_20261009/qualification/fused-git-graph-20261010/Cargo.toml --locked --offline --target aarch64-unknown-linux-gnu --no-default-features
cargo test --manifest-path /mnt/c/Dev/PerfectFoundations/_qual_polynomial_m1_20261009/qualification/fused-git-graph-20261010/Cargo.toml --locked --offline --target aarch64-unknown-linux-gnu --all-features
```

- **No-default features**: QEMU executed the target-specific Rust test binaries; 24 Qualification-owned tests passed, 0 failed; doc tests 0; command **exit 0**.
- **All features**: QEMU executed the target-specific Rust test binaries; 24 Qualification-owned tests passed, 0 failed; doc tests 0; command **exit 0**.
- The output explicitly identified the compiled Polynomial Git dependency as `59fa967b`, with all other pinned Git revisions matching the source graph.
- Rust compilation and runtime test output were observed via the connected Zen terminal; **no new committed raw terminal logs** are claimed by this note.

## Assurance boundary

This demonstrates that 24 independent cross-crate exact mathematical tests run successfully in each of two feature configurations on AArch64 Linux binaries through QEMU user-mode emulation. It does **not** validate real ARM64 hardware timing, resource behavior, hardware-specific effects, physical-machine determinism, standalone source-crate test suites, a hosted CI job, independent human review or final milestone acceptance. Existing Windows and WSL2 x86-64 test and byte-determinism records are separate and unchanged. Project status and formal milestones remain unchanged; no Perfectπ writes occurred.

## Other source/API audit

Read the exact pinned public Arithmetic `Integer::sub_mul_assign`, `scale_sub_mul_assign`, `unsigned_bit_length`, and `div_rem_trunc` implementation at `d99aef2a`. Arithmetic contracts explicitly preserve exact signed multiply-subtract and unsigned magnitude width; the Polynomial callsites use these operations consistent with those contracts. This is a bounded source inspection, **not** a comprehensive independent security/API approval.

## Follow-up gates

Run deterministic V1/V2 byte streams on QEMU if broader emulated determinism is desired; separately pursue physical ARM64 execution, hosted CI, formal independent API/security review, owner packaging/license decisions and acceptance.
