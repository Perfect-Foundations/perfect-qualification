#!/usr/bin/env python3
"""Reproduce exact immutable source graph, lock identity, and oracle fingerprints."""
from pathlib import Path
from hashlib import sha256
import tomllib
root=Path(__file__).resolve().parent
GRAPH={
 "perfect-error-correction":{
  "perfect-error-correction":"c25e906ebbd608391706b25307d0bedfaab80a3d",
  "perfect-algebra":"8c8c0794f6098db1545e4ca18a6bfb38e2e822f2",
  "perfect-number-theory":"2bdd2eee53615acb5beac8720c1d2be62120c080",
  "perfect-arithmetic":"9ec660f9abfd7fe93e441e77dbb1158c371603f5",
  "perfect-numeric":"19b6747cd852a47a694a020b97ba70b6b3ef259b"},
 "perfect-finance":{
  "perfect-finance":"63f704c68e1567991979fff1061266946285d153",
  "perfect-decimal":"f048affde6c5ea037b793580d295254695f7e97a",
  "perfect-rational":"97cce0ae8ff7ef206929d273e1d4a5774f6a2a34",
  "perfect-arithmetic":"1a54d3c7cbbae4e73325cc70fd2777a2427b1504",
  "perfect-numeric":"19b6747cd852a47a694a020b97ba70b6b3ef259b"}
}
VECTORS={
 "perfect-error-correction":{
  "hamming_v1.tsv":"5ddef5b87b2b994ced47a86a2f3c4b88db09f17603b54254866aa05c6c24cdf6",
  "rs_gf5_v1.tsv":"07b62970fc79b24fddc9a2494228c180f0a4600b75fe791db53db807683408e0",
  "rs_gf16_v1.tsv":"b16458ba91078c41d6c00d617dba5399014d49168322cd0303deba802b1e3ddb",
  "rs_gf17_n16_v1.tsv":"cf0b27977ee74c7a9adf7a2dc418791610f275e60d423d885459d440221b643f",
  "rs_bw_gf7_v1.tsv":"ee8a732e3568b14145c99103b336526e82e955953ac2c7a3cfe22fcf8805f378",
  "hamming_packed_v1.tsv":"9cf1d589247b80a75a0ad154a2e765951eeec14f360c9c7a07befd4787dd98e5"},
 "perfect-finance":{
  "day_count_v1.tsv":"0a113ac24472f45ed92707c43157f79c20cd1981d074f6c89c0975b82832ddc0",
  "money_quantize_v1.tsv":"d754eff8cefc686087dcda8429a03d4aab693e4bdf337b7b5e50d8dc27e3b76f",
  "cashflow_cancel_v1.tsv":"d97b4baf8f271d763cc788e62452fb271fd8254319738fc4260c088ed8a35ece",
  "multioperand_v1.tsv":"c59cf1b20dc18370ba0103f7ea372f17ef7121acfc839d3ace42875ac9e6b455",
  "currency_net_v1.tsv":"02c9f1c3973da070118382ae1ab6211e317af85ff21b04497b9ccc95fcee716b",
  "date_range_v1.tsv":"0529ed78ea046396b8f7ca80c541711a810179b909a30be926567496cff9896e"}}
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
        "lock_sha256",sha256(lock.read_bytes().replace(b"\r\n", b"\n")).hexdigest())
print("WAVE3_GRAPH_LOCK_FROZEN_VECTORS_PASS")

# Recompute new GF(7) oracle independently; frozen SHA alone is not enough.
import runpy
bw_path = root/"qualification"/"perfect-error-correction"/"scripts"/"bw_gf7_reference.py"
bw_data, bw_count = runpy.run_path(str(bw_path))["render"]()
bw_frozen = root/"qualification"/"perfect-error-correction"/"vectors"/"rs_bw_gf7_v1.tsv"
assert bw_data == bw_frozen.read_bytes() and bw_count == 16
print("BW_GF7_INDEPENDENT_REGEN_PASS", bw_count, sha256(bw_data).hexdigest())
# Source-independent regeneration of Wave 9 packed-bit and currency totals.
for name, script, output, expected in [
    ("perfect-error-correction","packed_reference.py","hamming_packed_v1.tsv",48),
    ("perfect-finance","currency_net_reference.py","currency_net_v1.tsv",6),
    ("perfect-finance","date_range_reference.py","date_range_v1.tsv",14),
]:
    directory=root/"qualification"/name
    namespace=runpy.run_path(str(directory/"scripts"/script))
    raw,count=namespace["render"]()
    assert count==expected and raw==(directory/"vectors"/output).read_bytes()
    print("WAVE10_INDEPENDENT_REGEN_PASS" if script=="date_range_reference.py" else "WAVE9_INDEPENDENT_REGEN_PASS", name,count,sha256(raw).hexdigest())

# Independent exact two-revision Finance performance harness provenance.
bench=root/"qualification"/"perfect-finance"/"benchmarks"/"exact-aggregation"
meta=tomllib.loads((bench/"Cargo.toml").read_text(encoding="utf-8"))
expected={
  "baseline":"90be47248298b49089d49f954866ed916bc57ed5",
  "corrected":"f716cb02c34dc79d51cef3540d7f0c0cc2ba930f",
}
assert set(meta["dependencies"])==set(expected)
for alias,sha in expected.items():
    entry=meta["dependencies"][alias]
    assert entry["package"]=="perfect-finance" and entry.get("path") is None
    assert entry["rev"]==sha and entry["git"]=="https://github.com/Perfect-Foundations/perfect-finance.git"
bench_lock=(bench/"Cargo.lock").read_bytes()
records=tomllib.loads(bench_lock.decode("utf-8"))["package"]
sources={x["source"] for x in records if x["name"]=="perfect-finance"}
assert sources=={
    f"git+https://github.com/Perfect-Foundations/perfect-finance.git?rev={sha}#{sha}"
    for sha in expected.values()
},sources
print("EXACT_FINANCE_DUAL_REVISION_BENCH_PASS",sha256(bench_lock.replace(b"\r\n", b"\n")).hexdigest())
