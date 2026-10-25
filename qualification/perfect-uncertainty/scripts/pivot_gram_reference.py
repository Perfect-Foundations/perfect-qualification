#!/usr/bin/env python3
"""Exact dyadic sign-Gram PSD proof and binary64-input variance reference.

Each row of integer sign matrix has rank=16 entries in {-1,1}. The
correlation R=S*S^T/16 is exact, PSD, with all values representable in
binary64; exact Fraction computes variance of selected binary64 g.
"""
import hashlib
import random
import struct
import sys
from fractions import Fraction as F
from pathlib import Path
DIR=Path(__file__).resolve().parent.parent/"vectors"
PATH=DIR/"pivot_gram_v1.tsv"
RNG=random.Random(0x261011A5)
REPRO=[24810,32994,42583,34078,14413,18809,11555,51663,56833,
       28018,424,29403,47595,16982,55178,23491,35799,33643]
def bits(v):return format(struct.unpack(">Q",struct.pack(">d",v))[0],"016x")
def build():
    rows=["# n mask0..mask_n-1 g0..g_n-1 in binary64 hex variance in binary64 hex"]
    sizes={18:0,24:0,32:0}
    for n in (18,24,32):
        for case in range(30):
            masks=REPRO.copy() if n==18 and case==0 else [RNG.getrandbits(16) for i in range(n)]
            corr=[[F(16-2*((masks[i]^masks[j]).bit_count()),16)
                   for j in range(n)]for i in range(n)]
            if case%4==0:
                g=[1e16 if i%2==0 else -1e16 for i in range(n)]
                g[-1]=1.0
            elif case%4==1:g=[1e12 if i%3==0 else (-1e12 if i%3==1 else 1.0) for i in range(n)]
            elif case%4==2:g=[1.0 if i%2==0 else -1.0 for i in range(n)]
            else:g=[(1e64 if i%4==0 else (-1e64 if i%4==1 else 1.0)) for i in range(n)]
            gs=[F.from_float(x) for x in g]
            v=sum((corr[i][j]*gs[i]*gs[j] for i in range(n) for j in range(n)),F())
            if v<=0:continue
            expected=float(v)
            if not (0<expected<float('inf')):continue
            rows.append(" ".join([str(n),*(str(v) for v in masks),*(bits(x) for x in g),bits(expected)]))
            sizes[n]+=1
    return ("\n".join(rows)+"\n").encode("ascii"),sizes
def main():
    raw,counts=build()
    if "--generate" in sys.argv:
        if PATH.exists():raise SystemExit("REFUSE_OVERWRITE "+str(PATH))
        PATH.write_bytes(raw)
    elif PATH.read_bytes()!=raw:raise SystemExit("PIVOT_GRAM_REFERENCE_CHANGED")
    print("INDEPENDENT_PIVOT_GRAM_PASS",sum(counts.values()),counts,hashlib.sha256(raw).hexdigest())
if __name__=="__main__":main()
