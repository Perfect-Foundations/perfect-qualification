#!/usr/bin/env python3
"""Independent SymPy Z[x]/Q[x] exhaustive bounded-degree GCD oracle.

Vector order and serialization are versioned; no production Rust computation
is used to create expected coefficients.
"""
import hashlib
from itertools import product
from pathlib import Path
from sympy import Poly, QQ, ZZ, symbols

x = symbols("x")
root = Path(__file__).resolve().parent
destination = root / "m2_degree2_zq_gcd_sympy_v1.tsv"

def canonical(c):
    c = list(c)
    while c and c[-1] == 0:
        c.pop()
    return tuple(c)

def poly(c, field):
    return Poly.from_list(list(reversed(c)) if c else [0], x, domain=field)

def low_to_high(p):
    if p.is_zero:
        return ()
    return tuple(reversed(p.all_coeffs()))

def ztxt(c):
    return ",".join(str(n) for n in c) if c else "_"

def qtxt(c):
    return ",".join(f"{v.p}/{v.q}" for v in c) if c else "_"

base = [()]
for d in range(3):
    base.extend(canonical(low + (lead,)) for low in product((-1, 0, 1), repeat=d)
                for lead in (-1, 1))
extra = (
    (2,), (-2,), (2, 2), (-2, -2), (-3, 0, 6), (0, -4, 2),
    (0, 0, -2), (6, -9, 3), (3, 0, -3), (0, 3, 3),
)
cases = list(dict.fromkeys(base + [canonical(v) for v in extra]))
assert len(cases) == 37 and cases[0] == ()
lines = []
for a in cases:
    for b in cases:
        az, bz = poly(a, ZZ), poly(b, ZZ)
        g = az.gcd(bz)
        if not g.is_zero and g.LC() < 0:
            g = -g
        aq, bq = poly(a, QQ), poly(b, QQ)
        h = aq.gcd(bq)
        if not h.is_zero:
            h = h.monic()
        zc, qc = low_to_high(g), low_to_high(h)
        assert (not zc or zc[-1] > 0) and (not qc or qc[-1] == 1)
        lines.append("\t".join((ztxt(a), ztxt(b), ztxt(zc), qtxt(qc))))
assert len(lines) == 1369
data = ("\n".join(lines) + "\n").encode("ascii")
destination.write_bytes(data)
print(f"ZQ_GCD_REFERENCE rows={len(lines)} bytes={len(data)} sha256={hashlib.sha256(data).hexdigest()} path={destination}")
