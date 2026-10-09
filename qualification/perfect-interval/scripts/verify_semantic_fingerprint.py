#!/usr/bin/env python3
"""Validate fixed canonical semantic transcript and calculate SHA-256."""
from pathlib import Path
from hashlib import sha256
import re
import sys

assert len(sys.argv)==2
log=Path(sys.argv[1]).read_text(encoding="utf-8-sig")
lines=[line[len("PF-SEM-RECORD "):] for line in log.splitlines()
    if line.startswith("PF-SEM-RECORD ")]
assert len(lines)==65,len(lines)
assert lines[0]=="PF-SEMANTIC-FINGERPRINT-V1"
for i,row in enumerate(lines[1:]):
    assert row.startswith(f"{i:04}|"),(i,row)
    assert row.count("|")==5,(i,row)
raw=("\n".join(lines)+"\n").encode("ascii")
fnv=0xcbf29ce484222325
for byte in raw:
    fnv=((fnv^byte)*0x100000001b3) & 0xffffffffffffffff
observed=re.findall(r"PF-SEM-V1-FNV64=([0-9a-f]{16}) CASES=64 BYTES=([0-9]+)",log)
assert len(observed)==1
assert observed[0]==(f"{fnv:016x}",str(len(raw))),observed
assert re.search(r"test result: ok\. 1 passed; 0 failed;",log),"no passing test summary"
print("PF_SEMANTIC_CASES",64)
print("PF_SEMANTIC_BYTES",len(raw))
print("PF_SEMANTIC_FNV1A64",f"{fnv:016x}")
actual_sha256=sha256(raw).hexdigest()
assert actual_sha256=="b73867901ba3a261a9ef06cafd6bbdaa8e1e10fbd4d66dd262200b83594a7f4b",(
    "REPRESENTATION_DRIFT_NOT_NECESSARILY_UNSOUND",actual_sha256)
print("PF_SEMANTIC_SHA256",actual_sha256)
print("PF_SEMANTIC_TRANSCRIPT_VERIFIED")
