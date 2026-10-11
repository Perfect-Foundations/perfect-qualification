#!/usr/bin/env python3
"""Check exact pinned revision identity, no duplicates, and checked-in locks.
No dependency on network or local Cargo caches.
"""
import pathlib, tomllib, hashlib, subprocess
root=pathlib.Path(__file__).resolve().parent
expected={
 "perfect-algebra":{
  "perfect-algebra":"8c8c0794f6098db1545e4ca18a6bfb38e2e822f2",
  "perfect-number-theory":"2bdd2eee53615acb5beac8720c1d2be62120c080",
  "perfect-arithmetic":"9ec660f9abfd7fe93e441e77dbb1158c371603f5",
  "perfect-numeric":"19b6747cd852a47a694a020b97ba70b6b3ef259b"},
 "perfect-units":{
  "perfect-units":"a98a063ef6fa719a697eb290cf6af90514eeafbb",
  "perfect-rational":"97cce0ae8ff7ef206929d273e1d4a5774f6a2a34",
  "perfect-arithmetic":"1a54d3c7cbbae4e73325cc70fd2777a2427b1504",
  "perfect-numeric":"19b6747cd852a47a694a020b97ba70b6b3ef259b"}}
for consumer,wanted in expected.items():
    path=root/"qualification"/consumer
    lock=path/"Cargo.lock"
    assert lock.exists(),lock
    manifest=tomllib.loads((path/"Cargo.toml").read_text(encoding="utf-8"))
    for name,sha in wanted.items():
        explicit=manifest["dependencies"][name]
        assert explicit["rev"]==sha,(consumer,name,"manifest revision mismatch")
        assert explicit["git"]==f"https://github.com/Perfect-Foundations/{name}.git"
    catalog=tomllib.loads(lock.read_text(encoding="utf-8"))["package"]
    selected={}
    for item in catalog:
        name=item["name"]
        if name.startswith("perfect-"):
            assert name not in selected,("duplicated Perfect dependency",consumer,name)
            selected[name]=item["source"]
        if name=="typenum":
            assert consumer=="perfect-units"
            assert item["version"]=="1.19.0"
            assert len(item.get("checksum",""))==64
    assert set(selected)==set(wanted),(consumer,selected.keys(),wanted.keys())
    for name,sha in wanted.items():
        source=selected[name]
        assert source==f"git+https://github.com/Perfect-Foundations/{name}.git?rev={sha}#{sha}",(consumer,name,source)
    for vector in sorted((path/"vectors").glob("*_v1.tsv")):
        data=vector.read_bytes()
        assert b"\r" not in data,("CRLF transformed fixed math corpus",vector)
        assert data.endswith(b"\n")
        print("FROZEN",vector.relative_to(root),"SHA256",hashlib.sha256(data).hexdigest())
    print("SOURCE_GRAPH_LOCK_PASS",consumer,"lock_sha256",hashlib.sha256(lock.read_bytes()).hexdigest())
print("WAVE2_CLEAN_SOURCE_GRAPH_PASS")
