#!/usr/bin/env python3
"""Fail-closed verification and materialization into a throwaway manifest only.

The committed Cargo.toml retains the authentic pinned Git dependency.
The temporary copy preserves the unmodified manifest beside its local rewrite.
No registry package or published lockfile identity is implied.
"""
from pathlib import Path
import hashlib
import json
import os
import subprocess
import tomllib

PIN="d88d8055f45fb075c905f1fe9e82a623029faf41"
URL="https://github.com/Perfect-Foundations/perfect-constants.git"

def main():
    root=Path(__file__).resolve().parents[1]
    manifest=root/"Cargo.toml"
    original=manifest.read_text(encoding="utf-8")
    fields=tomllib.loads(original)
    dep=fields["dependencies"]["perfect-constants"]
    assert dep["git"]==URL and dep["rev"]==PIN and dep["version"]=="=0.1.0"
    source=os.environ.get("CONSTANTS_CARGO_PATH","")
    assert source,"CONSTANTS_CARGO_PATH is required for offline materialization"
    source=Path(source).resolve(strict=True)
    assert (source/".git").exists(), "verified source must be a Git clone"
    head=subprocess.check_output(["git","-C",str(source),"rev-parse","HEAD"],text=True).strip()
    assert head==PIN,(head,PIN)
    assert not subprocess.check_output(["git","-C",str(source),"status","--porcelain"],text=True).strip(),"source checkout must be clean"
    pkg=tomllib.loads((source/"Cargo.toml").read_text(encoding="utf-8"))["package"]
    assert pkg["name"]=="perfect-constants" and pkg["version"]=="0.1.0"
    original_copy=root/"Cargo.toml.original-git-dependency"
    assert not original_copy.exists(),"already materialized"
    needle=f'git = "{URL}", rev = "{PIN}", version = "=0.1.0"'
    assert original.count(needle)==1
    exact_original=hashlib.sha256(original.encode("utf-8")).hexdigest()
    original_copy.write_text(original,encoding="utf-8")
    rewritten=original.replace(needle,"path = "+json.dumps(source.as_posix())+', version = "=0.1.0"')
    assert tomllib.loads(rewritten)["dependencies"]["perfect-constants"]["path"]==source.as_posix()
    manifest.write_text(rewritten,encoding="utf-8")
    print("PINNED_ORIGINAL_GIT_DEP",URL,PIN)
    print("ORIGINAL_MANIFEST_SHA256",exact_original)
    print("VERIFIED_CLONED_SOURCE",head)
    print("LOCAL_PATH_MATERIALIZED_IN_THROWAWAY_COPY_NOT_REGISTRY")
if __name__=="__main__":main()
