#!/usr/bin/env python3
"""Independent stdlib-only M1 oracle for Number Theory's public consumer.

Recomputes retained modular, CRT, and primality vectors from mathematical
integers, not from Perfect Arithmetic or Perfect Number Theory source.
"""
from __future__ import annotations
import math
from pathlib import Path
import random
import sys

ROOT=Path(__file__).resolve().parent
if ROOT.name == 'scripts':
    ROOT=ROOT.parent
VECTORS=ROOT/"vectors"

def modular_rows()->list[str]:
    rng=random.Random(0x6E756D7468656F72)
    rows=["# modulus signed_a signed_b exponent add sub mul power gcd_a inverse_or_minus1"]
    for m in list(range(2,98)) + [127,251,257,65537]:
        for _ in range(3):
            a=rng.randrange(-2*m,2*m+1)
            b=rng.randrange(-2*m,2*m+1)
            exponent=rng.randrange(0,64)
            g=math.gcd(a,m)
            inverse=pow(a,-1,m) if g==1 else -1
            rows.append(f"{m} {a} {b} {exponent} {(a+b)%m} {(a-b)%m} {(a*b)%m} {pow(a,exponent,m)} {g} {inverse}")
    return rows

def crt_rows()->list[str]:
    primes=(2,3,5,7,11,13,17)
    rows=["# modulus_a residue_a modulus_b residue_b expected_combined_residue"]
    for i,m in enumerate(primes):
        for n in primes[i+1:]:
            for a in (0,1,m-1):
                for b in (0,1,n-1):
                    x=next(k for k in range(m*n) if k%m==a%m and k%n==b%n)
                    rows.append(f"{m} {a%m} {n} {b%n} {x}")
    return rows

def small_prime(n:int)->bool:
    if n<2:return False
    for divisor in range(2,math.isqrt(n)+1):
        if n%divisor==0:return False
    return True

def lucas_lehmer(p:int)->bool:
    if not small_prime(p):return False
    number=(1<<p)-1
    acc=4
    for _ in range(p-2):acc=(acc*acc-2)%number
    return acc==0

def primality_rows()->list[str]:
    rows=["# candidate classification P=proven-prime C=composite"]
    for n in range(0,4097):rows.append(f"{n} {'P' if small_prime(n) else 'C'}")
    assert lucas_lehmer(61), "Independent Lucas-Lehmer proof failed"
    rows.append(f"{(1<<61)-1} P")
    composites=[
      (2047,23),
      (4294967297,641),
      (3215031751,151),
      (341550071728321,10670053),
      ((1<<64)-1,3)
    ]
    for n,factor in composites:
        assert n%factor==0 and n>factor>1,(n,factor)
        rows.append(f"{n} C")
    return rows

def main()->int:
    cases={
      "modular.tsv":modular_rows(),
      "crt.tsv":crt_rows(),
      "primality.tsv":primality_rows()
    }
    want_write=sys.argv[1:]==["--write"]
    if sys.argv[1:] and not want_write:
        raise SystemExit("only --write is supported")
    if want_write:VECTORS.mkdir(parents=True,exist_ok=True)
    for name,rows in cases.items():
        content="\n".join(rows)+"\n"
        path=VECTORS/name
        if want_write:path.write_text(content,encoding="ascii")
        else:
            assert path.read_text(encoding="ascii")==content,f"{name} changed"
        print(f"INDEPENDENT_{name.upper()}_PASS {len(rows)-1}")
    print("INDEPENDENT_NUMBER_THEORY_ORACLE_PASS")
    return 0

if __name__=="__main__":
    raise SystemExit(main())
