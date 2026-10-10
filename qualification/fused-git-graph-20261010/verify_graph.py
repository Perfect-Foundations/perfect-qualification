#!/usr/bin/env python3
"""Validate immutable Cargo source graph and retained lockfile (Python 3.9+)."""
import hashlib
import json
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parent
manifest = json.loads((ROOT / "source-exact.json").read_text(encoding="utf-8"))
cargo = (ROOT / "Cargo.toml").read_text(encoding="utf-8")
if "[patch." in cargo or "path =" in cargo:
    raise SystemExit("FAIL: source override or machine-local Cargo path")
lock = (ROOT / "Cargo.lock").read_bytes()
if hashlib.sha256(lock).hexdigest() != manifest["cargo_lock_sha256"]:
    raise SystemExit("FAIL: Cargo.lock differs from qualification manifest")
cmd = ["cargo", "+1.99.0", "metadata", "--locked", "--format-version", "1"]
metadata = json.loads(subprocess.run(cmd, cwd=ROOT, check=True, capture_output=True,
                                      text=True).stdout)
packages = metadata["packages"]
by_name = {name: [p for p in packages if p["name"] == name]
           for name in manifest["packages"]}
for name, sha in manifest["packages"].items():
    found = by_name[name]
    if len(found) != 1:
        raise SystemExit(f"FAIL: {name} has {len(found)} package identities")
    source = found[0]["source"] or ""
    if not (source.startswith("git+https://github.com/Perfect-Foundations/")
            and f"rev={sha}#{sha}" in source):
        raise SystemExit(f"FAIL: {name} resolved unexpected source: {source}")
nodes = {node["id"]: node for node in metadata["resolve"]["nodes"]}
arith_id = by_name["perfect-arithmetic"][0]["id"]
numeric_id = by_name["perfect-numeric"][0]["id"]
for owner in ["qualify-perfect-polynomial-fused-git",
              "perfect-polynomial", "perfect-rational"]:
    matches = [p for p in packages if p["name"] == owner]
    if len(matches) != 1 or arith_id not in nodes[matches[0]["id"]]["dependencies"]:
        raise SystemExit(f"FAIL: {owner} not dependent on unique Arithmetic source")
if numeric_id not in nodes[arith_id]["dependencies"]:
    raise SystemExit("FAIL: Arithmetic does not share pinned Numeric source")
poly_id = by_name["perfect-polynomial"][0]["id"]
rat_id = by_name["perfect-rational"][0]["id"]
if rat_id not in nodes[poly_id]["dependencies"]:
    raise SystemExit("FAIL: Polynomial lacks pinned Rational source")
if hashlib.sha256((ROOT / "Cargo.lock").read_bytes()).digest() != hashlib.sha256(lock).digest():
    raise SystemExit("FAIL: Cargo metadata changed frozen lock")
print("PASS immutable Git graph, one Arithmetic package, transitive identity, locked provenance")
for name, sha in manifest["packages"].items():
    print(name, sha)
