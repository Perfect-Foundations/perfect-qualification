#!/usr/bin/env python3
"""Replay the source-exact, separately pinned experimental integration fixture."""
import argparse
import json
import os
import shutil
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parent


def checked(args, **kwargs):
    print("+", " ".join(map(str, args)), flush=True)
    return subprocess.run(args, check=True, **kwargs)


def main():
    cli = argparse.ArgumentParser(description=__doc__)
    cli.add_argument("--workdir", required=True,
                     help="Existing empty directory for independent source clones")
    cli.add_argument("--skip-tests", action="store_true",
                     help="Only inspect identities; NOT qualification verification")
    opt = cli.parse_args()
    info = json.loads((ROOT / "source-exact.json").read_text(encoding="utf-8"))
    assert info["rust"] == "1.99.0"
    arithmetic = info["repos"]["perfect-arithmetic"]["sha"]
    polynomial = info["repos"]["perfect-polynomial"]["sha"]
    replay = ROOT / "replay_fused_integration.py"
    current_blob = checked(["git", "hash-object", str(replay)], capture_output=True, text=True).stdout.strip()
    expected_blob = info["repos"]["perfect-polynomial"]["replay_script_git_blob"]
    if current_blob != expected_blob:
        raise RuntimeError("Vendored replay differs from pinned Polynomial implementation")
    args = [sys.executable, str(replay), "--workdir", opt.workdir,
            "--arithmetic-sha", arithmetic, "--polynomial-sha", polynomial]
    if opt.skip_tests:
        args.append("--skip-tests")
    checked(args)
    poly = Path(opt.workdir).resolve() / "_poly_fused_integer_20261010"
    blob = checked(["git", "-C", str(poly), "rev-parse", "HEAD:Cargo.lock"],
                   capture_output=True, text=True).stdout.strip()
    if blob != info["repos"]["perfect-polynomial"]["cargo_lock_git_blob"]:
        raise RuntimeError("Frozen Cargo.lock does not match independent qualification manifest")
    # The independent Qualification consumer lives outside both source crates.
    # Copy its retained lockfile into the same exact adjacent checkout graph.
    consumer = Path(opt.workdir).resolve() / "consumer"
    shutil.copytree(ROOT / "consumer", consumer)
    lock_before = (consumer / "Cargo.lock").read_bytes()
    metadata = json.loads(checked(
        ["cargo", "+1.99.0", "metadata", "--locked", "--format-version", "1"],
        cwd=consumer, capture_output=True, text=True).stdout)
    arith = [p for p in metadata["packages"] if p["name"] == "perfect-arithmetic"]
    if len(arith) != 1 or arith[0]["source"] is not None:
        raise RuntimeError("Independent consumer has duplicate or non-path Arithmetic identities")
    expected = Path(opt.workdir).resolve() / "_arith_fused_integer_20261010" / "Cargo.toml"
    if Path(arith[0]["manifest_path"]).resolve() != expected.resolve():
        raise RuntimeError("Independent consumer resolved unexpected Arithmetic source")
    for name in ("qualify-fused-integer-polynomial", "perfect-polynomial", "perfect-rational"):
        package_id = next(p["id"] for p in metadata["packages"] if p["name"] == name)
        node = next(n for n in metadata["resolve"]["nodes"] if n["id"] == package_id)
        if arith[0]["id"] not in node["dependencies"]:
            raise RuntimeError(f"{name} does not share single Arithmetic identity")
    checked(["cargo", "+1.99.0", "tree", "--locked", "-i", "perfect-arithmetic"],
            cwd=consumer)
    if not opt.skip_tests:
        for flags in (["--all-features"], ["--no-default-features"]):
            checked(["cargo", "+1.99.0", "test", "--locked", *flags], cwd=consumer)
        checked(["cargo", "+1.99.0", "fmt", "--all", "--", "--check"], cwd=consumer)
        checked(["cargo", "+1.99.0", "clippy", "--all-targets", "--all-features",
                 "--locked", "--", "-D", "warnings"], cwd=consumer)
        env = os.environ.copy()
        env["RUSTDOCFLAGS"] = "-D warnings"
        checked(["cargo", "+1.99.0", "doc", "--all-features", "--no-deps", "--locked"],
                cwd=consumer, env=env)
    if (consumer / "Cargo.lock").read_bytes() != lock_before:
        raise RuntimeError("Independent consumer altered its retained Cargo.lock")
    print("PASS independent pinned source, single Arithmetic identity and retained locks",
          flush=True)
    if opt.skip_tests:
        print("WARNING: identity-only run; Qualification tests NOT executed", flush=True)


if __name__ == "__main__":
    main()
