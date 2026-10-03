#!/usr/bin/env python3
"""Fail-closed private Q6 release-candidate checker for Perfect Numeric.

Q6 qualifies a retained private candidate. Public opening, licensing, package
publication, and a public-facing vulnerability-reporting channel are separate
open-release gates and may remain intentionally deferred.
"""

from __future__ import annotations

import argparse
from pathlib import Path
import re
import sys
import tomllib


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
            problem(f"required release-candidate file missing: {relative}")
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
        problem(f"release-candidate metadata cannot be parsed: {error}")
        manifest = {}
        status = {}

    package = manifest.get("package")
    if not isinstance(package, dict):
        problem("Cargo.toml has no [package] table")
        package = {}

    if package.get("name") != "perfect-numeric":
        problem("unexpected package name")

    for key in ("version", "rust-version", "description", "repository", "documentation", "readme"):
        value = package.get(key)
        if not isinstance(value, str) or not value.strip():
            problem(f"package metadata {key!r} is missing")

    if package.get("publish") is not False:
        problem("private Q6 candidate must retain publish = false until explicit open-release authorization")

    keywords = package.get("keywords")
    categories = package.get("categories")
    if not isinstance(keywords, list) or not keywords:
        problem("package keywords are missing")
    if not isinstance(categories, list) or not categories:
        problem("package categories are missing")

    release = status.get("release")
    if not isinstance(release, dict):
        problem("project-status.toml has no [release] table")
        release = {}
    if release.get("release_status") != "unreleased":
        problem("private Q6 candidate must remain unreleased")
    if release.get("last_release") not in ("", None):
        problem("private Q6 candidate unexpectedly records a prior release")

    for relative in (
        "README.md",
        "CHANGELOG.md",
        "SECURITY.md",
        "MAINTENANCE.md",
        "docs/STABILITY.md",
        "docs/PACKAGING.md",
        "docs/SUPPLY-CHAIN-REVIEW.md",
        "docs/PERFORMANCE.md",
        "docs/verification/M6-PRERELEASE-REVIEW.md",
        "verification/host-benchmark-2026-10-03.json",
        "verification/semantic-fingerprint.txt",
    ):
        require_file(relative)

    security_path = root / "SECURITY.md"
    if security_path.is_file():
        normalized = re.sub(
            r"[^a-z0-9]+",
            " ",
            security_path.read_text(encoding="utf-8").lower(),
        )
        unresolved = any(
            phrase in normalized
            for phrase in (
                "not yet been verified",
                "has not yet been verified",
                "not yet verified",
                "public facing private vulnerability reporting channel has not",
            )
        )
        if unresolved:
            if "before perfect numeric or its package is made public" not in normalized:
                problem(
                    "deferred vulnerability reporting is not explicitly tied to the public-release gate"
                )
        elif "report" not in normalized or "vulnerab" not in normalized:
            problem("SECURITY.md lacks concrete vulnerability-reporting instructions")

    review_path = root / "docs/verification/M6-PRERELEASE-REVIEW.md"
    if review_path.is_file():
        review = re.sub(
            r"[^a-z0-9]+",
            " ",
            review_path.read_text(encoding="utf-8").lower(),
        )
        for phrase in (
            "project s own license remains tbd",
            "security handling open",
            "docs rs crates io release quality open",
        ):
            if phrase not in review:
                problem(f"M6 review does not retain deferred open-release gate: {phrase}")

    if problems:
        for message in problems:
            print(f"Q6 BLOCKER: {message}", file=sys.stderr)
        print(f"Q6 FAIL: {len(problems)} private release-candidate prerequisite(s) unresolved", file=sys.stderr)
        return 1

    license_value = release.get("license", "TBD")
    print(f"Q6 INFO: project license remains {license_value!r}; public licensing is deferred")
    print("Q6 INFO: publication remains disabled by design")
    print("Q6 INFO: public vulnerability-reporting verification remains an open-release gate")
    print("Q6 PASS: private release-candidate documentation/retention prerequisites are satisfied")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
