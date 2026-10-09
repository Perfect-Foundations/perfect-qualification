#!/usr/bin/env python3
"""Fail closed: exact 5-revision private source graph, disposable manifest rewriting.

Run ONLY on freshly cloned, throwaway qualification workspace. Never on a
production checkout; never on the worktree carrying an open PR.
"""
from pathlib import Path
import hashlib
import json
import os
import subprocess
import tomllib

PINS = {
 "perfect-interval": ("interval","30aca00817bb79f0be3bf43e96c767db9fe45bc5"),
 "perfect-float": ("float","bbd8b9e4aac0015facf0bb75819049cee68dc0e7"),
 "perfect-arithmetic": ("arithmetic","1a54d3c7cbbae4e73325cc70fd2777a2427b1504"),
 "perfect-rational": ("rational","35a8e9cc629ee578fe7b624e2134929ce7eeff8a"),
 "perfect-numeric": ("numeric","19b6747cd852a47a694a020b97ba70b6b3ef259b"),
}
def main():
    root=Path(os.environ["PFQ_EXACT_SOURCE_ROOT"]).resolve(strict=True)
    consumer=root/"qualification"/"qualification"/"perfect-interval"
    manifests=[consumer/"Cargo.toml"]+[
        root/d/"Cargo.toml" for d,_ in PINS.values()
    ]
    for name,(directory,expected) in PINS.items():
        source=root/directory
        sha=subprocess.check_output(["git","-C",str(source),"rev-parse","HEAD"],text=True).strip()
        assert sha==expected,(name,"SHA identity mismatch",sha,expected)
        assert tomllib.loads((source/"Cargo.toml").read_text())["package"]["name"]==name
        print("SOURCE_SHA",name,sha,flush=True)
    for path in manifests:
        original=path.read_text(encoding="utf-8")
        fields=tomllib.loads(original)
        revised=original
        for dependency,data in fields.get("dependencies",{}).items():
            if dependency not in PINS:
                continue
            directory,rev=PINS[dependency]
            url=f"https://github.com/Perfect-Foundations/{dependency}.git"
            assert data.get("git")==url and data.get("rev")==rev,(path,dependency,data)
            needle=f'git = "{url}", rev = "{rev}"'
            assert revised.count(needle)==1,(path,dependency)
            revised=revised.replace(needle,f'path = "{(root/directory).as_posix()}"')
            print("EDGE_PIN_VERIFIED",fields["package"]["name"],dependency,rev)
        back=path.with_name("Cargo.toml.original-pinned-git")
        assert not back.exists(),"Refusing repeated patch operation"
        back.write_text(original,encoding="utf-8")
        path.write_text(revised,encoding="utf-8")
        print("PINNED_MANIFEST_SHA256",fields["package"]["name"],hashlib.sha256(original.encode()).hexdigest())
    print("LOCAL_ONLY_SOURCE_GRAPH_MATERIALIZED")
if __name__=="__main__":main()
