#!/usr/bin/env python3
"""Clean, source-exact reproduction for experimental fused Polynomial/Arithmetic.

Requires Python 3.9+, git, Rust/Cargo 1.99.0, and private GitHub read access.
No paid runners, global Cargo configuration changes, or protected repo edits.
"""
import argparse
import json
import pathlib
import subprocess
import sys

ARITH_URL = "https://github.com/Perfect-Foundations/perfect-arithmetic.git"
POLY_URL = "https://github.com/Perfect-Foundations/perfect-polynomial.git"
ARITH_SHA = "7dde06a66d0a6ed024859e154ec9ee432060ef7c"
POLY_SHA = "d728fa54e7e826c40346e7d60821a762da6ddd57"
PROTECTED_ARITH = "1a54d3c7cbbae4e73325cc70fd2777a2427b1504"
RATIONAL_SHA = "35a8e9cc629ee578fe7b624e2134929ce7eeff8a"
NUMERIC_SHA = "19b6747cd852a47a694a020b97ba70b6b3ef259b"


def run(args, cwd=None, capture=False):
    print("+", " ".join(str(v) for v in args), flush=True)
    return subprocess.run(args, cwd=cwd, check=True, text=True,
                          stdout=subprocess.PIPE if capture else None).stdout


def clone(url, revision, folder):
    if folder.exists():
        raise RuntimeError("Refusing to overwrite existing directory: " + str(folder))
    run(["git", "clone", "--no-checkout", url, str(folder)])
    run(["git", "checkout", "--detach", revision], cwd=folder)
    actual = run(["git", "rev-parse", "HEAD"], cwd=folder, capture=True).strip()
    if actual != revision:
        raise RuntimeError(f"Revision mismatch: {actual} != {revision}")
    if run(["git", "status", "--porcelain"], cwd=folder, capture=True).strip():
        raise RuntimeError("Source tree is dirty: " + str(folder))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--workdir", required=True, type=pathlib.Path,
                        help="Existing, empty scratch directory, kept for inspection")
    parser.add_argument("--skip-tests", action="store_true",
                        help="Clone and validate identity only (not verification PASS)")
    parser.add_argument("--arithmetic-sha", default=ARITH_SHA,
                        help="Exact review SHA; required for post-baseline changes")
    parser.add_argument("--polynomial-sha", default=POLY_SHA,
                        help="Exact consumer SHA; required for post-baseline changes")
    args = parser.parse_args()
    for value in (args.arithmetic_sha, args.polynomial_sha):
        if len(value) != 40 or any(c not in "0123456789abcdef" for c in value):
            parser.error("Both source revisions must be complete lowercase 40-character SHAs")
    dest = args.workdir.resolve()
    if not dest.is_dir() or any(dest.iterdir()):
        parser.error("--workdir must be an EXISTING EMPTY directory")
    arith = dest / "_arith_fused_integer_20261010"
    poly = dest / "_poly_fused_integer_20261010"
    clone(ARITH_URL, args.arithmetic_sha, arith)
    clone(POLY_URL, args.polynomial_sha, poly)
    cargo_toml = (poly / "Cargo.toml").read_text(encoding="utf-8")
    if f'rev = "{PROTECTED_ARITH}"' not in cargo_toml:
        raise RuntimeError("Protected revision unexpectedly changed")
    if "path = '../_arith_fused_integer_20261010'" not in cargo_toml:
        raise RuntimeError("Expected test-only path patch is absent")
    cargo = "cargo"
    run([cargo, "+1.99.0", "tree", "--locked", "-i", "perfect-arithmetic"], cwd=poly)
    metadata = json.loads(run([cargo, "+1.99.0", "metadata", "--locked",
                               "--format-version", "1"], cwd=poly, capture=True))
    arithmetic = [p for p in metadata["packages"] if p["name"] == "perfect-arithmetic"]
    if len(arithmetic) != 1:
        raise RuntimeError(f"Expected one Arithmetic package, got {len(arithmetic)}")
    package = arithmetic[0]
    if pathlib.Path(package["manifest_path"]).resolve() != (arith / "Cargo.toml").resolve():
        raise RuntimeError("Arithmetic resolved to the wrong source")
    if package["source"] is not None:
        raise RuntimeError("Expected one path-patched Arithmetic package")
    by_name = {p["name"]: p for p in metadata["packages"]}
    for name, revision in [("perfect-numeric", NUMERIC_SHA),
                           ("perfect-rational", RATIONAL_SHA)]:
        pkg = by_name.get(name)
        if not pkg or revision not in str(pkg["source"]):
            raise RuntimeError(f"{name} source revision mismatch")
    graph = metadata["resolve"]
    arithmetic_id = package["id"]
    direct = next(n for n in graph["nodes"] if n["id"] ==
                  next(p["id"] for p in metadata["packages"] if p["name"] == "perfect-polynomial"))
    rational = next(n for n in graph["nodes"] if n["id"] == by_name["perfect-rational"]["id"])
    if arithmetic_id not in direct["dependencies"] or arithmetic_id not in rational["dependencies"]:
        raise RuntimeError("Arithmetic is not shared by direct and transitive consumers")
    print("VERIFIED exact SHA and single direct/transitive Arithmetic identity", flush=True)
    if not args.skip_tests:
        run([cargo, "+1.99.0", "test", "--all-features", "--locked"], cwd=poly)
        run([cargo, "+1.99.0", "test", "--no-default-features", "--locked"], cwd=poly)
        run([cargo, "+1.99.0", "clippy", "--all-targets", "--all-features",
             "--locked", "--", "-D", "warnings"], cwd=poly)
        print("PASS clean-checkout feature tests and warning-denied Clippy", flush=True)
    print(json.dumps({"arithmetic": args.arithmetic_sha, "polynomial": args.polynomial_sha,
                      "arithmetic_package_id": arithmetic_id,
                      "tests_executed": not args.skip_tests}, indent=2))


if __name__ == "__main__":
    main()
