#!/usr/bin/env python3
"""Render a fail-closed Perfect Arithmetic Q6 qualification report."""

from __future__ import annotations

import argparse
import json
import os
from pathlib import Path
import re
import tomllib

ORDERED_JOBS = (
    ("quality", "Q6 Quality / MSRV"),
    ("performance", "Candidate-bound performance"),
    ("fuzz", "Candidate-bound fuzz"),
    ("mutation", "Candidate-bound mutation"),
    ("class-a", "Class A matrix"),
    ("class-b", "Class B matrix"),
)

EXPECTED_LEGS = (
    ("quality", "Q6 Quality / MSRV", "quality"),
    ("performance", "Candidate-bound performance", "performance"),
    ("fuzz", "Candidate-bound fuzz", "fuzz"),
    ("mutation", "Candidate-bound mutation", "mutation"),
    ("class-a-ubuntu-24.04", "Class A / ubuntu-24.04", "class-a"),
    ("class-a-ubuntu-24.04-arm", "Class A / ubuntu-24.04-arm", "class-a"),
    ("class-a-windows-2025", "Class A / windows-2025", "class-a"),
    ("class-b-wasm32v1-none", "Class B / wasm32v1-none", "class-b"),
    ("class-b-thumbv7em-none-eabihf", "Class B / thumbv7em-none-eabihf", "class-b"),
    (
        "class-b-riscv64imac-unknown-none-elf",
        "Class B / riscv64imac-unknown-none-elf",
        "class-b",
    ),
)

REQUIRED_REUSE_SECTIONS = (
    "Destination binding",
    "Source revisions",
    "Dispositions",
    "Destination traceability and independent verification",
    "Deferred-impact state",
    "Current M6 effect",
    "Recheck",
)


def section_body(raw: str, heading: str) -> str | None:
    match = re.search(
        rf"(?ms)^## {re.escape(heading)}\s*\n(.*?)(?=^## |\Z)",
        raw,
    )
    return match.group(1).strip() if match is not None else None


def markdown_table(body: str) -> list[dict[str, str]]:
    rows = [
        line.strip()
        for line in body.splitlines()
        if line.strip().startswith("|")
    ]
    if len(rows) < 3:
        return []

    headers = [cell.strip() for cell in rows[0].strip("|").split("|")]
    parsed: list[dict[str, str]] = []
    for line in rows[2:]:
        cells = [cell.strip() for cell in line.strip("|").split("|")]
        if len(cells) != len(headers):
            continue
        parsed.append(
            {
                header: re.sub(r"\*\*", "", value).strip()
                for header, value in zip(headers, cells)
            }
        )
    return parsed


def bullet_items(body: str) -> list[str]:
    return [
        line[2:].strip()
        for line in body.splitlines()
        if line.startswith("- ")
    ]


def parse_historical_reuse(
    target_root: Path,
    checkout_ok: bool,
    blockers: list[str],
) -> dict[str, object]:
    review_path = target_root / "docs" / "PROVEN-REUSE-REVIEW.md"
    if not checkout_ok or not review_path.is_file():
        blockers.append(
            "exact target historical proven-reuse review was unavailable to the report job"
        )
        return {}

    try:
        raw = review_path.read_text(encoding="utf-8")
    except OSError as error:
        blockers.append(
            f"exact target historical proven-reuse review could not be read: {error}"
        )
        return {}

    sections: dict[str, str] = {}
    for heading in REQUIRED_REUSE_SECTIONS:
        body = section_body(raw, heading)
        if body is None:
            blockers.append(
                f"historical proven-reuse review missing report section: {heading}"
            )
        else:
            sections[heading] = body

    if len(sections) != len(REQUIRED_REUSE_SECTIONS):
        return {}

    source_revisions = bullet_items(sections["Source revisions"])
    dispositions = markdown_table(sections["Dispositions"])
    traceability = markdown_table(
        sections["Destination traceability and independent verification"]
    )
    deferred_impact = bullet_items(sections["Deferred-impact state"])
    recheck = bullet_items(sections["Recheck"])

    if not source_revisions:
        blockers.append(
            "historical proven-reuse report has no predecessor revision entries"
        )
    if not dispositions:
        blockers.append(
            "historical proven-reuse report has no destination dispositions"
        )
    if not traceability:
        blockers.append(
            "historical proven-reuse report has no requirement/ADR verification mapping"
        )
    if not deferred_impact:
        blockers.append(
            "historical proven-reuse report has no explicit deferred-impact disposition"
        )
    if not recheck:
        blockers.append(
            "historical proven-reuse report has no explicit mandatory recheck trigger"
        )

    required_disposition_fields = (
        "Source lesson",
        "Disposition",
        "Arithmetic consequence",
    )
    required_traceability_fields = (
        "Reuse item",
        "Affected requirement / ADR / gate",
        "Independent verification / reference method",
    )

    for field in required_disposition_fields:
        if dispositions and any(not row.get(field, "").strip() for row in dispositions):
            blockers.append(
                f"historical proven-reuse disposition table lacks nonempty field: {field}"
            )
    for field in required_traceability_fields:
        if traceability and any(not row.get(field, "").strip() for row in traceability):
            blockers.append(
                f"historical proven-reuse traceability table lacks nonempty field: {field}"
            )

    return {
        "destination_binding": sections["Destination binding"],
        "source_revisions": source_revisions,
        "dispositions": dispositions,
        "traceability_and_independent_verification": traceability,
        "deferred_impact": deferred_impact,
        "limitations": sections["Current M6 effect"],
        "recheck": recheck,
    }


def parse_status(
    target_root: Path,
    checkout_ok: bool,
    blockers: list[str],
) -> dict:
    status_path = target_root / "project-status.toml"
    if not checkout_ok or not status_path.is_file():
        blockers.append(
            "exact target project-status.toml was unavailable to the report job"
        )
        return {}

    try:
        with status_path.open("rb") as handle:
            return tomllib.load(handle)
    except (OSError, tomllib.TOMLDecodeError) as error:
        blockers.append(
            f"exact target project-status.toml could not be parsed: {error}"
        )
        return {}


def parse_needs(needs_json: str) -> tuple[dict[str, str], str]:
    try:
        needs = json.loads(needs_json or "{}")
    except json.JSONDecodeError as error:
        needs = {}
        decode_error = str(error)
    else:
        decode_error = ""

    raw_results = {
        job_id: (needs.get(job_id) or {}).get("result", "skipped")
        for job_id, _ in ORDERED_JOBS
    }
    return raw_results, decode_error


def load_leg_states(
    state_root: Path,
    blockers: list[str],
) -> dict[str, dict[str, str]]:
    states: dict[str, dict[str, str]] = {}
    valid_states = {"PASS", "FAIL", "BLOCKED"}

    for key, label, job_id in EXPECTED_LEGS:
        path = state_root / f"{key}.json"
        if not path.is_file():
            states[key] = {
                "label": label,
                "job_id": job_id,
                "qualification_state": "NOT-RUN",
                "failure_present": False,
            }
            continue

        try:
            payload = json.loads(path.read_text(encoding="utf-8"))
        except (OSError, json.JSONDecodeError) as error:
            blockers.append(f"malformed Q6 job-state artifact {path}: {error}")
            states[key] = {
                "label": label,
                "job_id": job_id,
                "qualification_state": "BLOCKED",
                "failure_present": False,
            }
            continue

        state = payload.get("state")
        if state not in valid_states:
            blockers.append(
                f"Q6 job-state artifact {path} has invalid state {state!r}"
            )
            state = "BLOCKED"

        failure_present = payload.get("failure_present", False)
        if not isinstance(failure_present, bool):
            blockers.append(
                f"Q6 job-state artifact {path} has invalid failure_present value"
            )
            failure_present = True

        states[key] = {
            "label": label,
            "job_id": job_id,
            "qualification_state": state,
            "failure_present": failure_present,
        }

    return states


def append_reuse_markdown(
    lines: list[str],
    historical_reuse: dict[str, object],
) -> None:
    lines.extend(["", "## Historical proven-reuse evidence", ""])

    if not historical_reuse:
        lines.append(
            "- Historical proven-reuse evidence unavailable; see blocker classification."
        )
        return

    lines.extend(
        [
            "### Destination binding",
            "",
            str(historical_reuse["destination_binding"]),
            "",
            "### Predecessor source revisions",
            "",
        ]
    )
    lines.extend(
        f"- {item}"
        for item in historical_reuse["source_revisions"]
    )

    lines.extend(
        [
            "",
            "### Destination dispositions",
            "",
            "| Source lesson | Disposition | Arithmetic consequence |",
            "| --- | --- | --- |",
        ]
    )
    for row in historical_reuse["dispositions"]:
        lines.append(
            f"| {row.get('Source lesson', '')} | "
            f"{row.get('Disposition', '')} | "
            f"{row.get('Arithmetic consequence', '')} |"
        )

    lines.extend(
        [
            "",
            "### Affected requirements / ADRs and independent verification",
            "",
            "| Reuse item | Affected requirement / ADR / gate | Independent verification / reference method |",
            "| --- | --- | --- |",
        ]
    )
    for row in historical_reuse["traceability_and_independent_verification"]:
        lines.append(
            f"| {row.get('Reuse item', '')} | "
            f"{row.get('Affected requirement / ADR / gate', '')} | "
            f"{row.get('Independent verification / reference method', '')} |"
        )

    lines.extend(
        [
            "",
            "### Limitations",
            "",
            str(historical_reuse["limitations"]),
            "",
            "### Remaining deferred impact",
            "",
        ]
    )
    deferred = historical_reuse["deferred_impact"]
    if deferred:
        lines.extend(f"- {item}" for item in deferred)
    else:
        lines.append("- None recorded.")

    lines.extend(["", "### Mandatory recheck", ""])
    recheck = historical_reuse["recheck"]
    if recheck:
        lines.extend(f"- {item}" for item in recheck)
    else:
        lines.append("- No recheck trigger recorded.")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--target-root", type=Path, required=True)
    parser.add_argument("--state-root", type=Path, required=True)
    args = parser.parse_args()

    raw_job_results, needs_decode_error = parse_needs(
        os.environ.get("NEEDS_JSON", "{}")
    )

    blockers: list[str] = []
    leg_states = load_leg_states(args.state_root, blockers)
    checkout_ok = os.environ.get("TARGET_STATUS_CHECKOUT") == "success"
    status = parse_status(args.target_root, checkout_ok, blockers)
    historical_reuse = parse_historical_reuse(
        args.target_root,
        checkout_ok,
        blockers,
    )

    if status:
        work = status.get("work", {})
        engineering = status.get("engineering", {})
        critical_blockers = int(work.get("critical_blockers", 0) or 0)
        if critical_blockers > 0:
            blockers.append(
                f"project-status work.critical_blockers={critical_blockers}"
            )

        for key in ("cargo_package", "offline_build"):
            value = engineering.get(key)
            if isinstance(value, str) and value.lower().startswith("blocked"):
                blockers.append(f"project-status engineering.{key}={value}")

    blocked_legs = [
        data["label"]
        for data in leg_states.values()
        if data["qualification_state"] == "BLOCKED"
    ]
    failures = [
        data["label"]
        for data in leg_states.values()
        if data["qualification_state"] == "FAIL"
        or data.get("failure_present", False)
    ]
    not_run = [
        data["label"]
        for data in leg_states.values()
        if data["qualification_state"] == "NOT-RUN"
    ]

    if blocked_legs:
        blockers.extend(
            f"qualification leg blocked: {label}" for label in blocked_legs
        )

    if blockers:
        overall = "BLOCKED"
    elif failures:
        overall = "FAIL"
    elif not_run:
        overall = "NOT-RUN"
    else:
        overall = "PASS"

    target_sha = os.environ["TARGET_SHA"]
    qualification_sha = os.environ["QUALIFICATION_SHA"]
    run_id = os.environ["RUN_ID"]
    run_attempt = os.environ["RUN_ATTEMPT"]

    lines = [
        "# Perfect Arithmetic Q6 Qualification Report",
        "",
        "## Exact identity",
        "",
        f"- Target repository: `{os.environ['TARGET_REPOSITORY']}`",
        f"- Target SHA: `{target_sha}`",
        f"- Qualification repository: `{os.environ['QUALIFICATION_REPOSITORY']}`",
        f"- Qualification workflow SHA: `{qualification_sha}`",
        f"- Caller repository: `{os.environ['CALLER_REPOSITORY']}`",
        f"- Caller SHA: `{os.environ['CALLER_SHA']}`",
        f"- GitHub run: `{run_id}`, attempt `{run_attempt}`",
        "",
        "## Exact configuration",
        "",
        "- MSRV: Rust 1.99.0; pinned stable replay: Rust 1.99.0.",
        "- Fuzz toolchain: nightly-2026-10-04, cargo-fuzz 0.13.2.",
        "- Mutation tool: cargo-mutants 27.1.0 with candidate config suppression disabled.",
        "- Class A: ubuntu-24.04, ubuntu-24.04-arm, windows-2025.",
        "- Class B: wasm32v1-none, thumbv7em-none-eabihf, riscv64imac-unknown-none-elf.",
        "- Candidate, consumer, performance, and fuzz dependency graphs use retained locks.",
    ]

    append_reuse_markdown(lines, historical_reuse)

    lines.extend(
        [
            "",
            "## Gates attempted and qualification states",
            "",
            "| Execution leg | Qualification state |",
            "| --- | --- |",
        ]
    )
    for key, _, _ in EXPECTED_LEGS:
        data = leg_states[key]
        lines.append(
            f"| {data['label']} | **{data['qualification_state']}** |"
        )

    lines.extend(
        [
            "",
            "### GitHub aggregate job results",
            "",
            "| Job | Raw GitHub result |",
            "| --- | --- |",
        ]
    )
    for job_id, label in ORDERED_JOBS:
        lines.append(f"| {label} | `{raw_job_results[job_id]}` |")

    lines.extend(
        [
            "",
            "Qualification-state labels come from retained per-leg state artifacts. "
            "A missing state artifact is NOT-RUN and is never promoted to PASS.",
            "",
            "## Qualification classification",
            "",
            f"**{overall}**",
            "",
            "### Failures",
            "",
        ]
    )
    if failures:
        lines.extend(f"- {item}" for item in failures)
    else:
        lines.append("- None recorded by aggregate job results.")

    lines.extend(["", "### Blockers", ""])
    if blockers:
        lines.extend(f"- {item}" for item in blockers)
    else:
        lines.append(
            "- None recorded from independent pre-Q6 target-status/report prerequisites."
        )

    lines.extend(["", "### Not run", ""])
    if not_run:
        lines.extend(f"- {item}" for item in not_run)
    else:
        lines.append("- None.")

    lines.extend(
        [
            "",
            "### Not applicable",
            "",
            "- None for the configured Q6 job set.",
            "",
            "## Evidence retained",
            "",
            f"- Performance artifact: `q6-performance-{run_id}-{run_attempt}` "
            "when the performance runner produced files.",
            f"- Overall report artifact: `q6-qualification-report-{run_id}-{run_attempt}`.",
            "- Exact commands, versions, target matrices, lockfiles, and job logs remain revision/run bound.",
            "",
            "## Remaining unqualified work",
            "",
        ]
    )

    remaining = [
        data["label"]
        for data in leg_states.values()
        if data["qualification_state"] != "PASS"
    ]
    if remaining:
        lines.extend(f"- {item}" for item in remaining)
    else:
        lines.append("- No configured Q6 execution job remains non-passing.")
    if blockers:
        lines.append(
            "- Independent pre-Q6 blocker(s) listed above remain unresolved."
        )

    lines.extend(
        [
            "- Public-release gates outside this private Q6 run remain governed "
            "by the exact target `project-status.toml`; this report does not "
            "release or publish the crate.",
            "",
            "## Next safe action",
            "",
        ]
    )

    if overall == "BLOCKED":
        next_action = (
            "Resolve the authoritative blocker(s) without treating unavailable "
            "evidence as a pass, then rerun Q6 on the exact resulting target and "
            "qualification revisions."
        )
    elif overall == "FAIL":
        next_action = (
            "Investigate and correct the failing qualification job(s), then rerun "
            "the exact target/configuration; do not award Q6 from this report."
        )
    elif overall == "NOT-RUN":
        next_action = (
            "Restore the unavailable/cancelled execution path and rerun the exact "
            "target/configuration."
        )
    else:
        next_action = (
            "Review and retain this report plus all evidence artifacts before "
            "changing authoritative status; a PASS report alone does not publish "
            "or release the crate."
        )
    lines.append(next_action)

    if needs_decode_error:
        lines.extend(
            [
                "",
                "## Report-generation warning",
                "",
                f"- Could not decode the `needs` context: {needs_decode_error}",
            ]
        )

    report_text = "\n".join(lines) + "\n"
    Path("q6-qualification-report.md").write_text(
        report_text,
        encoding="utf-8",
    )

    machine = {
        "schema_version": 2,
        "target_repository": os.environ["TARGET_REPOSITORY"],
        "target_sha": target_sha,
        "qualification_repository": os.environ["QUALIFICATION_REPOSITORY"],
        "qualification_sha": qualification_sha,
        "caller_repository": os.environ["CALLER_REPOSITORY"],
        "caller_sha": os.environ["CALLER_SHA"],
        "run_id": run_id,
        "run_attempt": run_attempt,
        "raw_github_job_results": raw_job_results,
        "execution_legs": leg_states,
        "qualification_state": overall,
        "failures": failures,
        "blockers": blockers,
        "not_run": not_run,
        "not_applicable": [],
        "historical_reuse": historical_reuse,
        "next_safe_action": next_action,
    }
    Path("q6-qualification-report.json").write_text(
        json.dumps(machine, indent=2, sort_keys=True) + "\n",
        encoding="utf-8",
    )

    summary = os.environ.get("GITHUB_STEP_SUMMARY")
    if summary:
        with Path(summary).open("a", encoding="utf-8") as handle:
            handle.write(report_text)

    print(f"Q6_REPORT_STATE={overall}")
    return 0 if overall == "PASS" else 1


if __name__ == "__main__":
    raise SystemExit(main())
