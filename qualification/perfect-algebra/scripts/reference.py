#!/usr/bin/env python3
"""Independent exact finite-field reference; Python integers only.
--generate creates a previously nonexistent corpus. Default: verify immutable bytes.
"""
import hashlib
from pathlib import Path
import sys
ROOT=Path(__file__).resolve().parent.parent
FAMILIES={"g22":(2,(1,1,1)),"g23":(2,(1,1,0,1)),
          "g32":(3,(1,0,1))}
def coeffs(x,p,n):
    out=[]
    for _ in range(n):
        out.append(x%p);x//=p
    return out
def encode(v,p):
    return sum(c*p**i for i,c in enumerate(v))
def product(a,b,p,mod):
    n=len(mod)-1
    x=coeffs(a,p,n);y=coeffs(b,p,n)
    out=[0]*(2*n-1)
    for i in range(n):
        for j in range(n):out[i+j]=(out[i+j]+x[i]*y[j])%p
    for i in range(2*n-2,n-1,-1):
        lead=out[i]
        for j in range(n):out[i-n+j]=(out[i-n+j]-lead*mod[j])%p
    return encode(out[:n],p)
def reference():
    p_rows=["# p a b plus minus times"]
    for p in (2,3,5,7,11):
        for a in range(p):
            for b in range(p):
                p_rows.append(f"{p} {a} {b} {(a+b)%p} {(a-b)%p} {(a*b)%p}")
    e_rows=["# id p a b plus times"]
    for name,(p,mod) in FAMILIES.items():
        n=len(mod)-1
        for a in range(p**n):
            for b in range(p**n):
                aa=coeffs(a,p,n);bb=coeffs(b,p,n)
                addition=encode([(x+y)%p for x,y in zip(aa,bb)],p)
                e_rows.append(f"{name} {p} {a} {b} {addition} {product(a,b,p,mod)}")
    return {"prime_v1.tsv":"\n".join(p_rows)+"\n",
            "extension_v1.tsv":"\n".join(e_rows)+"\n"}
def main():
    generated="--generate" in sys.argv
    for name,raw in reference().items():
        path=ROOT/"vectors"/name
        data=raw.encode("ascii")
        if generated:
            if path.exists():raise SystemExit(f"Refuse overwrite {path}")
            path.write_bytes(data)
        elif path.read_bytes()!=data:
            raise SystemExit(f"INDEPENDENT_ORACLE_MISMATCH {name}")
        print("ORACLE",name,"rows",len(raw.splitlines())-1,
              "sha256",hashlib.sha256(data).hexdigest())
    print("INDEPENDENT_ALGEBRA_REFERENCE_PASS")
if __name__=="__main__":main()
