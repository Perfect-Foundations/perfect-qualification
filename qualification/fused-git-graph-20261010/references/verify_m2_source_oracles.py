#!/usr/bin/env python3
"""Verify newly pinned Polynomial M2 oracles without copying committed TSVs.

Read exact Cargo Git checkout; run each generator in a temporary directory,
never modifying the pinned source cache.
"""
import hashlib
import importlib.util
import json
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile

ROOT = Path(__file__).resolve().parents[1]
manifest = json.loads((ROOT / "source-exact.json").read_text(encoding="utf-8"))
sha = manifest["packages"]["perfect-polynomial"]
data = json.loads(subprocess.run(
    ["cargo", "+1.99.0", "metadata", "--locked", "--format-version", "1"],
    cwd=ROOT, text=True, check=True, capture_output=True
).stdout)
packages = [p for p in data["packages"] if p["name"] == "perfect-polynomial"]
assert len(packages) == 1
assert f"rev={sha}#{sha}" in packages[0]["source"]
source = Path(packages[0]["manifest_path"]).parent

EXPECTED = [
    ("generate_m2_wide_division_oracle.py",
     "m2-wide-division-reference-v1.tsv", "m2_wide_division_oracle.rs",
     "ad5e6a2e4c03f2f498aab6e7ffcb7071d31495dcc31808268e1a7caf7a14e5f2", 38),
    ("generate_m2_rational_route_oracle.py",
     "m2-rational-route-oracle-v1.tsv", "m2_rational_route_oracle.rs",
     "145fc0432e8e412f6cc542c3215f0b9b87c6cefa5cc3a4ab633419f1d6b9494e", 12),
    ("generate_m2_prs_multiplicity_oracle.py",
     "m2-prs-multiplicity-sympy-v1.tsv", "m2_prs_multiplicity_oracle.rs",
     "6fa8a11ebcd9d503b5d81abef50adb2d8dc4f7f239b2ed880c5279c79b32b51e", 8),
    ("generate_m2_adaptive_composition_oracle.py",
     "m2-adaptive-composition-v1.tsv", "m2_adaptive_composition_oracle.rs",
     "f169c1f700c71f7d5add112814dc73e2ac177f38cf02b434d0128981acb87b40", 10),
    ("generate_m2_rational_euclid_chain.py",
     "m2-rational-euclid-chain-v1.tsv", "m2_rational_euclid_chain.rs",
     "8c5c0b25e39e707b5b200be8b14e0924538a77203ffb6a01415dfab5bf7ae943", 11),
]
EXPECTED.extend([
    ("generate_m2_prs_late_oracle.py","m2-prs-late-reference-v1.tsv",
     "m2_prs_late_reference.rs","d259d8814e7cf1ae2708fbb696880acd0c70430052bf191f3724efa3cec6351e",8),
    ("generate_m2_composed_derivative.py","m2-composed-derivative-v1.tsv",
     "m2_adaptive_derivative_composition.rs","dc68ee441f56183599af1f7427b0d51e4f63bb43c8fbe3ba49c7d6e64841d726",6),
    ("generate_m2_negative_late_fraction.py","m2-negative-late-fraction-v1.tsv",
     "lib:division","2915c83db7d9782c1d089ed7e730f10a5950a2982229488a2b1f0093e76ff859",2),
])
for script_name, fixture_name, rust_name, digest, count in EXPECTED:
    original_script = source / "verification" / script_name
    fixture = source / "verification" / fixture_name
    test = (source / "src" / "division.rs" if rust_name == "lib:division"
            else source / "tests" / rust_name)
    assert original_script.is_file() and fixture.is_file() and test.is_file()
    if rust_name == "lib:division":
        assert "mod m2_negative_late_resource_tests" in test.read_text(encoding="utf-8")
    original = fixture.read_bytes()
    assert hashlib.sha256(original).hexdigest() == digest, fixture_name
    assert len(original.decode("ascii").splitlines()) == count, fixture_name
    if (script_name in ("generate_m2_prs_multiplicity_oracle.py",
                        "generate_m2_prs_late_oracle.py")
            and importlib.util.find_spec("sympy") is None):
        print("PASS", fixture_name, count,
              "rows and digest; REGENERATION SKIPPED (SymPy not installed)")
        continue
    with tempfile.TemporaryDirectory(prefix="pf-source-oracle-") as folder:
        script = Path(folder) / script_name
        shutil.copyfile(original_script, script)
        subprocess.run([sys.executable, str(script)], cwd=folder, check=True,
                       stdout=subprocess.DEVNULL)
        regenerated = Path(folder) / fixture_name
        assert regenerated.is_file(), f"{script_name} failed to produce fixture"
        assert regenerated.read_bytes() == original, f"{script_name}: byte mismatch"
    print("PASS", fixture_name, count, "independent rows, digest, regeneration")
print("PASS all eight source-exact M2 oracle files at", sha)
fraction_contract = source / "verification" / "check_general_lift_fraction.py"
negative_monic_test = source / "tests" / "m2_negative_unit_monic.rs"
assert fraction_contract.is_file() and negative_monic_test.is_file()
subprocess.run([sys.executable, str(fraction_contract)], cwd=ROOT,
               check=True, stdout=subprocess.DEVNULL)
print("PASS pinned independent Fraction mixed-lift/guard contracts and negative-unit monic test")
# These scripts are executable source-exact independent mathematical
# references, not duplicate pinned TSV corpora. Keep unavailable SymPy
# regeneration explicitly distinct from verified fixture hashes.
for script_name, requires_sympy in (
    ("check_quotient_free_gcd_sympy.py", True),
    ("check_integer_reconstruction_sympy.py", True),
    ("check_sparse_fraction_convolution.py", False),
    ("check_infallible_fallback_fraction.py", False),
):
    pinned_script = source / "verification" / script_name
    assert pinned_script.is_file(), script_name
    if requires_sympy and importlib.util.find_spec("sympy") is None:
        print("PASS pinned", script_name,
              "exists; EXECUTION SKIPPED (SymPy not installed)")
        continue
    subprocess.run([sys.executable, str(pinned_script)], cwd=ROOT,
                   check=True, stdout=subprocess.DEVNULL)
    print("PASS pinned independent", script_name)
