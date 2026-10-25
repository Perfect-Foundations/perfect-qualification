# Published metrology numerical examples — exact source provenance (2026-10-11)

These are **published numerical values**, distinct from the project's
independent Python Fraction test corpora (3,088 mathematical references).
They do not increase the independent generated-reference count.

## JCGM 100:2008, GUM 1995 with minor corrections (corrected version 2010)

Official JCGM/BIPM publication:
https://www.bipm.org/documents/20126/2071204/JCGM_100_2008_E.pdf

* Section **5.2.2 Example**, printed page **22**, PDF page index **33**.
* Published setup: **ten** nominal `1,000 ohm` resistors, all calibrated
  using the **same** `1,000 ohm` reference resistor of standard uncertainty
  `u(R_S)=100 milliohm`. The ten resistors are connected in series,
  giving reference resistance `R_ref=10 kohm` with common-reference
  covariance equivalent to perfect correlation among the ten inputs.
* **Published numerical result**: combined standard uncertainty
  `u_c(R_ref)=10 × 100 milliohm = 1 ohm`.
* The published example explicitly contrasts that correct result with
  the incorrect **0.32 ohm** obtained by assuming the resistors are
  uncorrelated (square-root-of-ten scaling).

Executable reference `tests/published_metrology.rs` instantiates a 10x10
all-ones correlation matrix and `g_i=0.1 ohm`, verifying variance
`1 ohm^2` and combined standard uncertainty `1 ohm`. It also
constructs an identity matrix for the *incorrect* uncorrelated
comparison, obtaining `sqrt(0.1)≈0.316227766 ohm`, which the source
reports as **0.32 ohm**. This is published example verification, not an
invented NIST/JCGM source dataset or a claim of full GUM conformance.

## NIST Technical Note 1297 (1994 edition), Section 7.3

Official NIST webpage:
https://www.nist.gov/pml/nist-technical-note-1297/nist-tn-1297-7-reporting-uncertainty

* Published nominal mass estimate `m_s=100.02147 g`.
* Published combined standard uncertainty `u_c=0.35 mg`.
* Published `k=2` expanded uncertainty `U=0.00070 g`
  (=0.70 mg), and `k=2.26` `U=0.00079 g`
  (the arithmetic gives 0.791 mg before published reporting rounding).

The NIST section publishes the **combined** uncertainty, not the full
underlying measurement-input covariance decomposition. The executable
test therefore uses a mathematically equivalent **single-input** scalar
propagation to test the published combined value and expansion/reporting
arithmetic, not an invented set of original measurement components.
Unit conversions (mg/g) and expected reporting rounding are explicit.
A supplied coverage factor alone is not interpreted as evidence of a
particular probability or distribution, even though NIST separately
states assumptions for those example confidence interpretations.

## Execution and limits

Run with Rust 1.99:
`cargo test --locked --offline --test published_metrology`.

These source-exact public API numerical examples passed on Windows native
x86-64. WSL2 source-exact qualification and pristine replay are tracked
in separate final evidence. Neither example establishes universal
numerical accuracy or complete published-standard conformance.
