#!/usr/bin/env python3
"""Qualification-owned oracle; never imports production or reads production bits.

Checks seven mathematical definitions via independent high-precision stdlib
Decimal/Fraction computations; mpmath, when installed, adds a separate oracle.
Portable exact midpoint rounding to IEEE binary32/64 is checked from rational
input, not Python's f32 double-rounded intermediary.
"""
from __future__ import annotations
import decimal
from decimal import Decimal as D, localcontext
from fractions import Fraction as F
from math import comb
from pathlib import Path
import hashlib
import struct
import sys

ROOT = Path(__file__).resolve().parents[1]
VECTOR = ROOT / "vectors" / "native.tsv"
FINGERPRINT = ROOT / "semantic-fingerprint.txt"
DOMAIN = b"PF-CONSTANTS-MATH-V1\\0".replace(b"\\0", bytes((0,)))
NAMES = ("e","golden_ratio","sqrt2","ln2","euler_mascheroni",
         "catalan","apery_zeta3")
FNV_OFFSET = 0xcbf29ce484222325
FNV_PRIME = 0x100000001b3

def parse_vectors():
    rows=[]
    for raw in VECTOR.read_text(encoding="ascii").splitlines():
        if not raw or raw.startswith("#"): continue
        a,b,c=raw.split()
        rows.append((a,int(b,16),int(c,16)))
    assert tuple(x[0] for x in rows)==NAMES, "wrong identity/order/completeness"
    assert len(set(x[0] for x in rows))==7
    return rows

def to_fraction(d):
    return F(d)

def round_ieee(ratio, fraction_bits, bias):
    n,d=ratio.numerator,ratio.denominator
    assert n>0 and d>0
    power=n.bit_length()-d.bit_length()
    def ge_pow2(k):
        return n>=d*(1<<k) if k>=0 else n*(1<<(-k))>=d
    while not ge_pow2(power): power-=1
    while ge_pow2(power+1): power+=1
    shift=fraction_bits-power
    q,r=divmod(n<<shift,d) if shift>=0 else divmod(n,d<<(-shift))
    if 2*r>d or (2*r==d and q&1):q+=1
    if q==1<<(fraction_bits+1):
        q>>=1
        power+=1
    assert 1<<fraction_bits<=q<1<<(fraction_bits+1)
    exponent=power+bias
    assert 1<=exponent<(255 if fraction_bits==23 else 2047)
    return (exponent<<fraction_bits)|(q-(1<<fraction_bits))

def check_interval(name, low, high, expected32, expected64):
    assert 0<low<=high,(name,"nonpositive/invalid interval")
    for frac,bias,expected in ((23,127,expected32),(52,1023,expected64)):
        lhs=round_ieee(low,frac,bias)
        rhs=round_ieee(high,frac,bias)
        assert lhs==rhs==expected,(name,frac,hex(lhs),hex(rhs),hex(expected))
    print("STDLIB_INTERVAL_ROUNDING_PASS",name,flush=True)

def exact_alternating_catalan(n=160):
    # Independently coded Euler transform of beta(2) with rigorous tail < 2^-n.
    terms=[F(1,(2*k+1)**2) for k in range(n+1)]
    transformed=F(0)
    for j in range(n):
        transformed+=terms[0]/(1<<(j+1))
        terms=[a-b for a,b in zip(terms,terms[1:])]
    return transformed,transformed+F(1,1<<n)

def exact_apery(n=100):
    # Classical alternating central-binomial acceleration, with first omitted
    # term giving the alternating remainder interval.
    total=F(0)
    for k in range(1,n+1):
        v=F(5,2*k**3*comb(2*k,k))
        total+=v if k%2 else -v
    omitted=F(5,2*(n+1)**3*comb(2*(n+1),n+1))
    return (total,total+omitted) if n%2==0 else (total-omitted,total)

def bernoulli_even():
    return [
        F(1,6),-F(1,30),F(1,42),-F(1,30),F(5,66),
        -F(691,2730),F(7,6),-F(3617,510),F(43867,798),
        -F(174611,330)
    ]

def decimal_references():
    with localcontext() as ctx:
        ctx.prec=100
        two=D(2)
        five=D(5)
        e=D(1).exp()
        sq2=two.sqrt()
        gold=(D(1)+five.sqrt())/2
        ln2=two.ln()
        n=2048
        harmonic=sum((D(1)/D(k) for k in range(1,n+1)),D(0))
        em=harmonic-D(n).ln()-D(1)/(2*n)
        for k,coef in enumerate(bernoulli_even(),start=1):
            em+= (D(coef.numerator)/D(coef.denominator))/(2*k*D(n)**(2*k))
        # Truncation from next Bernoulli term plus Decimal arithmetic is
        # safely enclosed by 1e-37 for n=2048 / 100-digit precision.
        gamma_err=D("1e-37")
        approximate={
            "e":(e,e),
            "golden_ratio":(gold,gold),
            "sqrt2":(sq2,sq2),
            "ln2":(ln2,ln2),
            "euler_mascheroni":(em-gamma_err,em+gamma_err),
        }
        # Decimal built-in exp/ln/sqrt are documented correctly rounded at
        # context precision; widen their 100-digit results by 1e-90.
        for name in ("e","golden_ratio","sqrt2","ln2"):
            v=approximate[name][0]
            approximate[name]=(v-D("1e-90"),v+D("1e-90"))
        result={k:(F(lo),F(hi)) for k,(lo,hi) in approximate.items()}
        result["catalan"]=exact_alternating_catalan()
        result["apery_zeta3"]=exact_apery()
        return result

def optional_mpmath_oracle(rows):
    try: import mpmath as mp
    except ImportError:
        print("MPMATH_NOT_INSTALLED: retained independent Windows mpmath run required; stdlib exact/Decimal oracle runs here",flush=True)
        return
    mp.mp.dps=180
    values={
      "e":mp.e,"golden_ratio":(mp.mpf(1)+mp.sqrt(5))/2,
      "sqrt2":mp.sqrt(2),"ln2":mp.log(2),
      "euler_mascheroni":mp.euler,"catalan":mp.catalan,"apery_zeta3":mp.zeta(3)
    }
    for name,b32,b64 in rows:
        sign,man,exp,_=values[name]._mpf_
        assert sign==0
        ratio=F(man<<exp) if exp>=0 else F(man,1<<(-exp))
        assert round_ieee(ratio,23,127)==b32
        assert round_ieee(ratio,52,1023)==b64
        print("MPMATH_INDEPENDENT_PASS",name,flush=True)

def fingerprint(rows):
    data=bytearray(DOMAIN)
    for name,b32,b64 in rows:
        encoded=name.encode("utf-8")
        assert len(encoded)<256
        data.append(len(encoded))
        data+=encoded
        data+=struct.pack("<I",b32)
        data+=struct.pack("<Q",b64)
    h=FNV_OFFSET
    for octet in data:
        h=((h^octet)*FNV_PRIME)&0xffffffffffffffff
    return f"{h:016x}",hashlib.sha256(data).hexdigest()

def main():
    rows=parse_vectors()
    intervals=decimal_references()
    for name,b32,b64 in rows:
        low,high=intervals[name]
        check_interval(name,low,high,b32,b64)
    optional_mpmath_oracle(rows)
    expected,sha256=fingerprint(rows)
    if sys.argv[1:]==["--write-fingerprint"]:
        FINGERPRINT.write_text(expected+"\n",encoding="ascii")
    else:
        assert not sys.argv[1:], "unknown args"
        assert FINGERPRINT.read_text(encoding="ascii").strip()==expected,"fingerprint changed"
    print("FINGERPRINT_FNV64",expected,flush=True)
    print("REFERENCE_RECORD_SHA256",sha256,flush=True)
    print("INDEPENDENT_CONSTANTS_ORACLE_PASS 14",flush=True)

if __name__=="__main__":
    main()
