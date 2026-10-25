#!/usr/bin/env python3
"""Build existing exact V1/V2 programs for Linux AArch64 and compare raw QEMU bytes.

No historical stream is modified. No approval of physical ARM64 is implied.
Build output is forced into the caller-specified reusable target directory.
"""
import argparse
import hashlib
import json
import os
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[1]
EVIDENCE = ROOT / "determinism-evidence" / "current-59fa967"
EXPECTED_SHA = "59fa967bdf2c20dbde7a93acb22979c8625441af"
TARGET = "aarch64-unknown-linux-gnu"
EXAMPLES = {"v1": "canonical_workload", "v2": "canonical_boundary_v2"}

def command(argv, **kwargs):
    print("EXEC", " ".join(map(str, argv)), flush=True)
    return subprocess.run(argv, check=True, **kwargs)

def run():
    p = argparse.ArgumentParser()
    p.add_argument("--target-dir", type=Path, required=True)
    p.add_argument("--qemu", default="qemu-aarch64")
    p.add_argument("--sysroot", default="/usr/aarch64-linux-gnu")
    p.add_argument("--linker", default="aarch64-linux-gnu-gcc")
    options = p.parse_args()
    manifest = json.loads((EVIDENCE / "manifest.json").read_text())
    graph = json.loads((ROOT / "source-exact.json").read_text())
    assert manifest["source_polynomial_sha"] == EXPECTED_SHA
    assert graph["packages"]["perfect-polynomial"] == EXPECTED_SHA
    assert command(["rustc", "--version"], capture_output=True).stdout.startswith(b"rustc 1.99.0 ")
    dest = ROOT / "determinism-evidence" / "arm64-qemu-59fa967"
    env = os.environ.copy()
    env.update({
        "CARGO_TARGET_DIR": str(options.target_dir),
        "CARGO_NET_OFFLINE": "true",
        "CARGO_BUILD_JOBS": "2",
        "CARGO_PROFILE_DEV_DEBUG": "0",
        "CARGO_INCREMENTAL": "0",
        "CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER": options.linker,
        "QEMU_LD_PREFIX": options.sysroot,
    })
    results = []
    for mode in ("no-default", "all"):
        feature = "--no-default-features" if mode == "no-default" else "--all-features"
        command(["cargo", "build", "--manifest-path", str(ROOT / "Cargo.toml"),
                 "--locked", "--offline", "--target", TARGET, "--examples", feature],
                cwd=ROOT, env=env)
        for version, executable in EXAMPLES.items():
            path = options.target_dir / TARGET / "debug" / "examples" / executable
            if not path.is_file():
                raise RuntimeError(f"Missing cross-compiled executable: {path}")
            # The existing canonical examples write bytes to an explicit path,
            # rather than stdout; keep the temporary output on reusable WSL disk.
            from tempfile import TemporaryDirectory
            with TemporaryDirectory(prefix="pf-qemu-stream-", dir=options.target_dir) as work:
                output = Path(work) / f"{version}-{mode}.raw"
                try:
                    command([options.qemu, "-L", options.sysroot, str(path), str(output)],
                            env=env, capture_output=True)
                except subprocess.CalledProcessError as exc:
                    raise RuntimeError(f"QEMU program failed: {exc.stderr!r}") from exc
                actual = output.read_bytes()
            c = next(c for c in manifest["corpora"] if c["version"] == version)
            refpaths = [EVIDENCE / f"current-{version}-{host}-{fm}.txt"
                        for host in ("win", "linux") for fm in ("all", "no-default")]
            expected = refpaths[0].read_bytes()
            for reference in refpaths[1:]:
                if reference.read_bytes() != expected:
                    raise AssertionError(f"prior cross-host manifest mismatch: {reference}")
            if actual != expected:
                exp_rows, got_rows = expected.splitlines(), actual.splitlines()
                first = next((i for i in range(max(len(exp_rows),len(got_rows)))
                              if (exp_rows[i] if i<len(exp_rows) else None) !=
                                 (got_rows[i] if i<len(got_rows) else None)), None)
                raise AssertionError(f"{mode}/{version} first differing line {first}; "
                                     f"expected={exp_rows[first] if first is not None and first<len(exp_rows) else None!r} "
                                     f"actual={got_rows[first] if first is not None and first<len(got_rows) else None!r}")
            size = len(actual)
            sha = hashlib.sha256(actual).hexdigest()
            records = len(actual.splitlines()) - 1
            assert (size, sha, records) == (c["bytes"], c["sha256"], c["records"])
            dest.mkdir(parents=True, exist_ok=True)
            out = dest / f"arm64-{version}-{mode}.txt"
            out.write_bytes(actual)
            results.append((mode, version, records, size, sha, out))
            print("ARM64_QEMU_IDENTICAL", mode, version, records, size, sha,
                  "EXIT=0", flush=True)
    print("ARM64_QEMU_V1_V2_PARITY_PASS", len(results), "executions", flush=True)

if __name__ == "__main__":
    run()
