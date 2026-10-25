#!/usr/bin/env python3
"""Independent Python Fraction/SymPy V2 boundary oracle, no Rust re-use."""
from fractions import Fraction as F
import hashlib
from pathlib import Path
import sys
import sympy as sp

x=sp.Symbol("x")
def trim(v):
    a=list(v)
    while a and a[-1]==0:a.pop()
    return a
def mul(a,b):
    if not a or not b:return []
    c=[0]*(len(a)+len(b)-1)
    for i,u in enumerate(a):
        for j,v in enumerate(b):c[i+j]+=u*v
    return trim(c)
def sym_gcd(a,b,field=False):
    domain=sp.QQ if field else sp.ZZ
    aa=sp.Poly.from_list(list(reversed(a)),gens=x,domain=domain)
    bb=sp.Poly.from_list(list(reversed(b)),gens=x,domain=domain)
    val=sp.gcd(aa,bb)
    nums=list(reversed(val.all_coeffs()))
    return [F(str(v)) if field else int(v) for v in nums]
def record(id,domain,coeffs):
    a=trim(coeffs)
    if domain=="Z":
        values=",".join(map(str,a))
    else:
        values=",".join(f"{F(v).numerator}/{F(v).denominator}" for v in a)
    return f"{id}|{domain}|{len(a)}|{values}\n"
def err(id,name):return f"{id}|ERR|{name}\n"
def expected():
    rows=["PF-CANONICAL-POLYNOMIAL-V2\n"]
    def z(n,coeffs):rows.append(record(n,"Z",coeffs))
    def q(n,coeffs):rows.append(record(n,"Q",coeffs))
    a0=(1<<2048)-1;b3=(1<<2049)-1
    assert a0.bit_length()==2048 and b3.bit_length()==2049
    assert (a0*b3).bit_length()==4097
    a_over=[a0,0,0,0,1];b_over=[0,0,1,b3]
    z("z.prs_uncancellable_4097_gcd",sym_gcd(a_over,b_over))
    q("q.prs_uncancellable_4097_gcd",sym_gcd(a_over,b_over,True))
    a_fit=[1<<2048,0,0,0,1]
    b_fit=[0,0,1,1<<2047]
    assert (a_fit[0]*b_fit[3]).bit_length()==4096
    z("z.prs_4096_gcd",sym_gcd(a_fit,b_fit))
    w=(1<<257)+3
    wide_quo=[5,-w,w]
    z("z.wide_signed_exact_q",wide_quo)
    z("z.wide_signed_exact_negative_divisor",[-v for v in wide_quo])
    rows.append(err("z.wide_nonexact","NonExactDivision"))
    rows.append(err("z.nonintegral","NonIntegralQuotient"))
    rows.append(err("z.zero_divisor","DivisionByZero"))
    for bits in (512,513):
        divisor=[F(1),-F(1,1<<(bits-1))]
        quo=[F(-3)]+[F(0)]*10+[F(2)]
        remainder=[F(1,17)]
        quotient_product=mul(divisor,quo)
        dividend=quotient_product[:]
        dividend[0]+=remainder[0]
        # Independent exact long division, not a Rust output identity.
        r=dividend[:]
        result=[F(0)]*(len(r)-len(divisor)+1)
        while len(r)>=len(divisor):
            k=len(r)-len(divisor)
            t=F(r[-1])/divisor[-1]
            result[k]=t
            for j,d in enumerate(divisor):r[k+j]-=t*d
            r=trim(r)
        q(f"q.lcm{bits}.quotient",result)
        q(f"q.lcm{bits}.remainder",r)
    for steps in (11,12,160,161):
        monic=[F(0)]*(steps-1)+[F(1)]
        quotient_product=mul([1,1],monic)
        actual_q=[F(0)]*steps
        rr=quotient_product[:]
        while len(rr)>=2:
            k=len(rr)-2
            t=rr[-1]
            actual_q[k]=t
            rr[k]-=t
            rr.pop()
            rr=trim(rr)
        q(f"q.steps{steps}.quotient",actual_q)
        q(f"q.steps{steps}.remainder",rr)
    h2=mul([1,2,1],[1,2,1])
    left=[-42*v for v in mul(h2,[1,0,1])]
    right=[30*v for v in mul(h2,[2,1])]
    z("z.repeated_factor_gcd",sym_gcd(left,right))
    q("q.repeated_factor_gcd",sym_gcd(left,right,True))
    z("z.zero_after_cancellation",[])
    q("q.zero_after_cancellation",[])
    for n in (63,64,65):
        a=[(i*17)%23-11 for i in range(n)]
        b=[(i*19)%29-14 for i in range(n)]
        a[-1]=1;b[-1]=1
        z(f"z.mul_dense_{n}x{n}",mul(a,b))
    return "".join(rows).encode("ascii")

if __name__=="__main__":
    if len(sys.argv)<2:raise SystemExit("pass one or more V2 canonical stream files")
    oracle=expected()
    for item in sys.argv[1:]:
        actual=Path(item).read_bytes()
        if actual!=oracle:
            got=actual.splitlines()
            want=oracle.splitlines()
            diff=next(((i+1,got[i][:180],want[i][:180])
                for i in range(min(len(got),len(want))) if got[i]!=want[i]),None)
            raise SystemExit(f"V2_ORACLE_MISMATCH {item} first={diff} n={len(got)}/{len(want)}")
        print(f"V2_ORACLE_PASS {item} bytes={len(actual)} sha256={hashlib.sha256(actual).hexdigest()} sympy={sp.__version__}")
