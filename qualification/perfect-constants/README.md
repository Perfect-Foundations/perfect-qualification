# Perfect Constants M1 — independent public API qualification candidate

This stand-alone, non-production consumer depends on exact Git source
`d88d8055f45fb075c905f1fe9e82a623029faf41`.
Its committed Cargo.toml retains the authentic pinned Git declaration.
Offline local verification clones exact source and qualification revisions,
validates their identity, saves the original manifest, and rewrites only
the disposable consumer manifest to an exact local path. This is **not**
a published crate or registry lock.

The qualification-owned `vectors/native.tsv` contains seven IDs and
14 independent IEEE binary32/binary64 bits. `scripts/verify_oracle.py`
computes its own standard-library Decimal/Fraction mathematical intervals
and exact nearest-even rounding to validate all 14. A separate mpmath 1.3.0
180-digit reference is checked when installed; absent mpmath is reported
rather than falsely called executed. No production Rust is used to generate
reference expectations.

Canonical semantic fingerprint **Q-PC-MATH-V1** serializes the ASCII domain
`PF-CONSTANTS-MATH-V1` followed by NUL, then in fixed public ID order
one byte of UTF-8 name length, the UTF-8 name, little-endian binary32
`to_bits` and little-endian binary64 `to_bits`. FNV-1a 64:
`35f7e276b5ecd6dc`. SHA-256 of the independently serialized reference
records: `481cb9f138bae72bf8d6b204abdea518b5adcd8a90c0f205c23a083fe27b11ca`.

The public Rust consumer tests completeness, variants, stable names,
all 14 encodings, named-constant aliases, finite/range checks,
const-evaluation, and the independent fingerprint. The standalone
`alloc_probe` example instruments the host allocation API around 7,000
lookups (non-production unsafe allocator wrapper, with exact delegation).
It proves zero observed allocations for that bounded probe only, not every
context. No production dependency goes back to Qualification. No π or
CODATA constant is locally owned; absence is assessed separately by
API/source inspection, not by an impossible runtime negative test.

To execute: set `PF_CONSTANTS_REPO` to a local git clone containing the
pinned source and `PF_QUALIFICATION_REPO` to a clean checkout of the exact
qualification branch. Run
`bash qualification/perfect-constants/scripts/run-local.sh`
on Linux with installed Rust 1.99.0 and three supported Class-B targets.
Logs, local dev Cargo.lock hash, target/environment evidence, and summary
are preserved in a new `$HOME/pf-verify/constants-qualification.*` scratch tree.

The callable GitHub matrix is configured separately; **no configured job
is counted as executed**. Required native Linux AArch64 Class-A remains
BLOCKED/NOT-RUN until actual native execution is available. QEMU is a
different evidence category. This candidate does not award M1/M2/M6/Q6,
package publication, or registry readiness.
