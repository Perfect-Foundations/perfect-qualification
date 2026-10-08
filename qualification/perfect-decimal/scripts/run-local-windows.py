#!/usr/bin/env python3
"""Offline Windows M1 qualification with exact local Git revisions and retained logs.

Run from an authorized native Windows host with Git, Python and Rust 1.99.0.
No GitHub-hosted runner minutes, registry substitutes or source-tree writes.
"""
from __future__ import annotations

import hashlib
import os
from pathlib import Path
import platform
import shutil
import subprocess
import sys
import tempfile

if os.name != "nt":
    raise SystemExit("This entry point is for native Windows; use run-local.sh on Linux.")

ROOT = Path(__file__).resolve().parents[3]
SUITE = ROOT / "qualification" / "perfect-decimal"
PINS = {
    "decimal": "85f4408838131007146fff14b1c9c1d760c585a8",
    "numeric": "19b6747cd852a47a694a020b97ba70b6b3ef259b",
    "arithmetic": "1a54d3c7cbbae4e73325cc70fd2777a2427b1504",
    "rational": "97cce0ae8ff7ef206929d273e1d4a5774f6a2a34",
}

evidence_root = Path(os.environ.get("PF_EVIDENCE_ROOT", str(Path.home() / "pf-verify")))
evidence_root.mkdir(parents=True, exist_ok=True)
workspace = Path(tempfile.mkdtemp(prefix="decimal-win-qualification.", dir=evidence_root))
summary = workspace / "summary.txt"
env = {**os.environ, "CARGO_NET_OFFLINE": "true", "CARGO_TERM_COLOR": "never"}


def emit(message: str) -> None:
    print(message, flush=True)
    with summary.open("a", encoding="utf-8") as log:
        log.write(message + "\n")


def exec_capture(name: str, command: list[str], *, extra_env: dict[str, str] | None = None) -> str:
    result = subprocess.run(
        command, cwd=workspace, env={**env, **(extra_env or {})},
        capture_output=True, text=True, encoding="utf-8", errors="replace", check=False,
    )
    (workspace / (name + ".stdout")).write_text(result.stdout, encoding="utf-8")
    (workspace / (name + ".stderr")).write_text(result.stderr, encoding="utf-8")
    emit(f"{'PASS' if result.returncode == 0 else 'FAIL'} {name} EXIT={result.returncode}")
    if result.returncode:
        print(result.stderr[-2500:], result.stdout[-1000:], flush=True)
        raise SystemExit(result.returncode)
    if name.startswith("tests_"):
        for line in result.stdout.splitlines():
            if line.startswith("test result:"):
                emit(line)
    return result.stdout


emit(f"WORKSPACE={workspace}")
emit(f"QUALIFICATION_SHA={subprocess.check_output(['git', '-C', str(ROOT), 'rev-parse', 'HEAD'], text=True).strip()}")
emit(f"HOST={platform.system()} {platform.machine()}")
emit(exec_capture("rustc", ["rustc", "+1.99.0", "--version"]).strip())

for name, commit in PINS.items():
    key = f"PF_{name.upper()}_REPO"
    source = os.environ.get(key)
    if not source:
        raise SystemExit(f"MISSING_SOURCE_PATH={key}")
    original = Path(source).resolve()
    if not (original / ".git").exists():
        raise SystemExit(f"INVALID_GIT_SOURCE={original}")
    destination = workspace / name
    exec_capture(f"clone_{name}", ["git", "clone", "--quiet", "--no-hardlinks", str(original), str(destination)])
    exec_capture(f"checkout_{name}", ["git", "-C", str(destination), "checkout", "--quiet", "--detach", commit])
    actual = subprocess.check_output(["git", "-C", str(destination), "rev-parse", "HEAD"], text=True).strip()
    if actual != commit:
        raise SystemExit(f"WRONG_PIN={name} {actual}")
    emit(f"PIN_{name}={actual}")
    env[f"{name.upper()}_CARGO_PATH"] = str(destination)

# Stage the harness from committed Git objects, never from an edited working tree.
harness_sha = subprocess.check_output(
    ["git", "-C", str(ROOT), "rev-parse", "HEAD"], text=True
).strip()
clean_harness = workspace / "qualification"
exec_capture("clone_qualification", ["git", "clone", "--quiet", "--no-hardlinks", str(ROOT), str(clean_harness)])
exec_capture("checkout_qualification", ["git", "-C", str(clean_harness), "checkout", "--quiet", "--detach", harness_sha])
shutil.copytree(clean_harness / "qualification" / "perfect-decimal", workspace / "consumer")
consumer = workspace / "consumer"
manifest = consumer / "Cargo.toml"
env["QUAL_MANIFEST"] = str(manifest)
env["DECIMAL_SHELL_PATH"] = env["DECIMAL_CARGO_PATH"]

exec_capture("oracle_quantize", [sys.executable, str(consumer / "scripts" / "verify_oracle.py")])
exec_capture("oracle_fingerprint", [sys.executable, str(consumer / "scripts" / "verify_fingerprint.py")])
exec_capture("materializer", [sys.executable, str(consumer / "scripts" / "materialize_private.py")])
cargo = ["cargo", "+1.99.0"]
exec_capture("lock", cargo + ["generate-lockfile", "--offline", "--manifest-path", str(manifest)])
lockfile = consumer / "Cargo.lock"
lock_before = hashlib.sha256(lockfile.read_bytes()).hexdigest()
common = ["--manifest-path", str(manifest)]
exec_capture("fmt", cargo + ["fmt", *common, "--", "--check"])
exec_capture("tests_all", cargo + ["test", *common, "--locked", "--offline", "--all-features"])
exec_capture("tests_no_default", cargo + ["test", *common, "--locked", "--offline", "--no-default-features"])
exec_capture("clippy_all", cargo + ["clippy", *common, "--locked", "--offline", "--all-targets", "--all-features", "--", "-D", "warnings"])
exec_capture("clippy_no_default", cargo + ["clippy", *common, "--locked", "--offline", "--all-targets", "--no-default-features", "--", "-D", "warnings"])
exec_capture("rustdoc", cargo + ["doc", *common, "--locked", "--offline", "--all-features", "--no-deps"], extra_env={"RUSTDOCFLAGS": "-D warnings"})
lock_after = hashlib.sha256(lockfile.read_bytes()).hexdigest()
if lock_before != lock_after:
    raise SystemExit("LOCK_DRIFT")
emit(f"LOCK_SHA256={lock_after}")
emit("FINAL=PASS_LOCAL_ONLY")
