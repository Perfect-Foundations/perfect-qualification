#!/usr/bin/env python3
"""Redirect immutable Perfect-family Git pins into isolated qualification source snapshots.

No registry substitution: each source is required by the workflow at its pinned
Git revision, and all private dependency declarations are checked before edits.
"""
import os
from pathlib import Path
import subprocess

PINS={
"perfect-decimal":"ce9ff7c91beb9b1ee10e088d65465302b68eecc7",
"perfect-numeric":"19b6747cd852a47a694a020b97ba70b6b3ef259b",
"perfect-arithmetic":"1a54d3c7cbbae4e73325cc70fd2777a2427b1504",
"perfect-rational":"97cce0ae8ff7ef206929d273e1d4a5774f6a2a34",
}
roots={key:Path(os.environ[key.split("-")[1].upper()+"_CARGO_PATH"]) for key in PINS}
qualification=Path(os.environ["QUAL_MANIFEST"])
decimal_shell=Path(os.environ["DECIMAL_SHELL_PATH"])
head=subprocess.check_output(["git","-C",str(decimal_shell),"rev-parse","HEAD"],text=True).strip()
if head!=PINS["perfect-decimal"]:
    raise SystemExit(f"Unexpected Decimal checkout revision: {head}")
replacements={
qualification:list(PINS),
roots["perfect-decimal"]:["perfect-numeric","perfect-arithmetic","perfect-rational"],
roots["perfect-arithmetic"]:["perfect-numeric"],
roots["perfect-rational"]:["perfect-arithmetic"],
}
prepared={}
for manifest_base,names in replacements.items():
    path=manifest_base if manifest_base.name=="Cargo.toml" else manifest_base/"Cargo.toml"
    current=path.read_text(encoding="utf-8")
    updated=current
    for name in names:
        pin=PINS[name]
        expected=f'{name} = {{ git = "https://github.com/Perfect-Foundations/{name}.git", rev = "{pin}", version = "=0.1.0", default-features = false }}'
        target=str(roots[name]).replace("\\","/")
        replacement=f'{name} = {{ path = "{target}", version = "=0.1.0", default-features = false }}'
        if updated.count(expected)!=1:
            raise SystemExit(f"Expected exactly one pinned {name} dependency in {path}")
        updated=updated.replace(expected,replacement,1)
    prepared[path]=updated
for path,content in prepared.items():
    path.write_text(content,encoding="utf-8",newline="\n")
    print("VERIFIED_DEPENDENCY_REDIRECT",path)
print("EXACT_DECIMAL_SHA",head)
