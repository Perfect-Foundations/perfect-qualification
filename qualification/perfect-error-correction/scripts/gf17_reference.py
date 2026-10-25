#!/usr/bin/env python3
"""Independent exact prime-field GF(17) evaluation for n=16 k=12."""
from pathlib import Path
from hashlib import sha256
from random import Random
import sys
ROOT=Path(__file__).resolve().parent.parent
DEST=ROOT/"vectors"/"rs_gf17_n16_v1.tsv"
SEED=26101106
def render():
    rng=Random(SEED)
    rows=[
      [0]*12,
      [0]*11+[1],
      [0,1,0,2,4,0,6,8,0,10,12,16],
      [16,0,0,0,0,0,0,0,0,0,0,16]]
    for _ in range(4):
      rows.append([rng.randrange(17) for i in range(12)])
    lines=["# 12 GF17 ascending coefficients then 16 exact polynomial evaluations at x=0..15"]
    for coeff in rows:
      values=[]
      for x in range(16):
        a=0
        for c in reversed(coeff):
          a=(a*x+c)%17
        values.append(a)
      lines.append(" ".join(str(v) for v in coeff+values))
    return ("\n".join(lines)+"\n").encode("ascii"),len(rows)
def main():
    raw,count=render()
    if "--generate" in sys.argv:
      if DEST.exists():raise SystemExit("REFUSE_EXISTING_VECTOR_OVERWRITE")
      DEST.write_bytes(raw)
    elif DEST.read_bytes()!=raw:raise SystemExit("GF17_REFERENCE_CHANGED")
    print("INDEPENDENT_GF17_N16_ORACLE_PASS",count,sha256(raw).hexdigest())
if __name__=="__main__":main()
