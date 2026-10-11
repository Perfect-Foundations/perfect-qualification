#!/usr/bin/env python3
"""Reproduce exact immutable source graph, lock identity, and oracle fingerprints."""
from pathlib import Path
from hashlib import sha256
import tomllib
root=Path(__file__).resolve().parent
GRAPH={
 "perfect-error-correction":{
  "perfect-error-correction":"02594e730c156ed0e5f585ba0e5d348d6240e6fd",
  "perfect-algebra":"8c8c0794f6098db1545e4ca18a6bfb38e2e822f2",
  "perfect-number-theory":"2bdd2eee53615acb5beac8720c1d2be62120c080",
  "perfect-arithmetic":"9ec660f9abfd7fe93e441e77dbb1158c371603f5",
  "perfect-numeric":"19b6747cd852a47a694a020b97ba70b6b3ef259b"},
 "perfect-finance":{
  "perfect-finance":"6f7540a13d29ab97387e2b0305fd3c9c2739d9c6",
  "perfect-decimal":"f048affde6c5ea037b793580d295254695f7e97a",
  "perfect-rational":"97cce0ae8ff7ef206929d273e1d4a5774f6a2a34",
  "perfect-arithmetic":"1a54d3c7cbbae4e73325cc70fd2777a2427b1504",
  "perfect-numeric":"19b6747cd852a47a694a020b97ba70b6b3ef259b"}
}
VECTORS={
 "perfect-error-correction":{
  "hamming_v1.tsv":"5ddef5b87b2b994ced47a86a2f3c4b88db09f17603b54254866aa05c6c24cdf6",
  "rs_gf5_v1.tsv":"07b62970fc79b24fddc9a2494228c180f0a4600b75fe791db53db807683408e0",
  "rs_gf16_v1.tsv":"b16458ba91078c41d6c00d617dba5399014d49168322cd0303deba802b1e3ddb"},
 "perfect-finance":{
  "day_count_v1.tsv":"0a113ac24472f45ed92707c43157f79c20cd1981d074f6c89c0975b82832ddc0",
  "money_quantize_v1.tsv":"d754eff8cefc686087dcda8429a03d4aab693e4bdf337b7b5e50d8dc27e3b76f"}}
for name,wanted in GRAPH.items():
  p=root/"qualification"/name
  lock=p/"Cargo.lock"
  manifest=tomllib.loads((p/"Cargo.toml").read_text(encoding="utf-8"))
  for dep in manifest["dependencies"].values():
    assert dep.get("path") is None,("FORBID_SOURCE_OVERRIDE",name)
  packages=tomllib.loads(lock.read_text(encoding="utf-8"))["package"]
  actual={}
  for package in packages:
    pkg=package["name"]
    if pkg.startswith("perfect-"):
      assert pkg not in actual,("DUPLICATE_PERFECT_REVISION",name,pkg)
      actual[pkg]=package["source"]
    if pkg=="time":
      assert name=="perfect-finance" and package["version"]=="0.3.44"
  assert set(actual)==set(wanted),(name,actual.keys(),wanted.keys())
  for pkg,sha in wanted.items():
    expected=f"git+https://github.com/Perfect-Foundations/{pkg}.git?rev={sha}#{sha}"
    assert actual[pkg]==expected,(name,pkg,actual[pkg])
    if pkg in manifest["dependencies"]:
      assert manifest["dependencies"][pkg]["rev"]==sha
  for vector,digest in VECTORS[name].items():
    data=(p/"vectors"/vector).read_bytes()
    assert b"\r" not in data and data.endswith(b"\n")
    assert sha256(data).hexdigest()==digest,(name,vector)
  print("EXACT_GRAPH_AND_VECTOR_PASS",name,
        "lock_sha256",sha256(lock.read_bytes()).hexdigest())
print("WAVE3_GRAPH_LOCK_FROZEN_VECTORS_PASS")
