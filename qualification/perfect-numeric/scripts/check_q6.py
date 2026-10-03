#!/usr/bin/env python3
"""Fail-closed Q6 release-candidate metadata/security checker for Perfect Numeric."""

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
            problem(f"required release file missing: {relative}")
            return None
        return path

    manifest_path = require_file("Cargo.toml")
    if manifest_path is None:
        for message in problems:
            print(f"Q6 BLOCKER: {message}", file=sys.stderr)
        return 1

    try:
        with manifest_path.open("rb") as handle:
            manifest = tomllib.load(handle)
    except (OSError, tomllib.TOMLDecodeError) as error:
        problem(f"Cargo.toml cannot be parsed: {error}")
        manifest = {}

    package = manifest.get("package")
    if not isinstance(package, dict):
        problem("Cargo.toml has no [package] table")
        package = {}

    if package.get("name") != "perfect-numeric":
        problem("unexpected package name")

    for key in ("version", "rust-version", "description", "repository", "documentation", "readme"):
        value = package.get(key)
        if not isinstance(value, str) or not value.strip() or value.strip().upper() == "TBD":
            problem(f"package metadata {key!r} is missing or unresolved")

    if package.get("publish") is False:
        problem("publish = false still blocks release-candidate publication")

    license_expr = package.get("license")
    license_file = package.get("license-file")
    if license_expr and license_file:
        problem("set either package.license or package.license-file, not both")
    elif isinstance(license_expr, str):
        if not license_expr.strip() or license_expr.strip().upper() == "TBD":
            problem("package license expression is unresolved")
    elif isinstance(license_file, str):
        if not license_file.strip():
            problem("package license-file is empty")
        elif not (root / license_file).is_file():
            problem(f"license-file does not exist: {license_file}")
    else:
        problem("project license is unresolved; set package.license or package.license-file")

    keywords = package.get("keywords")
    categories = package.get("categories")
    if not isinstance(keywords, list) or not keywords:
        problem("package keywords are missing")
    if not isinstance(categories, list) or not categories:
        problem("package categories are missing")

    for relative in (
        "README.md",
        "CHANGELOG.md",
        "SECURITY.md",
        "MAINTENANCE.md",
        "docs/STABILITY.md",
        "docs/PACKAGING.md",
        "docs/SUPPLY-CHAIN-REVIEW.md",
    ):
        require_file(relative)

    security_path = root / "SECURITY.md"
    if security_path.is_file():
        security = security_path.read_text(encoding="utf-8").lower()
        normalized_security = re.sub(r"[^a-z0-9]+", " ", security)
        unresolved_security_phrases = (
            "not yet been verified",
            "has not yet been verified",
            "not yet verified",
            "public facing private vulnerability reporting channel has not",
        )
        if any(phrase in normalized_security for phrase in unresolved_security_phrases):
            problem("SECURITY.md still records the vulnerability-reporting channel as unverified")
        if "report" not in normalized_security or "vulnerab" not in normalized_security:
            problem("SECURITY.md does not contain concrete vulnerability-reporting instructions")

    if problems:
        for message in problems:
            print(f"Q6 BLOCKER: {message}", file=sys.stderr)
        print(f"Q6 FAIL: {len(problems)} release-candidate prerequisite(s) unresolved", file=sys.stderr)
        return 1

    print("Q6 PASS: release-candidate metadata/security document prerequisites are resolved")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
