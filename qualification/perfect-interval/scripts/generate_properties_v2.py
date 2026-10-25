#!/usr/bin/env python3
"""Separate seeded exact-rational property corpus, not a baseline replay.

Python Fraction selects exact extrema; MPFR independently rounds toward
negative/positive infinity and those enclosures are Fraction-verified.
"""
from pathlib import Path
from fractions import Fraction as F
from hashlib import sha256
import random
import generate_vectors as reference

SEED = 0xF09120261009
RNG = random.Random(SEED)
OPS = ("add", "sub", "mul", "div", "recip", "neg")
PREC = (2,3,5,7,9,13,16,21,24)
EXPS = (-3072,-2048,-511,-64,-24,-2,-1,0,1,2,24,64,511,2048,3072)
NUMS = (0,-1,1,-2,2,-3,3,-7,7,-15,15,-17,17,-1023,1023,-65535,65535,-1048575,1048575)

def frac(pair):
    n,e=pair
    return F(n<<e) if e>=0 else F(n,1<<(-e))
def ordered(a,b):
    return tuple(sorted((a,b),key=frac))
def make_pair(i,kind):
    e=RNG.choice(EXPS)
    n=RNG.choice(NUMS)
    m=RNG.choice(NUMS)
    if kind==0:
        m=n+RNG.choice((-1,0,1))
    if kind==1:
        return ordered((n,e),(-n,e))
    if kind==2:
        return ordered((0,e),(n,e))
    if kind==3:
        return ordered((n,e),(m,RNG.choice(EXPS)))
    return ordered((n,e),(m,e))
def generate():
    rows=[]
    for i in range(576):
        op=OPS[i%6]
        p=PREC[(i//6)%len(PREC)]
        a=make_pair(i,i%5)
        b=make_pair(i,(i+3)%5)
        if i%11==0: # strong cancellation
            n=RNG.choice((1,3,1023,1048575))
            e=RNG.choice(EXPS)
            a=ordered((n,e),(n+1,e))
            b=ordered((-n,e),(-n+1,e))
        if op=="recip" and frac(a[0])<=0<=frac(a[1]):
            a=ordered((RNG.choice((1,3,7,17)),RNG.choice(EXPS)),(RNG.choice((19,31,63)),RNG.choice(EXPS)))
        if op=="div" and frac(b[0])<=0<=frac(b[1]):
            b=ordered((-31,RNG.choice(EXPS)),(-1,RNG.choice(EXPS)))
        assert frac(a[0])<=frac(a[1]) and frac(b[0])<=frac(b[1])
        if op=="recip":assert not frac(a[0])<=0<=frac(a[1])
        if op=="div":assert not frac(b[0])<=0<=frac(b[1])
        lo,hi=reference.reference(op,a,b,p)
        higher=min(p+11,53)
        high_lo,high_hi=reference.reference(op,a,b,higher)
        fields=(op,str(p),*(str(t) for pair in (*a,*b) for t in pair),
            str(lo[0]),str(lo[1]),str(hi[0]),str(hi[1]),
            str(high_lo[0]),str(high_lo[1]),str(high_hi[0]),str(high_hi[1]))
        assert len(fields)==18
        rows.append("\t".join(fields))
    assert len(rows)==576
    text="# PF-INTERVAL-PROP-V2 seed=0xF09120261009 exact Fraction extrema MPFR 4.2.1 RNDD RNDU\n"+"\n".join(rows)+"\n"
    path=Path(__file__).resolve().parents[1]/"vectors/property_v2.tsv"
    path.write_bytes(text.encode("ascii"))
    print("PF_PROPERTY_CASES",len(rows))
    print("PF_PROPERTY_CORPUS_SHA256",sha256(text.encode("ascii")).hexdigest())
    print("PF_PROPERTY_CORPUS",path)
if __name__=="__main__":generate()
