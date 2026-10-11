#!/usr/bin/env python3
"""Fail-closed immutable four-crate source and independent-oracle verification."""
import hashlib
import json
import subprocess
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parent
OLD = ROOT.parent / "fused-git-graph-20261010"
SOURCES = {
    "perfect-arithmetic": "65d8f55641739d5d6a11c17fe7d68c8898d984d7",
    "perfect-numeric": "19b6747cd852a47a694a020b97ba70b6b3ef259b",
    "perfect-rational": "812d66757ce145a7a70fc33818c7b3059ecd3401",
    "perfect-polynomial": "f46660214f426833b69842f5e8d714b703adfcc2",
}

def canonical(path: Path) -> bytes:
    return path.read_bytes().replace(b"\r\n", b"\n")

def checksum(path: Path) -> str:
    return hashlib.sha256(canonical(path)).hexdigest()

def verify() -> None:
    evidence = json.loads((ROOT / "source-exact.json").read_text(encoding="utf-8"))
    assert evidence["schema"] == "perfect-foundations.qual-poly-rational-20261011.v1"
    assert evidence["packages"] == SOURCES
    assert evidence["cargo_lock_sha256"] == checksum(ROOT / "Cargo.lock")
    manifest = tomllib.loads(canonical(ROOT / "Cargo.toml").decode("utf-8"))
    assert manifest["package"]["publish"] is False
    assert manifest["features"]["default"] == ["std"]
    assert not any(k in manifest for k in ("patch", "replace", "workspace"))
    deps = manifest["dependencies"]
    assert set(deps) == {"perfect-polynomial", "perfect-arithmetic", "perfect-rational"}
    for name, data in deps.items():
        assert data["rev"] == SOURCES[name], (name, data)
        assert data["version"] == "=0.1.0"
        assert data["default-features"] is False
        assert data["git"] == f"https://github.com/Perfect-Foundations/{name}.git"

    lock = tomllib.loads(canonical(ROOT / "Cargo.lock").decode("utf-8"))
    entries = lock["package"]
    for name, sha in SOURCES.items():
        matches = [p for p in entries if p["name"] == name]
        assert len(matches) == 1, (name, len(matches))
        assert matches[0]["version"] == "0.1.0"
        expected = f"git+https://github.com/Perfect-Foundations/{name}.git?rev={sha}#{sha}"
        assert matches[0]["source"] == expected, (name, matches[0]["source"])
    for name, expected_hash in evidence["imported_test_sha256"].items():
        path = OLD / "tests" / name
        assert path.is_file() and checksum(path) == expected_hash, path
    assert len(evidence["imported_test_sha256"]) == 10
    assert checksum(ROOT / "tests" / "new_graph_cross_operations.rs") == evidence["cross_operation_test_sha256"]
    assert checksum(ROOT / "benchmarks" / "constant_gcd_20261011.rs") == evidence["constant_gcd_benchmark_sha256"]
    assert checksum(ROOT / "benchmarks" / "prs_degree48_20261011.rs") == evidence["prs_degree48_benchmark_sha256"]
    assert checksum(ROOT / "benchmarks" / "prs_degree64_20261011.rs") == evidence["prs_degree64_benchmark_sha256"]
    assert checksum(ROOT / "benchmarks" / "prs_degree64_measurements_20261011.tsv") == evidence["prs_degree64_measurements_sha256"]
    assert checksum(ROOT / "benchmarks" / "prs_degree96_20261011.rs") == evidence["prs_degree96_benchmark_sha256"]
    assert checksum(ROOT / "benchmarks" / "prs_degree96_measurements_20261011.tsv") == evidence["prs_degree96_measurements_sha256"]
    assert checksum(ROOT / "benchmarks" / "sign_equivalent_20261011.rs") == evidence["sign_equivalent_benchmark_sha256"]
    assert checksum(ROOT / "benchmarks" / "sign_equivalent_measurements_20261011.tsv") == evidence["sign_equivalent_measurements_sha256"]
    assert checksum(ROOT / "benchmarks" / "monic_general_20261011.rs") == evidence["monic_general_benchmark_sha256"]
    assert checksum(ROOT / "benchmarks" / "monic_general_measurements_20261011.tsv") == evidence["monic_general_measurements_sha256"]
    assert checksum(OLD / "references" / "m2_degree2_zq_gcd_sympy_v1.tsv") == evidence["sympy_oracle_sha256"]
    print("EXACT_GRAPH_AND_INDEPENDENT_TEST_FIXTURES_PASS", SOURCES["perfect-polynomial"], SOURCES["perfect-rational"])

if __name__ == "__main__":
    verify()
