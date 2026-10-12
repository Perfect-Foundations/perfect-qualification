#!/usr/bin/env python3
"""Independent exact dyadic Gram-correlation and binary64-input variance oracle.

R = (1-epsilon)*sign_gram + epsilon*I, sign_gram = S*S^T/16.
Integer sign Gram is exactly positive semidefinite. With binary epsilon=2^-40,
all correlation entries are exact dyadic binary64 values. Source verifier
recomputes byte-for-byte and the generator refuses overwriting existing data.
"""
from fractions import Fraction as F
from hashlib import sha256
from pathlib import Path
import math
import random
import struct
import sys

HERE=Path(__file__).resolve().parent.parent
DEST=HERE/"vectors"/"gram_covariance_v1.tsv"
RNG_SEED=0x2610119A35
def bits(x):
    return f"{struct.unpack('>Q',struct.pack('>d',x))[0]:016x}"
def exact_variance(mat, vec):
    g=[F.from_float(v) for v in vec]
    result=sum((g[i]*g[i] for i in range(len(vec))), F())
    for i in range(len(vec)):
        for j in range(i+1,len(vec)):
            result+=2*g[i]*g[j]*F.from_float(mat[i][j])
    return result
def build():
    rng=random.Random(RNG_SEED)
    rows=["# n variant matrix_rowmajor_binary64_hex contributions_binary64_hex variance_binary64_hex"]
    counts={}
    for n in (3,4,6,8,12,16):
        for mode in ("rank1","duplicate","random","near"):
            for trial in range(4):
                signs=[[1 if rng.getrandbits(1) else -1 for _ in range(16)] for i in range(n)]
                if mode=="rank1":
                    signs=[signs[0] for _ in range(n)]
                elif mode=="duplicate":
                    signs=[signs[i//2*2] for i in range(n)]
                eps=2.0**-40 if mode=="near" else 0.0
                R=[[0.0 for _ in range(n)] for _ in range(n)]
                for i in range(n):
                    for j in range(n):
                        if i==j:R[i][j]=1.0
                        else:
                            gram=sum(a*b for a,b in zip(signs[i],signs[j]))/16.0
                            R[i][j]=(1.0-eps)*gram
                for pattern in range(5):
                    for scale in (1.0,1e12,1e16,1e50):
                        if pattern==0:
                            g=[scale if i%2==0 else -scale for i in range(n)]
                        elif pattern==1:
                            g=[scale if i%2==0 else -scale for i in range(n)]
                            g[-1]=1.0
                        elif pattern==2:
                            g=[scale if i%3==0 else (-scale if i%3==1 else 1.0) for i in range(n)]
                        elif pattern==3:
                            g=[(1.0 if rng.getrandbits(1) else -1.0)*scale for _ in range(n)]
                        else:
                            g=[(1.0 if i%2==0 else -1.0)*(scale if i<n//2 else 1.0) for i in range(n)]
                        v=exact_variance(R,g)
                        if v<=0:continue
                        expected=float(v)
                        if not math.isfinite(expected) or expected<=0:continue
                        label=f"{mode}/{n}/{trial}/{pattern}/{scale}"
                        rows.append(" ".join([str(n),label,*(bits(x) for r in R for x in r),*(bits(x) for x in g),bits(expected)]))
                        counts[n]=counts.get(n,0)+1
    return ("\n".join(rows)+"\n").encode("ascii"),counts
def main():
    raw,counts=build()
    if "--generate" in sys.argv:
        if DEST.exists():raise SystemExit("REFUSE_OVERWRITE "+str(DEST))
        DEST.write_bytes(raw)
    elif DEST.read_bytes()!=raw:
        raise SystemExit("INDEPENDENT_EXACT_DYADIC_GRAM_REFERENCE_MISMATCH")
    print("EXACT_GRAM_ORACLE_PASS",sum(counts.values()),dict(sorted(counts.items())),
          "SHA256",sha256(raw).hexdigest())
if __name__=="__main__":main()
