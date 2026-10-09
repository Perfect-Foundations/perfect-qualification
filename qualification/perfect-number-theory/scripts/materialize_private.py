#!/usr/bin/env python3
"""Redirect exact pinned private Git sources only in disposable workspaces."""
from __future__ import annotations

import os
from pathlib import Path
import subprocess
import tomllib

REVISIONS = {
    "perfect-number-theory": "41aaa4763da2d11aea9e65666f9ed7f84d43ed61",
    "perfect-arithmetic": "9ec660f9abfd7fe93e441e77dbb1158c371603f5",
    "perfect-numeric": "19b6747cd852a47a694a020b97ba70b6b3ef259b",
}
ENV = {
    "perfect-number-theory": "NUMBER_THEORY_CARGO_PATH",
    "perfect-arithmetic": "ARITHMETIC_CARGO_PATH",
    "perfect-numeric": "NUMERIC_CARGO_PATH",
}

def source(name: str) -> Path:
    raw = os.environ.get(ENV[name])
    if not raw:
        raise SystemExit(f"Missing {ENV[name]}")
    path = Path(raw).resolve(strict=True)
    if not (path / ".git").exists():
        raise SystemExit(f"Not a separate Git source checkout: {name}")
    actual = subprocess.check_output(
        ["git", "-C", str(path), "rev-parse", "HEAD"], text=True
    ).strip()
    if actual != REVISIONS[name]:
        raise SystemExit(f"Wrong {name} source SHA: {actual}")
    return path

def replace(manifest: Path, name: str, pin: str, local: Path) -> None:
    old = (
        f'{name} = {{ git = "https://github.com/Perfect-Foundations/{name}.git", '
        f'rev = "{pin}", version = "=0.1.0", default-features = false }}'
    )
    new = (
        f'{name} = {{ path = "{local.as_posix()}", '
        'version = "=0.1.0", default-features = false }'
    )
    current = manifest.read_text(encoding="utf-8")
    if current.count(old) != 1:
        raise SystemExit(f"Expected exactly one pristine {name} pin in {manifest}")
    updated = current.replace(old, new)
    parsed = tomllib.loads(updated)
    assert parsed["dependencies"][name]["path"] == local.as_posix()
    manifest.write_text(updated, encoding="utf-8", newline="\n")

def main() -> None:
    consumer_raw = os.environ.get("QUAL_MANIFEST")
    if not consumer_raw:
        raise SystemExit("Missing QUAL_MANIFEST")
    manifest = Path(consumer_raw).resolve(strict=True)
    number_theory = source("perfect-number-theory")
    arithmetic = source("perfect-arithmetic")
    numeric = source("perfect-numeric")
    # Confirm every original declaration before making *any* changes.
    checks = [
        (manifest, "perfect-number-theory", number_theory),
        (manifest, "perfect-arithmetic", arithmetic),
        (number_theory / "Cargo.toml", "perfect-arithmetic", arithmetic),
        (arithmetic / "Cargo.toml", "perfect-numeric", numeric),
    ]
    for path, dependency, _ in checks:
        expected = (
            f'{dependency} = {{ git = "https://github.com/Perfect-Foundations/{dependency}.git", '
            f'rev = "{REVISIONS[dependency]}", version = "=0.1.0", default-features = false }}'
        )
        if path.read_text(encoding="utf-8").count(expected) != 1:
            raise SystemExit(f"Pristine dependency pin missing: {path} {dependency}")
    for path, dependency, location in checks:
        replace(path, dependency, REVISIONS[dependency], location)
    print("NT_PRIVATE_SOURCE_EXACT_PINS_PASS", *REVISIONS.values())

if __name__ == "__main__":
    main()
