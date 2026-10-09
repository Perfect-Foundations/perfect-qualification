#!/usr/bin/env python3
"""Independent MPFI 1.5.3 differential oracle for the qualification TSV.

Verification-only ctypes ABI wrapper for Ubuntu noble x86_64 MPFI runtime;
no MPFI/GMP/MPFR crate dependency and no production code. The installed
libmpfr6 supplies MPFR arithmetic; MPFI is loaded from unprivileged extracted
Ubuntu .deb referenced via PFQ_MPFI_LIB.
"""
import ctypes as C
from fractions import Fraction as F
from hashlib import sha256
import os
from pathlib import Path

class MPFRValue(C.Structure):
    _fields_=[("prec",C.c_long),("sign",C.c_int),("exp",C.c_long),("d",C.c_void_p)]
class MPFIValue(C.Structure):
    _fields_=[("left",MPFRValue),("right",MPFRValue)]
PFR=C.POINTER(MPFRValue)
PFI=C.POINTER(MPFIValue)
assert C.sizeof(MPFRValue)==32 and C.sizeof(MPFIValue)==64,"unsupported MPFI ABI"

mpfr=C.CDLL("libmpfr.so.6")
mpfi=C.CDLL(os.environ["PFQ_MPFI_LIB"])
mpfr.mpfr_get_version.restype=C.c_char_p
mpfi.mpfi_get_version.restype=C.c_char_p
mpfr.mpfr_init2.argtypes=[PFR,C.c_long]
mpfr.mpfr_clear.argtypes=[PFR]
mpfr.mpfr_set_si_2exp.argtypes=[PFR,C.c_long,C.c_long,C.c_int]
mpfr.mpfr_get_str.argtypes=[C.c_void_p,C.POINTER(C.c_long),C.c_int,C.c_size_t,PFR,C.c_int]
mpfr.mpfr_get_str.restype=C.c_void_p
mpfr.mpfr_free_str.argtypes=[C.c_void_p]
mpfi.mpfi_init2.argtypes=[PFI,C.c_long]
mpfi.mpfi_clear.argtypes=[PFI]
mpfi.mpfi_interv_fr.argtypes=[PFI,PFR,PFR]
mpfi.mpfi_get_left.argtypes=[PFR,PFI]
mpfi.mpfi_get_right.argtypes=[PFR,PFI]
for fn in ("add","sub","mul","div"):
    getattr(mpfi,"mpfi_"+fn).argtypes=[PFI,PFI,PFI]
for fn in ("inv","neg"):
    getattr(mpfi,"mpfi_"+fn).argtypes=[PFI,PFI]
assert mpfr.mpfr_get_version().decode().startswith("4.2.1")
assert mpfi.mpfi_get_version().decode().startswith("1.5.3")

def fraction(n,e):
    return F(n<<e) if e>=0 else F(n,1<<(-e))

def exact_read(x):
    exp=C.c_long()
    memory=mpfr.mpfr_get_str(None,C.byref(exp),2,0,C.byref(x),0)
    assert memory
    try: digits=C.string_at(memory).decode("ascii")
    finally:mpfr.mpfr_free_str(memory)
    negative=digits.startswith("-")
    bits=digits.lstrip("-")
    assert set(bits)<=set("01"),("unexpected special value",digits)
    n=int(bits,2)
    if negative:n=-n
    exponent=exp.value-len(bits)
    while n and n%2==0:
        n//=2;exponent+=1
    if n==0:exponent=0
    return n,exponent,fraction(n,exponent)

def evaluate(op,a,b):
    left=[MPFRValue(),MPFRValue()]
    right=[MPFRValue(),MPFRValue()]
    result_sides=[MPFRValue(),MPFRValue()]
    intervals=[MPFIValue(),MPFIValue(),MPFIValue()]
    for v in left+right+result_sides:mpfr.mpfr_init2(C.byref(v),256)
    for iv in intervals:mpfi.mpfi_init2(C.byref(iv),256)
    try:
        for x,(n,e) in zip(left,a):
            mpfr.mpfr_set_si_2exp(C.byref(x),n,e,0)
        for x,(n,e) in zip(right,b):
            mpfr.mpfr_set_si_2exp(C.byref(x),n,e,0)
        mpfi.mpfi_interv_fr(C.byref(intervals[0]),C.byref(left[0]),C.byref(left[1]))
        mpfi.mpfi_interv_fr(C.byref(intervals[1]),C.byref(right[0]),C.byref(right[1]))
        if op in ("add","sub","mul","div"):
            getattr(mpfi,"mpfi_"+op)(C.byref(intervals[2]),C.byref(intervals[0]),C.byref(intervals[1]))
        elif op in ("inv","neg"):
            getattr(mpfi,"mpfi_"+op)(C.byref(intervals[2]),C.byref(intervals[0]))
        else:raise AssertionError(op)
        mpfi.mpfi_get_left(C.byref(result_sides[0]),C.byref(intervals[2]))
        mpfi.mpfi_get_right(C.byref(result_sides[1]),C.byref(intervals[2]))
        return exact_read(result_sides[0]),exact_read(result_sides[1])
    finally:
        for v in intervals:mpfi.mpfi_clear(C.byref(v))
        for v in left+right+result_sides:mpfr.mpfr_clear(C.byref(v))

def bounds(op,a,b):
    lo,hi=[fraction(*x) for x in a]
    bl,bh=[fraction(*x) for x in b]
    if op=="add":return lo+bl,hi+bh
    if op=="sub":return lo-bh,hi-bl
    if op=="neg":return -hi,-lo
    if op=="recip":
        assert not lo<=0<=hi
        return min(F(1)/lo,F(1)/hi),max(F(1)/lo,F(1)/hi)
    if op in ("mul","div"):
        if op=="div": assert not bl<=0<=bh
        values=[x*y if op=="mul" else x/y for x in (lo,hi) for y in (bl,bh)]
        return min(values),max(values)
    raise AssertionError(op)

def main():
    path=Path(__file__).resolve().parents[1]/"vectors/real_m1_mpfr_20261008.tsv"
    raw=path.read_bytes()
    assert sha256(raw).hexdigest()=="604bf0ae6b6d506c4aaa93ebe0e374a23fb7b1327222b9a53aa43293d163c744"
    count=0
    digest=sha256()
    for idx,line in enumerate(raw.decode("ascii").splitlines(),start=1):
        if not line or line.startswith("#"):continue
        cells=line.split()
        assert len(cells)==14
        op=cells[0]
        a=((int(cells[2]),int(cells[3])),(int(cells[4]),int(cells[5])))
        b=((int(cells[6]),int(cells[7])),(int(cells[8]),int(cells[9])))
        certified_lower=fraction(int(cells[10]),int(cells[11]))
        certified_upper=fraction(int(cells[12]),int(cells[13]))
        true_lower,true_upper=bounds(op,a,b)
        (lower,upper)=evaluate("inv" if op=="recip" else op,a,b)
        assert lower[2]<=true_lower<=true_upper<=upper[2],(idx,op,"MPFI soundness",lower,upper)
        # MPFI reference at 256-bit working precision is at least as tight as
        # the independently computed MPFR p-bit outward rounded oracle.
        assert certified_lower<=lower[2] and upper[2]<=certified_upper,(idx,op,"MPFR/MPFI cross-check")
        digest.update(f"{idx}:{op}:{lower[0]}:{lower[1]}:{upper[0]}:{upper[1]}\n".encode())
        count+=1
    assert count==226,count
    print("MPFI_VERSION",mpfi.mpfi_get_version().decode())
    print("MPFR_VERSION",mpfr.mpfr_get_version().decode())
    print("MPFI_PACKAGED_ABI_SIZE",C.sizeof(MPFIValue),"MPFR_STRUCT_SIZE",C.sizeof(MPFRValue))
    print("MPFI_256_BIT_INDEPENDENT_CASES_PASS",count)
    print("MPFI_REFERENCE_OUTPUT_SHA256",digest.hexdigest())
    print("CONTAINMENT_MPFR_COARSE_MPFI_FINE_EXACT_FRACTION_PASS")
if __name__=="__main__":main()
