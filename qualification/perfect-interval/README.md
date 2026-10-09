# Perfect Interval — independent mathematical qualification candidate

**Candidate source:** `Perfect-Foundations/perfect-interval@30aca00817bb79f0be3bf43e96c767db9fe45bc5` (the tested executable revision).
The later Interval PR #2 docs/status commits do not change production code or tests.
This is qualification-only; no new production dependency.

## Mathematical oracle (reproducible)

`scripts/generate_vectors.py` uses **GNU MPFR 4.2.1** via Python 3.12's
`ctypes` binding to the installed `libmpfr.so.6`, plus Python's standard
`fractions.Fraction` exact arithmetic. No computation calls Perfect Interval
or Perfect Float to generate expected results. Its fixed PRNG seed is
`0x51991b8a`; target precisions are 3, 4, 5, 8, 13, 24, and 53 bits.
Inputs are signed dyadics of exactly stated numerator and power-of-two
exponent (including exponents ±2048 and precision-boundary values).
For each of 226 finite valid addition, subtraction, multiplication, division,
reciprocal, and negation cases, exact Fraction expressions independently
select the mathematical extrema of interval endpoints; MPFR then rounds the
corresponding exact expression toward -∞ and +∞ at destination precision.
Every generated MPFR pair is checked using Fraction against the actual exact
mathematical minimum and maximum before a vector is admitted.

Checked-in corpus: `vectors/real_m1_mpfr_20261008.tsv`.
SHA-256: `604bf0ae6b6d506c4aaa93ebe0e374a23fb7b1327222b9a53aa43293d163c744`.

The Rust consumer uses only the public Perfect Interval/Float API.
It recreates oracle's dyadics *exactly* in 128-bit Perfect Float through
Rational construction (asserting that conversion was exact), and checks:
- every returned Interval lower endpoint <= independent certified MPFR lower;
- every returned Interval upper endpoint >= certified MPFR upper;
- Ball conversions preserve the original mathematical endpoints;
- Ball arithmetic converted back to Interval encloses certified results;
- unexpected Empty, non-conservative narrow results, or float errors fail tests.

An Interval result wider than MPFR is acceptable and must **not** be called
incorrect. A Ball result may conservatively become Entire; the test records
such cases separately. In the initial Windows campaign, no Ball result became
Entire. The corpus is finite; it is rigorous *for those cases*, not a proof
for every possible exponent, precision, or input set.

Additional independent verification now runs **GNU MPFI 1.5.3**, extracted
unprivileged from the official Ubuntu noble `libmpfi0` binary package.
There is no system installation and no production native dependency.
The package SHA-256 is
`212f784d45037823c57af4484176926da21687234e2ec1c111368ee4171ac1c6`.
`scripts/verify_mpfi_reference.py` uses the reviewed Ubuntu x86-64 ABI
(`MPFIValue` 64 bytes; `MPFRValue` 32 bytes) via verification-only ctypes,
evaluates all 226 input intervals at 256-bit precision and independently
checks exact Fraction minima/maxima. It also confirms every MPFI 256-bit
enclosure lies within the independently generated destination-p-bit MPFR
enclosure. Output/reference digest:
`e63166822e791dd74507bdb5c3f9662b1979a6390d9456fd643aaa6452a13728`.

MPFI uses MPFR internally; this is independent **from production**, not an
entirely independent MPFR arithmetic backend. FLINT/Arb is still NOT RUN. MPFR
is verification-only and is not linked to the Rust consumer or production.

## Identity and execution

The original consumer `Cargo.toml` pins Interval + Arithmetic + Rational
by Git SHA. Upstream pinned Interval -> Float + Arithmetic, Float ->
Numeric + Arithmetic + Rational, and remaining graph edges are checked
against the five exact reviewed revisions. `scripts/materialize_exact.py`
refuses unexpected source SHA/manifest revision and rewrites **only**
disposable cloned manifests after retaining original manifest digests.
This is not a registry crate or published lockfile.

`scripts/run-local.sh` clones the six repositories from
`PFQ_QUAL_REPO`, `PFQ_INTERVAL_REPO`, `PFQ_FLOAT_REPO`,
`PFQ_ARITHMETIC_REPO`, `PFQ_RATIONAL_REPO`, `PFQ_NUMERIC_REPO`.
It requires `PFQ_QUAL_SHA` and installed Rust 1.99.0; it retains
all command outputs under a unique `$HOME/pf-verify` scratch directory.
If libmpfr is installed, regenerate and SHA-verify the corpus before
running Rust tests. If absent, record regeneration as NOT RUN while
still exercising checked-in independent source vectors. The callable
GitHub workflow is merely configured, **not executed**.

No native Linux ARM64 acceptance or official Q0–Q5 claim follows from
Windows/Linux testing or QEMU emulation. Production Perfect Interval M1
and independent mathematical M2 remain subject to their actual gates.
Perfectπ was not changed.
