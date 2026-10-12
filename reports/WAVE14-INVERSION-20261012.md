# Wave 14 — Independent Extension Inversion Qualification

Source revision pinned: `perfect-algebra`
`6a2d4924466ebeab79b849f56e6d9a8b9acfdf24`.
Existing Units dependency is unchanged.

The production change fuses `t_prev - quotient * t_current` in
extended Euclid into a single exact-polynomial result buffer. It
does not alter public identity/representations, error precedence,
canonical exactness, irreducibility, or resource-limit semantics.

New independent binary-field inverse oracle:
for GF(16) modulus 0x13 and GF(256) modulus 0x11b, brute-force
`a*b=1` using independent raw-bit carryless polynomial multiplication
and modular reduction, then compare the exact expected element to
`ExtensionFieldElement::inverse()` for *every* nonzero element.
Both zero inversions refuse. Existing 369 frozen Algebra references
remain bit-identical; these algorithmic checks are additional tests,
not new frozen independent reference rows.

In source tests, all nonzero GF(4), GF(8), GF(9), GF(16),
GF(25), GF(27), and all GF(256) elements are checked for exact
inverse consistency. The implementation uses no unsafe code,
new dependencies or global mutable state.

Local release-mode Windows x86-64 Rust 1.99, 15 test-process
samples per exact revision, 16 inversions per sample:
GF(16) old 208 allocations/71552 requested bytes/81300 ns median,
candidate 160 allocations/53248 requested bytes/63100 ns median.
GF(256) old 224 allocations/121472 bytes/135200 ns median,
candidate 176 allocations/94848 bytes/99500 ns median.
Physical peak RSS and stack are not measured; timing runs were
sequential and not interleaved.

Source-exact final Windows/WSL2 Qualification replay and accepted
release gates must be recorded separately. No M1/Q6 promotion.
