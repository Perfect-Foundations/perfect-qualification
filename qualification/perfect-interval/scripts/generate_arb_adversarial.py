#!/usr/bin/env python3
"""New deterministic adversarial data, distinct from historical 226-case corpus.

All finite true extrema use Python Fraction. Destination p-bit directed
bounds are from GNU MPFR 4.2.1 and independently certified against Fraction.
FLINT/Arb verification is a separate program.
"""
import hashlib
from fractions import Fraction as F
from pathlib import Path
import random
import generate_vectors as previous_reference

SEED=0xA8B20261009
PRECISIONS=(2,3,4,5,7,11,24,53)
EXPONENTS=(-4096,-2048,-257,-63,-1,0,1,63,257,2048,4096)
COEFFICIENTS=(0,1,-1,2,-2,3,-3,7,-7,15,-15,17,-17,2**30-1,-(2**30-1),2**30+1,-(2**30+1),2**50-1,-(2**50-1),2**50+1,-(2**50+1))
OPS=("add","sub","mul","div","recip","neg")

def value(pair):
    n,e=pair
    return F(n<<e) if e>=0 else F(n,1<<(-e))

def interval(n0,e0,n1,e1):
    a=(n0,e0);b=(n1,e1)
    return tuple(sorted((a,b),key=value))

def valid(op,a,b):
    if op=="recip":
        return not value(a[0])<=0<=value(a[1])
    if op=="div":
        return not value(b[0])<=0<=value(b[1])
    return True

def build():
    r=random.Random(SEED)
    cases=[]
    # Explicit cancellation, zero endpoints, extreme heterogeneous exponents,
    # and both exact and inexact low-precision operations.
    for index in range(390):
        op=OPS[index%6];p=PRECISIONS[index%len(PRECISIONS)]
        if index<90:
            e=EXPONENTS[(index//6)%len(EXPONENTS)]
            k=[1,3,2**30-1,2**30+1,2**50-1,2**50+1][(index//9)%6]
            a=interval(-k,e,k,e) if index%4==0 else interval(k,e,k+1,e)
            b=interval(-k,e,-k+1,e) if index%5==0 else interval(k,e,k,e)
        else:
            e0=r.choice(EXPONENTS)
            e1=r.choice(EXPONENTS)
            if index%9==0: e1=e0
            a=interval(r.choice(COEFFICIENTS),e0,r.choice(COEFFICIENTS),e1)
            b=interval(r.choice(COEFFICIENTS),r.choice(EXPONENTS),r.choice(COEFFICIENTS),r.choice(EXPONENTS))
            if index%14==0:
                a=interval(0,e0,0,e0)
            if index%17==0:
                b=interval(1,e1,1,e1)
        if not valid(op,a,b):
            continue
        lo,hi=previous_reference.reference(op,a,b,p)
        row=[op,str(p),*(str(z) for pair in (*a,*b) for z in pair),str(lo[0]),str(lo[1]),str(hi[0]),str(hi[1])]
        assert len(row)==14
        cases.append("\t".join(row))
    assert len(cases)>270,len(cases)
    base=Path(__file__).resolve().parents[1]
    target=base/"vectors/arb_adversarial_v1.tsv"
    raw=("# PF-INTERVAL-ARB-ADVERSARIAL-V1 seed=0xA8B20261009 input dyadics n*2^e "
        "Fraction exact extrema and GNU MPFR 4.2.1 RNDD/RNDU destination\n"
        + "\n".join(cases)+"\n")
    target.write_bytes(raw.encode("ascii"))
    print("ADVERSARIAL_CASES",len(cases))
    print("CORPUS_SHA256",hashlib.sha256(raw.encode("ascii")).hexdigest())
    print("CORPUS",target)
if __name__=="__main__":build()
