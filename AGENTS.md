# Perfect Qualification Agent Instructions

This file is the authoritative repository instruction file for qualification agents.

## Mission
Perfect Qualification owns cross-family integration/qualification evidence. It is never a production dependency and never substitutes for a crate's own local verification.

## Non-negotiable truth rules
- Qualify only exact repository revisions and exact configurations.
- Never infer qualification from a green local test suite, another SHA, a similar target, or an older report.
- A report must distinguish pass, fail, blocked, not-run, and not-applicable.
- Never rewrite a gate to match an implementation.
- Never claim release readiness when open-release gates remain intentionally unperformed.
- Preserve failed/blocker reports as evidence; do not erase history to make status look cleaner.

## Required preflight
Before a qualification run:
- inspect live target repo SHA/status, its `project-status.toml`, requirements/ADRs/traceability, CI, packaging evidence, dependency SHAs, prior qualification reports, and the applicable family historical-reuse disposition;
- inspect this repository's current qualification contract and scripts;
- verify the target configuration is reproducible and the intended revision is immutable.

## Evidence discipline
Retain commands, versions, targets, features, inputs/vectors, result summaries, exact SHAs, and predecessor source/disposition provenance where applicable. Qualification proves only the tested configuration.

Required historical-reuse evidence follows `qualification/HISTORICAL-REUSE.md`. Missing required review evidence is BLOCKED, not PASS or NOT_APPLICABLE.

## Perfectπ
Perfectπ remains read-only. Qualification may consume/reference it but must not modify it.

## Concurrency/file integrity
Never discard concurrent work or force-push. Preserve UTF-8/line endings and report generation deterministically.

## Required report
State target repo/SHA, qualification gates attempted, exact configuration, evidence/results, failures/blockers, what remains unqualified, and the next safe action.
