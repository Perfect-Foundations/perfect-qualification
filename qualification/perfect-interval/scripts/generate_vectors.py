#!/usr/bin/env python3
"""Independent verification vectors: Fraction exact extrema + MPFR 4.2.1.
Uses installed libmpfr.so.6 through ctypes, not production Rust.
Every result at p bits is checked to contain the Fraction exact extremum.
"""
import ctypes as C
from fractions import Fraction as F
from hashlib import sha256
from pathlib import Path
import random

mp=C.CDLL("libmpfr.so.6")
class V(C.Structure):
    _fields_=[("prec",C.c_long),("sign",C.c_int),("exp",C.c_long),("d",C.c_void_p)]
ptr=C.POINTER(V)
mp.mpfr_get_version.restype=C.c_char_p
mp.mpfr_init2.argtypes=[ptr,C.c_long]
mp.mpfr_clear.argtypes=[ptr]
mp.mpfr_set_si_2exp.argtypes=[ptr,C.c_long,C.c_long,C.c_int]
for name in ("mpfr_add","mpfr_sub","mpfr_mul","mpfr_div"):
    getattr(mp,name).argtypes=[ptr,ptr,ptr,C.c_int]
mp.mpfr_get_str.argtypes=[C.c_void_p,C.POINTER(C.c_long),C.c_int,C.c_size_t,ptr,C.c_int]
mp.mpfr_get_str.restype=C.c_void_p
mp.mpfr_free_str.argtypes=[C.c_void_p]
assert mp.mpfr_get_version().decode().startswith("4.2.1")

def readbits(x):
    exp=C.c_long()
    mem=mp.mpfr_get_str(None,C.byref(exp),2,0,C.byref(x),0)
    assert mem
    digits=C.string_at(mem).decode("ascii")
    mp.mpfr_free_str(mem)
    negative=digits.startswith("-")
    bits=digits.lstrip("-")
    assert set(bits)<=set("01"),digits
    value=-int(bits,2) if negative else int(bits,2)
    exponent=exp.value-len(bits)
    while value and value%2==0:
        value//=2
        exponent+=1
    if not value: exponent=0
    exact=F(value << exponent) if exponent>=0 else F(value,1<<(-exponent))
    return value,exponent,exact

def directed(left,right,operation,prec,rounding):
    a,b,c=V(),V(),V()
    for v,p in ((a,256),(b,256),(c,prec)):mp.mpfr_init2(C.byref(v),p)
    try:
        mp.mpfr_set_si_2exp(C.byref(a),left[0],left[1],0)
        mp.mpfr_set_si_2exp(C.byref(b),right[0],right[1],0)
        getattr(mp,"mpfr_"+operation)(C.byref(c),C.byref(a),C.byref(b),rounding)
        return readbits(c)
    finally:
        for v in (a,b,c):mp.mpfr_clear(C.byref(v))

def val(arg):
    n,e=arg
    return F(n<<e) if e>=0 else F(n,1<<(-e))

def reference(op,a,b,prec):
    la,ua=map(val,a)
    lb,ub=map(val,b)
    one=(1,0)
    zero=(0,0)
    if op=="add":
        tasks=[(la+lb,a[0],b[0],"add"),(ua+ub,a[1],b[1],"add")]
    elif op=="sub":
        tasks=[(la-ub,a[0],b[1],"sub"),(ua-lb,a[1],b[0],"sub")]
    elif op=="neg":
        tasks=[(-ua,zero,a[1],"sub"),(-la,zero,a[0],"sub")]
    elif op=="mul":
        tasks=[(x*y,xa,ya,"mul") for x,xa in ((la,a[0]),(ua,a[1])) for y,ya in ((lb,b[0]),(ub,b[1]))]
    elif op in ("div","recip"):
        if op=="recip":
            if la<=0<=ua:return None
            tasks=[(F(1)/x,one,xa,"div") for x,xa in ((la,a[0]),(ua,a[1]))]
        else:
            if lb<=0<=ub:return None
            tasks=[(x/y,xa,ya,"div") for x,xa in ((la,a[0]),(ua,a[1])) for y,ya in ((lb,b[0]),(ub,b[1]))]
    else:raise AssertionError(op)
    small=min(t[0] for t in tasks)
    big=max(t[0] for t in tasks)
    lower_choices=[directed(l,r,method,prec,3) for v,l,r,method in tasks if v==small]
    upper_choices=[directed(l,r,method,prec,2) for v,l,r,method in tasks if v==big]
    low=min(lower_choices,key=lambda x:x[2])
    high=max(upper_choices,key=lambda x:x[2])
    assert low[2]<=small<=big<=high[2],(op,a,b,prec,low,high,small,big)
    return low,high

def make():
    seed=0x5199_1B8A
    rng=random.Random(seed)
    samples=[]
    exponents=[-2048,-257,-64,-24,-3,-1,0,1,3,24,64,257,2048]
    opcodes=["add","sub","mul","div","recip","neg"]
    # Include zero, negative/positive, near-power-of-two, far exponent pairs.
    patterns=[(-7,-3),(-1,0),(0,0),(0,1),(3,7),(-7,7),(-1,1),(1,1),(-1,-1),(7,9),(15,17)]
    for ix in range(260):
        p=[3,4,5,8,13,24,53][ix%7]
        op=opcodes[ix%6]
        ap=patterns[rng.randrange(len(patterns))]
        bp=patterns[rng.randrange(len(patterns))]
        ae=exponents[ix%len(exponents)]
        be=exponents[(ix*5+3)%len(exponents)]
        a=((ap[0],ae),(ap[1],ae))
        b=((bp[0],be),(bp[1],be))
        reference_result=reference(op,a,b,p)
        if reference_result is not None:
            low,high=reference_result
            # Tab-separated numeric fields; second-operand values unused for unary ops
            row=[op,str(p),str(a[0][0]),str(a[0][1]),str(a[1][0]),str(a[1][1]),
                str(b[0][0]),str(b[0][1]),str(b[1][0]),str(b[1][1]),
                str(low[0]),str(low[1]),str(high[0]),str(high[1])]
            samples.append("\t".join(row))
    assert len(samples)>190,len(samples)
    raw="# MPFR 4.2.1 / Python fractions.Fraction / seed=0x51991b8a / directed RNDD=3 RNDU=2\n"+"\n".join(samples)+"\n"
    target=Path(__file__).resolve().parents[1] / "vectors" / "real_m1_mpfr_20261008.tsv"
    target.write_text(raw,encoding="ascii")
    print("MPFR_VERSION",mp.mpfr_get_version().decode())
    print("EXACT_FRACTION_MPFR_ENCLOSURES_CERTIFIED",len(samples))
    print("CORPUS_SHA256",sha256(raw.encode()).hexdigest())
    print("VECTOR_PATH",target)
if __name__=="__main__":make()
