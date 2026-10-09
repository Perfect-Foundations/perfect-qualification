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

## Adversarial FLINT/Arb reference phase (2026-10-09)

The **new and separate** `PF-INTERVAL-ARB-ADVERSARIAL-V1` corpus is in
`vectors/arb_adversarial_v1.tsv` (330 valid finite cases; seed
`0xA8B20261009`; SHA-256
`8bb8011b5bf4e3292ea631df2d8e5f13bbdbb6e08f1df2eea8402b39fd9f9e57`).
Exact dyadic inputs span signed binary exponents -4096 to +4096 with
significands up to 51 bits, source endpoint precisions 64 and 128 bits,
and destination precisions 2–53 bits. Operations are add, sub, mul,
division, reciprocal and negation. The independent `Fraction` generator
identifies exact extrema, then installed GNU MPFR 4.2.1 provides directed
reference endpoints. This corpus does **not** replace the 226-case baseline.

A verification-only C executable, `scripts/flint_arb_oracle.c`, is built
with `-Wall -Wextra -Werror` against the official Ubuntu noble free
FLINT/Arb **3.0.1** headers/runtime, extracted without root/system changes.
The C implementation constructs exact `arf` dyadic input endpoints,
uses `arb_set_interval_arf`, and extracts certified dyadic enclosure
endpoints with `arb_get_interval_fmpz_2exp`. The independent Python
`verify_arb_output.py` recomputes Fraction extrema and tests enclosure
membership; it does not treat Arb as automatically correct.

Of 330 inputs, **292** yield finite Arb enclosures that contain the exact
mathematical extrema. **38** (23 divisions, 15 reciprocals) yield nonfinite
Arb intervals after Arb's radius representation widens very asymmetric
inputs; these are explicitly **UNSUPPORTED, NOT PASS** for Arb. All 330
cases retain rigorous independently checked Fraction/MPFR p-bit reference
bounds and remain in the Rust consumer.

The pinned locally obtained FLINT packages and SHA-256s are in
`verification/M2-ARB-ADVERSARIAL-20261009.json`. Arb reference output is
retained in `vectors/arb_reference_256_v1.tsv` (SHA-256
`2961c84f449a49c98d4ed5bfaed5e4d8e501e4eb423f3eedb98bfa57df2d4ee4`),
validated with the exact-rational oracle even when FLINT is not installed.
The callable CI validates the committed corpus and reference but does **not**
claim live Arb regeneration when the library is absent.

New Rust public-API tests (`tests/arb_adversarial.rs`) test Interval and
Ball enclosures, exact mixed source precisions, and three repeated Ball
round trips with width monotonicity. The separate `domain_semantics.rs`
tests canonical signed zero, Empty/Entire, unbounded endpoints, singular
reciprocal/division, and typed errors.

No universal inclusion proof, optimally narrow results, all extended-real
semantic branches, native ARM64 Class-A coverage, hosted Q0–Q5 approval,
or completed M1/M2 acceptance follows from this finite test set. Arb/FLINT
is verification-only and is not a production dependency.

### Execution records and exact candidate

The new candidate was executed from exact **qualification source**
`215ec5f3f5f72ad0e200a0dee3034f97fa5b68ea` against unchanged
Perfect Interval `30aca00817bb79f0be3bf43e96c767db9fe45bc5`:

- Windows x86-64 native, Rust 1.99.0: 2 new adversarial test functions
  (330 Interval + 330 Ball cases) and 4 semantic tests PASS in each
  feature configuration; warning-denied Clippy and rustfmt PASS.
- Ubuntu WSL2 native x86-64: same 6 test functions and feature modes PASS,
  plus exact-source rerun of FLINT 3.0.1, separately regenerated MPFR
  corpus, exact-rational validation, and warning-denied Clippy; logs at
  `/home/ubuntu/pf-verify/interval-arb-m2-20261009.Usnakk36/`.
- QEMU 8.2.2 user-mode AArch64 target on x86-64 Linux: 6 new test
  functions PASS per feature mode, network disabled, non-root,
  caps dropped, read-only source, bounded resources. **Not native ARM64.**
  Local log at
  `C:\Dev\PerfectFoundations\_qualification_interval_m1_logs_20261008\flint-arb-adversarial-qemu-aarch64-20261009.log`.
- The 330-case Ball operation checks conservatively widened to Entire in
  41 cases. Three chained conversion round trips for each input saw
  42 width-increase steps; no source endpoints were excluded.
- Two retained Zen-specific exact-reference replay entrypoints:
  `scripts/run-zen-linux-adversarial-20261009.sh` and
  `scripts/run-zen-qemu-adversarial-20261009.sh`. No source test
  results were promoted to acceptance by these checks.

All new source changes are confined to Perfect Qualification PR #9.
No production source change, native ARM64 waiver, or M1/M2 release gate.

### Independent Arb endpoint-corner remedy for the 38 representation-limited inputs

A *separate* verification-only FLINT 3.0.1 C implementation,
`scripts/flint_arb_corners.c`, constructs each signed dyadic input endpoint
as an exact Arb **point** and evaluates binary operations on all four
endpoint pairs (or two unary endpoints) at 256-bit precision. For supported
finite rectangular arithmetic with no zero denominator, the mathematical
minimum/maximum occurs at one of these corners. Python
`scripts/verify_arb_corners.py` independently checks each Arb output
contains that corner's exact Fraction result and the aggregate corner
enclosures contain the Fraction-derived global extrema.

**1,140/1,140** point-corner Arb results PASS independent Fraction checks,
covering extrema for **330/330** corpus intervals. This includes all 38
cases whose *direct interval-ball Arb evaluation* remains
`UNSUPPORTED_NONFINITE`. The two methods retain distinct classifications;
the latter 38 were not relabeled as native interval-Arb passes.

The independently certified point-corner output is
`vectors/arb_corners_256_v1.tsv`, SHA-256
`fb294e2de3f59c702924a0ae41be014f8583080f06a5e582a0e5496a6f87ba1d`.
Its full verification log:
`/home/ubuntu/pf-verify/interval-flint-arb-20261009/logs/arb-point-corner-certified.log`.
Point-corner confirmation remains a finite-domain mathematical reference
and cannot establish universal numerical correctness or M2 acceptance.

## Bounded M2 property and semantic fingerprint phase (2026-10-09)

The separate `PF-INTERVAL-PROP-V2` independent `fractions.Fraction` and
GNU MPFR 4.2.1 two-precision corpus has 576 deterministic finite dyadic
cases, seed `0xF09120261009`, SHA-256
`286537c0f309c87a9377207f6a0e615b96cb3e674c91adcdb3611ffec3e65de7`.
These are new cases, not a rerun of the historical 226/330 campaigns.
`scripts/generate_properties_v2.py` recreates every input and independent
destination and higher-precision directed endpoint; both references are
checked with exact Fraction extrema before emission. Initial p-only bounds
must **not** be compared to the tighter higher-precision result; a previous
test-harness false failure was corrected before source commit.

`tests/property_v2.rs` tests soundness at both precisions and checks
mathematically appropriate hull, intersection, double negation, widening
and 12 chained Interval→Ball→Interval conversions on 144 selected cases.
The extended conversion checks enforce input containment and nondecreasing
width, recording widening separately from exclusion.

`tests/semantic_fingerprint_v1.rs` encodes 64 designated independent
property cases, including both Interval and Ball outputs, requested
precision, bounded/Empty/Entire result category, endpoint and midpoint/
radius values, signed-zero signs and stored result precision.
The encoder reconstructs normalized exact signed binary significands
through **public Float numerical comparisons**; it does not inspect
private representations, use Debug, depend on locale, or hash host-native
floating bytes. Its header is `PF-SEMANTIC-FINGERPRINT-V1`; exact ASCII
rows terminate in LF. It calculates FNV-1a 64-bit (offset
`cbf29ce484222325`, prime `100000001b3`) in Rust.
`scripts/verify_semantic_fingerprint.py` independently validates the
transcript FNV-1a and computes SHA-256 of the exact bytes.
The mathematical soundness of each fingerprinted case is separately
checked against the independently generated certified extrema.
Fingerprints identify **representation-level outcomes**; mathematically
distinct but enclosing results are not automatically numerical failures.
Comparison across platforms must rely on actual executed outputs.

Windows native x86-64 and WSL2 Linux x86-64 produced matching canonical
SHA-256 `b73867901ba3a261a9ef06cafd6bbdaa8e1e10fbd4d66dd262200b83594a7f4b`
and FNV-1a-64 `d02931bfb27fe00c` in both feature modes; QEMU
AArch64 user-mode emulation also reproduced both digests. This is
observed 64-case representation identity, not proof of universal
cross-platform determinism. Rust 1.99.0; fixed exact source.

### Executed exact-source replay and outstanding official CI blocker

Executable qualification commit `9fe77817542fdc73fa9f968e0ce06a6e80efa18f`
was tested against production Interval
`30aca00817bb79f0be3bf43e96c767db9fe45bc5`,
Float `bbd8b9e4`, Arithmetic `1a54d3c`, Rational `35a8e9c`
and Numeric `19b6747`, all exact pinned commits.

Reproducibility entrypoints for Zen:
`scripts/run-zen-property-v2-linux.sh` (disposable six-repo
SHA-checked source graph), and `scripts/run-zen-property-v2-qemu.sh`
(read-only offline isolated AArch64 QEMU Rust execution). Linux logs:
`/home/ubuntu/pf-verify/interval-property-v2.fQjXPYon/`.
QEMU log:
`C:\Dev\PerfectFoundations\_qualification_interval_m1_logs_20261008\semantic-property-qemu-aarch64-20261009.log`.

The callable CI workflow additionally verifies the retained corpus digest,
independent Arb certificates, and the golden semantic SHA-256 in both
feature modes when actually invoked. This workflow has **not been run**:
the repository reported zero registered self-hosted runners; organization
runner listing returned 403 for insufficient runner visibility. No
zero-cost hosted-runner allocation was verified. This remains a
qualification blocker, not a passing or failed CI run.
