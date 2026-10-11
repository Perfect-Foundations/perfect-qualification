#!/usr/bin/env python3
"""Independent exact binary64-input quadratic variance fixtures.

All finite input binary64 values are converted to exact Fraction values.
The oracle computes sum(g_i**2)+2*rho*sum(g_i*g_j for i<j), independent
of the production matrix-vector/dot-product and FMA implementation.
"""
from fractions import Fraction
from pathlib import Path
import hashlib
import math
import struct
import sys
ROOT=Path(__file__).resolve().parent.parent
OUTPUT=ROOT/"vectors"/"conditioning_v1.tsv"
def bits(value):
    return f"{struct.unpack('>Q',struct.pack('>d',value))[0]:016x}"
def exact_variance(rho,g):
    coeff=Fraction.from_float(rho)
    vals=[Fraction.from_float(x) for x in g]
    return sum((v*v for v in vals),Fraction(0))+2*coeff*sum(
        (vals[i]*vals[j] for i in range(len(vals)) for j in range(i+1,len(vals))),
        Fraction(0))
def corpus():
    rows=["# n rho_bits g0_bits g1_bits g2_bits g3_bits expected_bits"]
    near_plus=math.nextafter(1.0,0.0)
    near_minus=math.nextafter(-1.0,0.0)
    near40=1.0-2.0**-40
    scales=[1e12,1e16,1e32,1e64,1e100,1e150]
    cases=[]
    for scale in scales:
        for rho in (near_plus,1.0-2.0**-40,0.5,0.0,-0.5,near_minus):
            for g in ((scale,-scale),(scale,scale),(scale,-scale+1.0)):
                cases.append((rho,g))
    for rho,g in cases:
        if rho<0.0 and g[0] != g[1]: # retain but still valid: rho in [-1,1]
            pass
        v=exact_variance(rho,g)
        assert v>=0
        expected=float(v)
        assert math.isfinite(expected) and expected>0.0
        rows.append(" ".join([str(len(g)),bits(rho),*(bits(x) for x in (g+(0.0,)*(4-len(g)))),bits(expected)]))
    for scale in scales:
        for rho in (near_plus,near40,0.5,0.0):
            for g in ((scale,-scale,1.0),(scale,1.0,-scale),(1.0,scale,-scale)):
                v=exact_variance(rho,g)
                expected=float(v)
                assert math.isfinite(expected) and expected>0
                rows.append(" ".join([str(3),bits(rho),*(bits(x) for x in (g+(0.0,))),bits(expected)]))
    for scale in scales:
        for rho in (near_plus,near40,0.5):
            for g in ((scale,-scale,1.0,0.0),
                      (scale,1.0,-scale,0.0),
                      (0.0,scale,1.0,-scale)):
                v=exact_variance(rho,g)
                expected=float(v)
                assert math.isfinite(expected) and expected>0
                rows.append(" ".join([str(4),bits(rho),*(bits(x) for x in g),bits(expected)]))
    return ("\n".join(rows)+"\n").encode("ascii")
def main():
    raw=corpus()
    if "--generate" in sys.argv:
        if OUTPUT.exists(): raise SystemExit("REFUSE_OVERWRITE "+str(OUTPUT))
        OUTPUT.write_bytes(raw)
    elif OUTPUT.read_bytes()!=raw:
        raise SystemExit("CONDITIONING_REFERENCE_CORRUPTION")
    print("INDEPENDENT_BINARY64_FRACTION_CONDITIONING_PASS",
          len(raw.splitlines())-1,hashlib.sha256(raw).hexdigest())
if __name__=="__main__":
    main()
