#!/usr/bin/env python3
"""Fail-closed Q6 release-candidate metadata/security checker for Perfect Numeric."""

from __future__ import annotations

import argparse
from pathlib import Path
import sys
import tomllib


def fail(message: str) -> None:
    print(f"Q6 FAIL: {message}", file=sys.stderr)
    raise SystemExit(1)


def require_file(root: Path, relative: str) -> Path:
    path = root / relative
    if not path.is_file():
        fail(f"required release file missing: {relative}")
    return path


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("source", type=Path)
    args = parser.parse_args()
    root = args.source.resolve()

    manifest_path = require_file(root, "Cargo.toml")
    with manifest_path.open("rb") as handle:
        manifest = tomllib.load(handle)

    package = manifest.get("package")
    if not isinstance(package, dict):
        fail("Cargo.toml has no [package] table")

    if package.get("name") != "perfect-numeric":
        fail("unexpected package name")

    for key in ("version", "rust-version", "description", "repository", "documentation", "readme"):
        value = package.get(key)
        if not isinstance(value, str) or not value.strip() or value.strip().upper() == "TBD":
            fail(f"package metadata {key!r} is missing or unresolved")

    if package.get("publish") is False:
        fail("publish = false still blocks release-candidate publication")

    license_expr = package.get("license")
    license_file = package.get("license-file")
    if license_expr and license_file:
        fail("set either package.license or package.license-file, not both")
    if isinstance(license_expr, str):
        if not license_expr.strip() or license_expr.strip().upper() == "TBD":
            fail("package license expression is unresolved")
    elif isinstance(license_file, str):
        if not license_file.strip():
            fail("package license-file is empty")
        require_file(root, license_file)
    else:
        fail("project license is unresolved; set package.license or package.license-file")

    keywords = package.get("keywords")
    categories = package.get("categories")
    if not isinstance(keywords, list) or not keywords:
        fail("package keywords are missing")
    if not isinstance(categories, list) or not categories:
        fail("package categories are missing")

    for relative in (
        "README.md",
        "CHANGELOG.md",
        "SECURITY.md",
        "MAINTENANCE.md",
        "docs/STABILITY.md",
        "docs/PACKAGING.md",
        "docs/SUPPLY-CHAIN-REVIEW.md",
    ):
        require_file(root, relative)

    security = (root / "SECURITY.md").read_text(encoding="utf-8").lower()
    unresolved_security_phrases = (
        "not yet been verified",
        "has not yet been verified",
        "not yet verified",
        "public-facing private vulnerability-reporting channel has not",
    )
    if any(phrase in security for phrase in unresolved_security_phrases):
        fail("SECURITY.md still records the vulnerability-reporting channel as unverified")
    if "report" not in security or "vulnerab" not in security:
        fail("SECURITY.md does not contain concrete vulnerability-reporting instructions")

    print("Q6 PASS: release-candidate metadata/security document prerequisites are resolved")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
