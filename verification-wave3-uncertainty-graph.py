#!/usr/bin/env python3
"""Verify SHA-exact source, one dependency revision and frozen math corpus."""
import hashlib
import pathlib
import tomllib

ROOT=pathlib.Path(__file__).resolve().parent
CONSUMER=ROOT/"qualification"/"perfect-uncertainty"
REV="1ca644f946cb9adea9f817675a464a5800713b25"
VECTORS={
    "gum2_v1.tsv":"c6e23aaeaa5a6d20ac53a47ce32201d8f2544921b429edb07589593b312dd5c4",
    "gum3_v1.tsv":"dae756ec8b8578ff89300cd6246b87a4d3c8702ea190201b19db33aee8e8668a",
    "adversarial_gum_v1.tsv":"23e5c039b6728ade8cd56d42a5e5b3ebfd023821f2fe860e76fddcedc8fed8e4",
}
manifest=tomllib.loads((CONSUMER/"Cargo.toml").read_text(encoding="utf-8"))
deps=manifest["dependencies"]
assert set(deps)=={"perfect-uncertainty"},("UNEXPECTED_DIRECT_DEPENDENCY",deps.keys())
assert deps["perfect-uncertainty"]=={
    "git":"https://github.com/Perfect-Foundations/perfect-uncertainty.git",
    "rev":REV,"version":"=0.1.0","default-features":False,
}
packages=tomllib.loads((CONSUMER/"Cargo.lock").read_text(encoding="utf-8"))["package"]
uni=[x for x in packages if x["name"]=="perfect-uncertainty"]
assert len(uni)==1 and uni[0]["source"]==(
    f"git+https://github.com/Perfect-Foundations/perfect-uncertainty.git?rev={REV}#{REV}"
),("SOURCE_IDENTITY",uni)
libm=[x for x in packages if x["name"]=="libm"]
assert len(libm)==1 and libm[0]["version"]=="0.2.15"
assert len(libm[0]["checksum"])==64
assert len(packages)==3,("EXTRA_TRANSITIVE_DEPENDENCIES",packages)
for filename,expected in VECTORS.items():
    raw=(CONSUMER/"vectors"/filename).read_bytes()
    assert b"\r" not in raw and raw.endswith(b"\n"),("MUTATED_EOL",filename)
    assert hashlib.sha256(raw).hexdigest()==expected,("CORPUS_CHANGED",filename)
    print("INDEPENDENT_FRACTION_SHA_PASS",filename,expected)
print("UNCERTAINTY_EXACT_GIT_GRAPH_PASS",
    hashlib.sha256((CONSUMER/"Cargo.lock").read_bytes()).hexdigest())
