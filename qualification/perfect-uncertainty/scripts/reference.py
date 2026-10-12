#!/usr/bin/env python3
"""Independent exact Rational GUM variance; no f64 or production Rust calls.

Uses diagonal-plus-paired-off-diagonal computation, independent of the
production row-major floating quadratic form and source-vector generation.
"""
from fractions import Fraction as Q
from hashlib import sha256
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parent.parent
def pair(x):
    return str(x.numerator) + " " + str(x.denominator)

def exact_variance(matrix, sensitivities):
    n=len(sensitivities)
    result=sum((sensitivities[i]**2 for i in range(n)), Q(0))
    for row in range(n):
        for col in range(row+1,n):
            result += 2 * sensitivities[row] * matrix[row][col] * sensitivities[col]
    return result

def vectors():
    samples=[Q(0), Q(1), Q(-1), Q(3), Q(4), Q(-7),
             Q(1,2), Q(-2,3), Q(5,7)]
    rhos=[Q(-1),Q(-3,4),Q(-1,3),Q(0),Q(1,3),Q(3,4),Q(1)]
    two=["# rho_num rho_den a_num a_den b_num b_den variance_num variance_den"]
    for rho in rhos:
        matrix=((Q(1),rho),(rho,Q(1)))
        for a,b in zip(samples,reversed(samples)):
            v=exact_variance(matrix,(a,b))
            assert v>=0
            two.append(" ".join((pair(rho),pair(a),pair(b),pair(v))))
    mats=[
        ((Q(1),Q(0),Q(0)),(Q(0),Q(1),Q(0)),(Q(0),Q(0),Q(1))),
        ((Q(1),Q(1),Q(1)),(Q(1),Q(1),Q(1)),(Q(1),Q(1),Q(1))),
        ((Q(1),Q(3,5),Q(0)),(Q(3,5),Q(1),Q(4,5)),(Q(0),Q(4,5),Q(1))),
        ((Q(1),Q(3,5),Q(4,5)),(Q(3,5),Q(1),Q(0)),(Q(4,5),Q(0),Q(1))),
        ((Q(1),Q(1),Q(0)),(Q(1),Q(1),Q(0)),(Q(0),Q(0),Q(1))),
    ]
    cases=[(1,2,3),(-4,2,1),(0,0,0),(1,-1,0),
           (3,-5,8),(11,12,-7),(-1,-1,-1),(9,-3,4)]
    three=["# r12n r12d r13n r13d r23n r23d g1n g1d g2n g2d g3n g3d variancen varianced"]
    for matrix in mats:
        for case in cases:
            g=tuple(Q(x) for x in case)
            v=exact_variance(matrix,g)
            assert v>=0,(matrix,case,v)
            three.append(" ".join([pair(matrix[0][1]),pair(matrix[0][2]),
               pair(matrix[1][2]),*(pair(x) for x in g),pair(v)]))
    return {"gum2_v1.tsv":"\n".join(two)+"\n",
            "gum3_v1.tsv":"\n".join(three)+"\n"}

def main():
    for filename,content in vectors().items():
        raw=content.encode("ascii")
        path=ROOT/"vectors"/filename
        if "--generate" in sys.argv:
            if path.exists():
                raise SystemExit("REFUSE_OVERWRITE "+str(path))
            path.write_bytes(raw)
        elif path.read_bytes()!=raw:
            raise SystemExit("INDEPENDENT_FRACTION_REFERENCE_MISMATCH "+str(path))
        print("INDEPENDENT_GUM_FRACTION_PASS",filename,
              len(content.splitlines())-1,sha256(raw).hexdigest())
if __name__=="__main__":
    main()
