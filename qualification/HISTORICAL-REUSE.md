# Historical Proven-Reuse Qualification

Perfect Qualification enforces the Perfect Foundations [Historical Proven-Reuse Gate](https://github.com/Perfect-Foundations/perfect-family/blob/main/docs/HISTORICAL-REUSE-GATE.md).

This control prevents defects already exposed in LMES and Perfectπ from being forgotten when a Perfect crate advances.

## Qualification rule

A required historical-reuse review has five possible qualification outcomes:

- **PASS** — required dispositions exist, exact source revisions are retained, adapted work has destination-specific evidence, and no unresolved release-blocking deferral remains.
- **FAIL** — evidence proves the destination violates an applicable reuse/assurance requirement.
- **BLOCKED** — the review, source revision, required evidence, or prerequisite is missing/unavailable.
- **INCONCLUSIVE** — evidence ran but cannot establish the required property.
- **NOT_APPLICABLE** — applicability was explicitly evaluated and justified.

Missing evidence is **BLOCKED**, never NOT_APPLICABLE and never PASS.

## Gate integration

### Q0 — repository integrity

Check:

- the crate has evaluated its row in the family Proven Reuse Applicability Matrix;
- source project/revision identifiers are exact;
- dispositions use the family vocabulary;
- any adapted source/code/data has provenance/licensing records;
- deferred items identify impact and recheck gate.

### Q2 — semantic integration

Where predecessor semantics were adapted, verify the destination preserves the intended ownership boundary.

Examples:

- Perfectπ remains the π specialist rather than an unrelated utility dependency;
- LMES medical semantics remain outside generic Perfect crates;
- provenance/integrity is not promoted to factual truth;
- uncertainty is not silently converted into confidence/certainty;
- exact/rounded/lossy numerical states remain explicit.

### Q4 — robustness

Applicable historical defect classes must be exercised by destination-specific evidence.

Examples include:

- deep-path fuzz reachability rather than nominal fuzz execution;
- precision/resource boundary regressions;
- parser/routing false-PASS conditions where structured policy text is processed;
- negative/error/unavailable states;
- suppression/fixture-discovery/test-isolation controls where relevant.

### Q5 — external reference

Adapted code or algorithms MUST NOT serve as their own independent oracle.

Qualification records the independent reference and why it is materially independent.

### Q6 — release candidate

Q6 is BLOCKED if an applicable M1/M3/M5/M6 reuse disposition is absent or if DEFERRED_WITH_IMPACT remains release-blocking.

Retained reports created before adoption of this gate remain historical evidence. New requalification/release-candidate claims apply the current policy.

## Perfectπ and LMES source boundaries

- Perfectπ is read-only from Perfect Foundations qualification work.
- The LMES GitHub mirror is a reference snapshot, not authority for current LMES GitLab state.
- Qualification may consume predecessor lessons without importing source-project domain semantics.
- Source verification does not transfer automatically to the destination revision.

## Required report fields

A qualification report involving this gate records:

- target repo/SHA/configuration;
- source predecessor/revision;
- destination disposition;
- affected requirement/ADR;
- verification method and independent oracle;
- PASS/FAIL/BLOCKED/INCONCLUSIVE/NOT_APPLICABLE outcome;
- limitations;
- remaining deferred impact.
