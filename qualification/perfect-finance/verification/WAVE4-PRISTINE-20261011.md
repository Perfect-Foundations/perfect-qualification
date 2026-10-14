# Exact-head Wave 4 downstream Finance + Error Correction Qualification — 2026-10-11

**Qualification tested HEAD:** `Perfect-Foundations/perfect-qualification@6614a3add6964b38b7096d84aee5d6c745a626f8` (draft PR #12). A detached worktree was recreated at this exact commit with a clean Git status. Windows native x86-64 and WSL2 Linux x86-64 tests, lints, oracles, and source/lock/hash verifier all executed from the clean worktree without local path overrides.

**Production Git revisions under test:**
- Finance `Perfect-Foundations/perfect-finance@90be47248298b49089d49f954866ed916bc57ed5` (draft PR #2; independent cancellation-aware cashflow fix).
- Error Correction `Perfect-Foundations/perfect-error-correction@02594e730c156ed0e5f585ba0e5d348d6240e6fd` (draft PR #2; unchanged decoder; expanded verified boundaries).
- Exact preexisting decimal, rational, algebra, number theory, arithmetic and numeric dependency identities retained in Cargo.lock. Verified Finance lock SHA256 `f627bc61e260e76e1ae3e3d4ff3f058db305366a7c6d23e964c411d3c81a35f4`, Error Correction lock SHA256 `c23e6916ff6329476cbf00c57e70e2d4be45d01c4244e22f62f69b5c18fc3cd0`.

**Frozen independent mathematical references:** Finance 100 rows (36 day-count, 48 quantization and 16 newly independent exact cashflow Fraction/Decimal sums). Error Correction 45 unchanged rows (16 Hamming, 25 GF(5), four GF(16) polynomial evaluations). The Finance new SHA256 is `d97b4baf8f271d763cc788e62452fb271fd8254319738fc4260c088ed8a35ece`. Every corpus was reproduced and SHA-verified on both operating systems. Cyclic/reversed Finance permutations and **2,250 independent GF(5) Reed-Solomon decoding error/erasure injection patterns** are additional runtime checks, *not new frozen mathematical-reference rows*. Error Correction source was not changed.

**Exact pristine test matrix:** Rust 1.99 `cargo test --locked --offline` and all-target warning-denied Clippy passed for both `--no-default-features` and `--all-features` on native Windows and WSL2. Four Finance test functions per host-feature configuration = **16/16**, five Error Correction test functions per host-feature configuration = **20/20**, aggregate **36/36 independently executed tests**. Both source/lock/hash validators and the independent Python references passed. Windows and WSL2 run on the same physical x86-64 CPU.

Previous source-local checks separately confirmed both changed production crates compile for wasm32v1-none, Thumb and RISC-V no_std targets (compile-only). No native ARM64 or hosted CI run, approved M1/Q6 gate, protected merge or release. Perfectπ was not modified.
