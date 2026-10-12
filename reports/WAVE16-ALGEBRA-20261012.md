# Wave 16 — Algebra/Units source-exact qualification

Algebra pin: `20b2f10ae3390808d412be84c7af76ffbcb2766d`.
Units source unchanged.

New qualification test computes expected extension-field subtraction
independently as GF(2) polynomial XOR, covering all GF(16) pairs
and selected GF(256) operands, plus incompatible-parent refusal.
Previously frozen 369 mathematical Algebra rows remain untouched.

Windows Native Rust 1.99, both feature configurations, rustfmt,
all-target tests, warning-denied Clippy, independent reference
regeneration and locked exact source checks passed on working source.
Final detached-checkout replay is a separate required gate; production
implementation alone does not grant M1/Q6 acceptance.

No hosted CI, release, or protected branch merge.
