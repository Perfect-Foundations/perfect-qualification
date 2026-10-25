#!/usr/bin/env python3
"""Exact rational verification of the separate FLINT/Arb adversarial corpus.

Arb finite enclosures are verified independently against Python Fraction
mathematical extrema. Arb nonfinite output is NOT counted as qualified.
Destination MPFR reference bounds are checked for all vectors independently.
"""
from pathlib import Path
from fractions import Fraction as F
import sys
import hashlib

root=Path(__file__).resolve().parents[1]
vectors=root/"vectors/arb_adversarial_v1.tsv"
source=vectors.read_bytes()
digest=hashlib.sha256(source).hexdigest()
expected="8bb8011b5bf4e3292ea631df2d8e5f13bbdbb6e08f1df2eea8402b39fd9f9e57"
assert digest==expected,(digest,expected)
if len(sys.argv)>2:
    raise SystemExit("Expected zero or one Arb output path")
is_committed=len(sys.argv)==1
results_path=(root/"vectors/arb_reference_256_v1.tsv") if is_committed else Path(sys.argv[1])
results=results_path.read_bytes()
result_hash=hashlib.sha256(results).hexdigest()
if is_committed:
    assert result_hash=="2961c84f449a49c98d4ed5bfaed5e4d8e501e4eb423f3eedb98bfa57df2d4ee4",result_hash

def v(n,e):
    return F(n<<e) if e>=0 else F(n,1<<(-e))

def true_bounds(op,a,b):
    lo,hi=a;bl,bh=b
    if op=="add":return lo+bl,hi+bh
    if op=="sub":return lo-bh,hi-bl
    if op=="neg":return -hi,-lo
    if op=="recip":
        assert not lo<=0<=hi
        return min(F(1)/lo,F(1)/hi),max(F(1)/lo,F(1)/hi)
    if op in ("mul","div"):
        if op=="div": assert not bl<=0<=bh
        vals=[x*y if op=="mul" else x/y for x in (lo,hi) for y in (bl,bh)]
        return min(vals),max(vals)
    raise ValueError(op)

cases=[(i,line.split()) for i,line in enumerate(source.decode("ascii").splitlines(),1) if line and not line.startswith("#")]
arb=[line.split() for line in results.decode("ascii").splitlines() if line]
assert len(cases)==len(arb)==330,(len(cases),len(arb))
finite=0; unsupported=0; wider_than_mpfr=0
kinds={}
for (index,cells),answer in zip(cases,arb):
    op=cells[0]
    a=(v(int(cells[2]),int(cells[3])),v(int(cells[4]),int(cells[5])))
    b=(v(int(cells[6]),int(cells[7])),v(int(cells[8]),int(cells[9])))
    assert a[0]<=a[1] and b[0]<=b[1]
    real_lo,real_hi=true_bounds(op,a,b)
    p_low=v(int(cells[10]),int(cells[11]))
    p_high=v(int(cells[12]),int(cells[13]))
    assert p_low<=real_lo<=real_hi<=p_high,(index,op,"MPFR exact reference failure")
    assert int(answer[0])==index and answer[1]==op,(index,answer)
    if answer[2]=="UNSUPPORTED_NONFINITE":
        assert len(answer)==3
        unsupported+=1
        kinds[op]=kinds.get(op,0)+1
        continue
    assert len(answer)==6,(index,len(answer))
    arb_lo=v(int(answer[2]),int(answer[3]))
    arb_hi=v(int(answer[4]),int(answer[5]))
    assert arb_lo<=real_lo<=real_hi<=arb_hi,(index,op,"FLINT/Arb EXCLUDED EXACT ANSWER",arb_lo,arb_hi,real_lo,real_hi)
    if arb_lo<p_low or arb_hi>p_high: wider_than_mpfr+=1
    finite+=1
print("ARB_INPUT_CORPUS_SHA256",digest)
print("ARB_RAW_OUTPUT_SHA256",result_hash)
print("EXACT_FRACTION_MPFR_BOUND_PASS",len(cases))
print("FLINT_ARB_FINITE_VERIFIED",finite)
print("FLINT_ARB_UNSUPPORTED_NONFINITE_NOT_COUNTED",unsupported)
print("FLINT_ARB_UNSUPPORTED_BY_OP",sorted(kinds.items()))
print("FLINT_ARB_WIDER_THAN_MPFR_FOR_VALID_INPUTS",wider_than_mpfr)
print("FLINT_ARB_EXACT_FRACTION_CONTAINMENT_PASS",finite)
