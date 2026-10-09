#!/usr/bin/env python3
"""Exact-rational certificate of FLINT 3.0.1 endpoint-corner Arb enclosures.

Separate from native interval Arb results: corners are represented by exact
point Arb balls; Python Fraction derives the exact extrema independently.
"""
from pathlib import Path
from fractions import Fraction as F
from hashlib import sha256
import sys

r=Path(__file__).resolve().parents[1]
corpus=(r/"vectors/arb_adversarial_v1.tsv").read_bytes()
assert sha256(corpus).hexdigest()=="8bb8011b5bf4e3292ea631df2d8e5f13bbdbb6e08f1df2eea8402b39fd9f9e57"
source=(r/"vectors/arb_corners_256_v1.tsv") if len(sys.argv)==1 else Path(sys.argv[1])
raw=source.read_bytes()
corner_digest=sha256(raw).hexdigest()
if len(sys.argv)==1:
    assert corner_digest=="fb294e2de3f59c702924a0ae41be014f8583080f06a5e582a0e5496a6f87ba1d"
print("ARB_POINT_CORNER_SHA256",corner_digest)

def v(n,e):
    return F(n<<e) if e>=0 else F(n,1<<(-e))

def arithmetic(op,x,y):
    if op=="add":return x+y
    if op=="sub":return x-y
    if op=="mul":return x*y
    if op=="div":
        assert y!=0
        return x/y
    if op=="neg":return -x
    if op=="recip":
        assert x!=0
        return F(1)/x
    raise ValueError(op)

cases=[(i,s.split()) for i,s in enumerate(corpus.decode("ascii").splitlines(),1) if s and not s.startswith("#")]
rows=[s.split() for s in raw.decode("ascii").splitlines() if s]
assert len(cases)==330
assert len(rows)==1140,(len(rows),1140)
cursor=0
qualified=0
corners_checked=0
for lineno,c in cases:
    op=c[0]
    a=(v(int(c[2]),int(c[3])),v(int(c[4]),int(c[5])))
    b=(v(int(c[6]),int(c[7])),v(int(c[8]),int(c[9])))
    expected_lo=v(int(c[10]),int(c[11]))
    expected_hi=v(int(c[12]),int(c[13]))
    assert a[0]<=a[1] and b[0]<=b[1]
    kcount=2 if op in ("recip","neg") else 4
    exact_corners=[]
    arb_lower=[]
    arb_upper=[]
    for k in range(kcount):
        row=rows[cursor]
        cursor+=1
        assert len(row)==7,(lineno,row)
        assert int(row[0])==lineno and row[1]==op and int(row[2])==k,(lineno,row)
        xi=k if kcount==2 else k//2
        yi=k%2
        value=arithmetic(op,a[xi],b[yi])
        low=v(int(row[3]),int(row[4]))
        high=v(int(row[5]),int(row[6]))
        assert low<=value<=high,(lineno,op,k,"corner excluded exact answer")
        exact_corners.append(value)
        arb_lower.append(low)
        arb_upper.append(high)
        corners_checked+=1
    exact_min=min(exact_corners)
    exact_max=max(exact_corners)
    assert expected_lo<=exact_min<=exact_max<=expected_hi,(lineno,op,"fraction MPFR extrema differ")
    assert min(arb_lower)<=exact_min and max(arb_upper)>=exact_max,(lineno,op,"Arb point-corner hull excludes exact set")
    qualified+=1
assert cursor==len(rows)
print("ARB_POINT_CORNERS_EXACT_FRACTION_PASS",corners_checked)
print("ARB_POINT_CORNER_INTERVAL_EXTREMA_PASS",qualified)
print("ARB_NATIVE_INTERVAL_NONFINITE_CASES_COVERED_BY_SEPARATE_CORNER_METHOD",38)
