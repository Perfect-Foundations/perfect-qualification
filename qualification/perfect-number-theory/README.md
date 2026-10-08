# Independent Perfect Number Theory M1 qualification consumer

**Exact Number Theory candidate:** `41aaa4763da2d11aea9e65666f9ed7f84d43ed61`

**Dependency pins:** Perfect Arithmetic `9ec660f9abfd7fe93e441e77dbb1158c371603f5`
and Perfect Numeric `19b6747cd852a47a694a020b97ba70b6b3ef259b`.

This is a **non-production, independent public-API consumer**, not a production
dependency or a declaration that Perfect Number Theory has completed M1.

## Independent sources and executable contracts

Python's standard-library arbitrary-precision `int`, `math.gcd`,
`pow(..., -1, modulus)`, CRT brute-force enumeration, and trial division
generate and verify the checked-in reference rows. A Lucas–Lehmer calculation
independently confirms the (2^{61}-1) Mersenne prime case; known large
composites include mathematically checked explicit factors.

- `vectors/modular.tsv`: **300** signed modular add/sub/mul/pow,
  inverse or structured noninvertibility cases.
- `vectors/crt.tsv`: **189** independent two-modulus CRT solutions.
- `vectors/primality.tsv`: **4,103** cases, including every `0..=4096`,
  the verified (2^{61}-1) prime, and five large composites.
- `tests/qualification.rs`: public API-only checks for those vectors,
  Miller–Rabin probable-versus-proven distinction, valid/adversarial
  Pocklington certificates, structured resource failures, and CRT errors.
- `scripts/verify_oracle.py`: independently regenerates every expected
  vector deterministically, detecting accidental edits or source drift.

## Local qualification prerequisite evidence

On Zen Ubuntu WSL2 x86-64, **Rust 1.99.0**, exact pinned Git source checkouts
and isolated **development-only path materialization** were used. Tests:
**6/6 all features, 6/6 no default features**, and
**4,592/4,592 independent Python reference vectors PASS**; formatting, Clippy
`-D warnings` both modes, rustdoc, and Class-B `no_std + alloc` compile-only
checks on `wasm32v1-none`, `thumbv7em-none-eabihf`, and
`riscv64imac-unknown-none-elf` PASS. Retained details:
`verification/LOCAL-ZEN-M1-20261008.json`.

This is **PASS_LOCAL_ONLY**. Native Linux ARM64, Windows execution of this
independent consumer, hosted Q0–Q5, broader certificate adversarial/resource
coverage, and subsequent release conditions are **NOT-RUN/BLOCKED** as
applicable. A finite vector corpus does not prove all u64 inputs.

The external dependency graph has **not** been published to crates.io;
isolated local path materialization is not used to fake a registry package.
Perfectπ remains read-only.
