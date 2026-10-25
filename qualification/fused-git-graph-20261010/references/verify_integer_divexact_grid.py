#!/usr/bin/env python3
"""Source-exact, no-fixture-copy validation of Polynomial's Integer division oracle."""
import collections
import hashlib
import json
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]
manifest = json.loads((ROOT / "source-exact.json").read_text(encoding="utf-8"))
sha = manifest["packages"]["perfect-polynomial"]
meta = json.loads(subprocess.run(
    ["cargo", "+1.99.0", "metadata", "--locked", "--format-version", "1"],
    cwd=ROOT, text=True, capture_output=True, check=True
).stdout)
packages = [p for p in meta["packages"] if p["name"] == "perfect-polynomial"]
assert len(packages) == 1, "expected exactly one Polynomial package"
package = packages[0]
assert f"rev={sha}#{sha}" in package["source"], "Polynomial source SHA mismatch"
source = Path(package["manifest_path"]).parent
generator = source / "verification" / "generate_m2_integer_divexact_grid.py"
fixture = source / "verification" / "m2-integer-divexact-grid-v1.tsv"
rust_test = source / "tests" / "m2_integer_divexact_grid.rs"
assert generator.is_file() and fixture.is_file() and rust_test.is_file()
original = fixture.read_bytes()
digest = hashlib.sha256(original).hexdigest()
expected_digest = "2a293f30bff53386b2371f7bf5e8d67e13d3a2891cf311ae3f1a52d9d7aa401a"
assert digest == expected_digest, f"source corpus digest mismatch: {digest}"
rows = original.decode("ascii").splitlines()
counts = collections.Counter()
seen = set()
for line in rows:
    dividend, divisor, outcome = line.split("\t")
    assert (dividend, divisor) not in seen, "duplicate operand pair"
    seen.add((dividend, divisor))
    counts[outcome.split(":")[1] if outcome.startswith("ERR:") else "ExactQuotient"] += 1
assert len(rows) == len(seen) == 15625, "case count mismatch"
assert dict(counts) == dict(ExactQuotient=868, NonExactDivision=14320,
                            NonIntegralQuotient=312, DivisionByZero=125), counts
try:
    subprocess.run([sys.executable, str(generator)], check=True, cwd=source)
    assert fixture.read_bytes() == original, "generator is not byte deterministic"
finally:
    if fixture.read_bytes() != original:
        fixture.write_bytes(original)
print(f"PASS source-exact Polynomial {sha}: 15625 unique oracle pairs; counts, SHA256, regeneration, and Rust test presence")
