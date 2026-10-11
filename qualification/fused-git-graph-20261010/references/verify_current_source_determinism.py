#!/usr/bin/env python3
"""Verify *current* immutable-source bytes for both established V1/V2 corpora."""
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
EVIDENCE = ROOT / "determinism-evidence" / "current-59fa967"
manifest = json.loads((EVIDENCE / "manifest.json").read_text(encoding="utf-8"))
graph = json.loads((ROOT / "source-exact.json").read_text(encoding="utf-8"))
sha = manifest["source_polynomial_sha"]
assert manifest["schema"] == "perfect-foundations.current-source-determinism.v1"
assert graph["packages"]["perfect-polynomial"] == sha
assert sha == "59fa967bdf2c20dbde7a93acb22979c8625441af"
for corpus in manifest["corpora"]:
    version = corpus["version"]
    name = ("V1" if version == "v1" else "V2")
    paths = [
        EVIDENCE / f"current-{version}-{host}-{mode}.txt"
        for host in ("win", "linux")
        for mode in ("all", "no-default")
    ]
    data = [p.read_bytes() for p in paths]
    baseline = data[0]
    assert baseline.startswith(f"PF-CANONICAL-POLYNOMIAL-{name}\n".encode("ascii"))
    assert baseline.endswith(b"\n") and b"\r" not in baseline
    assert all(item == baseline for item in data), f"{version}: bytes differ"
    rows = baseline.splitlines()[1:]
    labels = [item.split(b"|", 1)[0] for item in rows]
    assert len(labels) == corpus["records"] and len(labels) == len(set(labels))
    assert len(baseline) == corpus["bytes"]
    digest = hashlib.sha256(baseline).hexdigest()
    assert digest == corpus["sha256"]
    print("CURRENT_SOURCE_PARITY", sha, version, corpus["records"],
          len(baseline), digest, "4 hosts/features byte-identical")
print("CURRENT_SOURCE_DETERMINISM_PASS; independent SymPy oracles are separate")
