# Perfect Number Theory — Independent M1 Consumer

This qualification harness is **not** a Number Theory production dependency.
It tests only documented public APIs and builds its expected vectors using
Python's standard-library integers and arithmetic, independent of Number Theory.

Pinned source graph:

- Number Theory: `41aaa4763da2d11aea9e65666f9ed7f84d43ed61`
- Arithmetic: `9ec660f9abfd7fe93e441e77dbb1158c371603f5`
- Numeric: `19b6747cd852a47a694a020b97ba70b6b3ef259b`
- MSRV: Rust 1.99.0

## Scope

- 300 independently generated modular add/subtract/multiply/power and
  inverse/GCD vectors, including signed operands and zero.
- 189 independently enumerated CRT congruence cases.
- 4,103 deterministic u64 prime/composite reference cases, including
  complete bounded trial division, Lucas–Lehmer verified Mersenne prime,
  and composites with independent explicit factors.
- An independent public API Bézout test: 4,225 bounded pairs, plus
  high-bit and zero-input identities.
- Explicit Miller–Rabin probability-vs-proof separation, Pocklington
  acceptance and invalid-certificate/resource-limit rejection.
- Canonical output fingerprint `0c8f081bb3fb5121`, FNV-1a/64 of 4,592
  newline-delimited **observed** public API vector responses. The Python oracle
  independently regenerates this reference; do not silently regenerate it
  to match a failing implementation.

## Run at no additional infrastructure cost

On an authorized Linux x86-64 machine with local clones containing the three
exact revisions and Rust 1.99.0 (with the declared Class-B targets installed):

```bash
PF_NUMBER_THEORY_REPO=/path/to/perfect-number-theory \
PF_ARITHMETIC_REPO=/path/to/perfect-arithmetic \
PF_NUMERIC_REPO=/path/to/perfect-numeric \
PF_EVIDENCE_ROOT="$HOME/pf-verify" \
bash qualification/perfect-number-theory/scripts/run-local.sh
```

The script makes **separate new clones**, checks each exact commit SHA, clones
the qualification harness, and changes only disposable manifests to point at
verified local sources. It checks the independently generated vectors before
Cargo work, executes Rust 1.99.0 feature-mode tests, denied-warning Clippy,
rustdoc, formatting, and three Class-B `no_std` compilation targets offline.
Logs and an unchanged development-lock checksum remain in a new directory.

A development lock created from genuine exact local Git snapshots is **not** a
published-registry package or release lock; no registry identity is impersonated.

## Gate boundaries

Local passing results are not hosted Q0–Q5 qualification, Q6 release readiness,
or published-registry dependency verification. A Class-B cross-compile does not
prove target runtime behavior. A QEMU run is emulated, not native ARM64. The
full native Ubuntu x86-64/ARM64/Windows independent matrix remains subject to
available authorized runners and exact-source evidence.

See `verification/LOCAL-ZEN-NT-M1-20261008.json` for the tested configuration.
