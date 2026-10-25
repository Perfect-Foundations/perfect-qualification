#!/usr/bin/env python3
"""Independent exact Python Fraction/SymPy oracle for canonical_workload.

Python >=3.11, SymPy 1.14.0 (test-time only, never Cargo dependency).
Recompute every record rather than learning values from Rust output.
"""
from fractions import Fraction as F
from pathlib import Path
import hashlib
import sys
import sympy as sp

def canon(p):
    p = list(p)
    while p and p[-1] == 0:
        p.pop()
    return p
def add(a,b):
    return canon([(a[i] if i<len(a) else 0)+(b[i] if i<len(b) else 0) for i in range(max(len(a),len(b)))])
def neg(a):
    return [-x for x in a]
def sub(a,b):
    return add(a,neg(b))
def mul(a,b):
    if not a or not b: return []
    r = [0]*(len(a)+len(b)-1)
    for i,x in enumerate(a):
        for j,y in enumerate(b): r[i+j] += x*y
    return canon(r)
def deriv(a):
    return canon([i*x for i,x in enumerate(a)][1:])
def evaluate(a,x):
    r=0
    for c in reversed(a): r=r*x+c
    return r
def qdiv(a,b):
    a,b=canon(a),canon(b)
    assert b
    q=[F(0)]*max(0,len(a)-len(b)+1)
    while len(a)>=len(b):
        k=len(a)-len(b)
        t=F(a[-1])/b[-1]
        q[k]=t
        for i,c in enumerate(b): a[k+i]-=t*c
        a=canon(a)
    return canon(q),a
def sympy_gcd(a,b,rational=False):
    x=sp.Symbol("x")
    domain=sp.QQ if rational else sp.ZZ
    pa=sp.Poly.from_list(list(reversed(a)),gens=x,domain=domain)
    pb=sp.Poly.from_list(list(reversed(b)),gens=x,domain=domain)
    g=sp.gcd(pa,pb)
    result=list(reversed(g.all_coeffs()))
    if rational: return [F(str(v)) for v in result]
    return [int(v) for v in result]
def fmt_int(v):
    return str(int(v))
def fmt_rat(v):
    q=F(v)
    return f"{q.numerator}/{q.denominator}"
def record(name,domain,vals):
    p=canon(vals)
    encode=fmt_int if domain=="Z" else fmt_rat
    return f"{name}|{domain}|{len(p)}|{','.join(map(encode,p))}\n"
def err(name,kind):
    return f"{name}|ERR|{kind}\n"
def dense(length,salt):
    seed=20261010 ^ salt
    out=[]
    for _ in range(length):
        seed=(seed*6364136223846793005+1442695040888963407)&((1<<64)-1)
        out.append(((seed >> 33)%41)-20)
    out[-1]=1
    return out
def expected():
    parts=["PF-CANONICAL-POLYNOMIAL-V1\n"]
    def z(name,p): parts.append(record(name,"Z",p))
    def q(name,p): parts.append(record(name,"Q",p))
    a=[-7,0,5,-9,1]
    b=[3,-2,0,1]
    z("z.zero",[])
    z("z.add",add(a,b)); z("z.sub",sub(a,b)); z("z.mul",mul(a,b))
    z("z.derivative",deriv(a)); z("z.eval_negative",[evaluate(a,-11)])
    z("z.exact_quotient",a)
    parts.append(err("z.nonexact_error","NonExactDivision"))
    parts.append(err("z.zero_divisor","DivisionByZero"))
    z("z.content",[42]); z("z.primitive",a)
    h=[5,-2,1]; u=[2,3,0,0,0,0,1]; v=[1,-4,0,0,0,1]
    az=mul(mul(h,u),[12]); bz=mul(mul(h,v),[-18])
    z("z.multistage_gcd",sympy_gcd(az,bz))
    q("q.multistage_gcd",sympy_gcd(az,bz,True))
    qa=[F(3,7),F(-2,5),F(0),F(11,13),F(-7,2)]
    qb=[F(5,11),F(1,3),F(-9,7)]
    q("q.add",add(qa,qb));q("q.sub",sub(qa,qb));q("q.mul",mul(qa,qb))
    q("q.derivative",deriv(qa));q("q.eval_negative",[evaluate(qa,F(-3))])
    quotient,remainder=qdiv(qa,qb)
    q("q.quotient",quotient);q("q.remainder",remainder)
    q("q.monic",[x/qa[-1] for x in qa]);q("q.zero",[])
    wide=-(1<<521)+17
    w=[wide,0,-wide,1]
    z("z.wide_input",w)
    z("z.wide_mul",mul(w,[3,-5,7]))
    z("z.wide_derivative",deriv(w))
    rq=[F(1,1<<513),F(-4),F(1)]
    q("q.large_denominator_mul",mul(rq,qa))
    product_plus=add(mul(rq,qb),[F(1,17)])
    wide_quotient,wide_remainder=qdiv(product_plus,qb)
    q("q.large_denominator_quotient",wide_quotient)
    q("q.large_denominator_remainder",wide_remainder)
    d1=dense(97,1); d2=dense(95,2)
    z("z.dense_kara_97x95",mul(d1,d2))
    sparse=[0]*129
    sparse[0]=-31; sparse[67]=11; sparse[128]=1
    z("z.sparse_129x97",mul(sparse,d1))
    z("z.dense_derivative",deriv(d1))
    z("z.dense_eval",[evaluate(d1,-2)])
    late_a=[29878539771007619802695925865,38337129496435065462512069061,26176432220010364463952772943,36455492137305517204742188554,-31583172706667982753055670592,24146646319036245156938719383,-33042054106061461237049169535,27587738088759190405566614755,39124099768788597505792466446,37241909747421405995232538844]
    late_b=[-32510778393112134553770803457,22708147360007579159236093779,20811411071155743672883378584,29928754231223780915951098300,-34504213327522140517185037813,-36255739560194229515569692825,-32845031363358562485482133337,31138875340773139053657760992,-29878475348932783589686397830]
    z("z.late_prs_gcd",sympy_gcd(late_a,late_b))
    q("q.late_prs_gcd",sympy_gcd(late_a,late_b,True))
    z("z.width_guard_fallback",[1<<4097,1])
    z("z.degree_guard_fallback",[-2]+[0]*32+[6])
    parts.append(err("z.nonintegral_error","NonIntegralQuotient"))
    return "".join(parts).encode("ascii")

def run(paths):
    oracle=expected()
    for name in paths:
        actual=Path(name).read_bytes()
        if actual != oracle:
            aa=actual.decode("ascii").splitlines()
            bb=oracle.decode("ascii").splitlines()
            mismatch=next(((i+1,x,y) for i,(x,y) in
                          enumerate(zip(aa,bb)) if x!=y),None)
            if mismatch is None:
                mismatch=("line-count",len(aa),len(bb))
            raise SystemExit(f"ORACLE_MISMATCH {name}: first mismatch {mismatch}")
        print(f"SYMPY_FRACTION_ORACLE_PASS {name} bytes={len(actual)} sha256={hashlib.sha256(actual).hexdigest()} sympy={sp.__version__}")
if __name__ == "__main__":
    if len(sys.argv)<2: raise SystemExit("usage: verify_canonical_workload.py canonical_file...")
    run(sys.argv[1:])
