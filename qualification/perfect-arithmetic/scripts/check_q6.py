#!/usr/bin/env python3
"""Fail-closed private Q6 prerelease checker for Perfect Arithmetic.

This checker validates the retained private-candidate contract. Public opening,
project licensing, crates.io/docs.rs publication, and public-facing
vulnerability reporting are later open-release gates.

The final Arithmetic package/offline-build gate is *not* waived here. It is
checked separately by the reusable Q6 workflow and remains blocked until the
required Perfect Numeric registry release exists.
"""

from __future__ import annotations

import argparse
from pathlib import Path
import re
import sys
import tomllib


ARITHMETIC_REV = "9ec660f9abfd7fe93e441e77dbb1158c371603f5"
NUMERIC_REV = "19b6747cd852a47a694a020b97ba70b6b3ef259b"


def normalized_text(path: Path) -> str:
    return re.sub(r"[^a-z0-9]+", " ", path.read_text(encoding="utf-8").lower())


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("source", type=Path)
    args = parser.parse_args()
    root = args.source.resolve()
    problems: list[str] = []

    def problem(message: str) -> None:
        problems.append(message)

    def require_file(relative: str) -> Path | None:
        path = root / relative
        if not path.is_file():
            problem(f"required Q6 file missing: {relative}")
            return None
        return path

    manifest_path = require_file("Cargo.toml")
    status_path = require_file("project-status.toml")
    if manifest_path is None or status_path is None:
        for message in problems:
            print(f"Q6 BLOCKER: {message}", file=sys.stderr)
        return 1

    try:
        with manifest_path.open("rb") as handle:
            manifest = tomllib.load(handle)
        with status_path.open("rb") as handle:
            status = tomllib.load(handle)
    except (OSError, tomllib.TOMLDecodeError) as error:
        problem(f"Q6 metadata cannot be parsed: {error}")
        manifest = {}
        status = {}

    package = manifest.get("package")
    if not isinstance(package, dict):
        problem("Cargo.toml has no [package] table")
        package = {}

    if package.get("name") != "perfect-arithmetic":
        problem("unexpected package name")
    if package.get("rust-version") != "1.99":
        problem("Q6 candidate must retain the verified Rust 1.99 MSRV declaration")

    for key in (
        "version",
        "rust-version",
        "description",
        "repository",
        "homepage",
        "documentation",
        "readme",
    ):
        value = package.get(key)
        if not isinstance(value, str) or not value.strip():
            problem(f"package metadata {key!r} is missing")

    if package.get("publish") is not False:
        problem("private Q6 candidate must retain publish = false")

    if not isinstance(package.get("keywords"), list) or not package.get("keywords"):
        problem("package keywords are missing")
    if not isinstance(package.get("categories"), list) or not package.get("categories"):
        problem("package categories are missing")

    lifecycle = status.get("lifecycle")
    if not isinstance(lifecycle, dict):
        problem("project-status.toml has no [lifecycle] table")
        lifecycle = {}
    if lifecycle.get("current_milestone") != "M6":
        problem("Q6 candidate is not recorded at M6")

    release = status.get("release")
    if not isinstance(release, dict):
        problem("project-status.toml has no [release] table")
        release = {}
    if release.get("release_status") != "unreleased":
        problem("private Q6 candidate must remain unreleased")
    if release.get("last_release") not in ("", None):
        problem("private Q6 candidate unexpectedly records a prior release")

    engineering = status.get("engineering")
    if not isinstance(engineering, dict):
        problem("project-status.toml has no [engineering] table")
        engineering = {}
    if engineering.get("qualification_status") not in (
        "q0-q5-passing-q6-open",
        "q0-q6-passing",
    ):
        problem("Q0-Q5 qualification status is not retained")

    for relative in (
        "README.md",
        "CHANGELOG.md",
        "SECURITY.md",
        "MAINTENANCE.md",
        "docs/STABILITY.md",
        "docs/PACKAGING.md",
        "docs/SUPPLY-CHAIN-REVIEW.md",
        "docs/PROVEN-REUSE-REVIEW.md",
        "docs/verification/M5-HARDENING.md",
        "docs/verification/M6-PRERELEASE-REVIEW.md",
        "docs/evidence/M3-PERFORMANCE-ARCHITECTURE.md",
        "verification/semantic-fingerprint.txt",
    ):
        require_file(relative)

    stability = root / "docs/STABILITY.md"
    if stability.is_file():
        text = normalized_text(stability)
        for phrase in (
            "exact arithmetic",
            "backend",
            "high throughput",
            "no std",
            "rust 1 99",
            "semantic compatibility",
        ):
            if phrase not in text:
                problem(f"stability contract missing required topic: {phrase}")

    supply_chain = root / "docs/SUPPLY-CHAIN-REVIEW.md"
    if supply_chain.is_file():
        text = normalized_text(supply_chain)
        for phrase in (
            "perfect numeric",
            "num bigint",
            "0 5 1",
            "num integer",
            "0 1 47",
            "dashu int",
            "0 6 2",
            "license",
            "advis",
            "build script",
            "ffi",
        ):
            if phrase not in text:
                problem(f"supply-chain review missing required topic: {phrase}")
        if NUMERIC_REV not in supply_chain.read_text(encoding="utf-8"):
            problem("supply-chain review does not retain the exact Perfect Numeric revision")

    security = root / "SECURITY.md"
    if security.is_file():
        text = normalized_text(security)
        unresolved = any(
            phrase in text
            for phrase in (
                "not yet been verified",
                "has not yet been verified",
                "not yet verified",
            )
        )
        if unresolved and "before perfect arithmetic or its package is made public" not in text:
            problem("deferred vulnerability reporting is not tied to the public-release gate")
        if "vulnerab" not in text or "report" not in text:
            problem("SECURITY.md lacks vulnerability-reporting policy")

    review = root / "docs/verification/M6-PRERELEASE-REVIEW.md"
    if review.is_file():
        text = normalized_text(review)
        for phrase in (
            "q0 q5",
            "q6 release candidate",
            "project license remains tbd",
            "security handling open",
            "docs rs crates io release quality open",
            "perfect numeric registry",
            "offline packaged build",
            "dependency license advisory review",
        ):
            if phrase not in text:
                problem(f"M6 review missing required retained state: {phrase}")

    packaging = root / "docs/PACKAGING.md"
    if packaging.is_file():
        text = normalized_text(packaging)
        for phrase in (
            "blocked",
            "perfect numeric",
            "registry",
            "cargo package",
            "offline",
        ):
            if phrase not in text:
                problem(f"packaging record missing blocker/sequence topic: {phrase}")

    reuse_review = root / "docs/PROVEN-REUSE-REVIEW.md"
    if reuse_review.is_file():
        raw = reuse_review.read_text(encoding="utf-8")
        text = normalized_text(reuse_review)
        for phrase in (
            "historical proven reuse review",
            "m6 catch up",
            "perfect main",
            "perfect specialized runtime pr 13 reviewed head",
            "lmes github mirror reference",
            "reuse method only",
            "evaluated rejected",
            "documentation disposition catch up only",
            "does not close q6",
        ):
            if phrase not in text:
                problem(f"historical reuse review missing required topic: {phrase}")
        for revision in (
            "d95e1f7864bd12bf58f38951680126aedbf63412",
            "bdf7baacc180ac8a245d67b5b81fc7861d40339b",
            "95a6cb30cae8cf4a35ff33878e2f9348d1471db5",
        ):
            if revision not in raw:
                problem(f"historical reuse review missing exact predecessor revision: {revision}")
        if "DEFERRED_WITH_IMPACT" in raw:
            problem("historical reuse review retains a release-blocking DEFERRED_WITH_IMPACT disposition")

    if problems:
        for message in problems:
            print(f"Q6 BLOCKER: {message}", file=sys.stderr)
        print(
            f"Q6 FAIL: {len(problems)} private release-candidate prerequisite(s) unresolved",
            file=sys.stderr,
        )
        return 1

    print(f"Q6 INFO: exact retained Arithmetic baseline is {ARITHMETIC_REV}")
    print(f"Q6 INFO: exact retained Perfect Numeric dependency is {NUMERIC_REV}")
    print(f"Q6 INFO: project license remains {release.get('license', 'TBD')!r}")
    print("Q6 INFO: public publication/security-channel gates may remain deferred")
    print("Q6 PASS: non-package private prerelease prerequisites are retained")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
