"""Seeded independent SymPy exact polynomial oracle; bounded exploratory robustness."""
from pathlib import Path
from random import Random
from sympy import Poly, ZZ, symbols
from time import perf_counter
seed=20261009
rng=Random(seed)
x=symbols("x")
start=perf_counter()
def coefficients(p):
    if p.is_zero: return "_"
    return ",".join(str(int(k)) for k in reversed(p.all_coeffs()))
out=["name\ta\tb\tgcd_z\ta_plus_b\ta_times_b\ta_derivative"]
for i in range(500):
    da=rng.randrange(0,11)
    db=rng.randrange(0,11)
    a=Poly(sum(rng.randrange(-16,17)*x**j for j in range(da+1)),x,domain=ZZ)
    b=Poly(sum(rng.randrange(-16,17)*x**j for j in range(db+1)),x,domain=ZZ)
    if i%4==0:
        common=Poly(x+1 if i%8 else 2*x+3,x,domain=ZZ)
        a=a*common;b=b*common
    if i%11==0: a=a.mul_ground(-6)
    if i%13==0: b=b.mul_ground(15)
    if i%37==0: a=Poly(0,x,domain=ZZ)
    if i%41==0: b=Poly(0,x,domain=ZZ)
    # Avoid both zero for divisibility expectations.
    if a.is_zero and b.is_zero: b=Poly(1,x,domain=ZZ)
    g=a.gcd(b)
    out.append("\t".join([str(i),coefficients(a),coefficients(b),coefficients(g),
                          coefficients(a+b),coefficients(a*b),coefficients(a.diff())]))
dest=Path(__file__).parent.parent/"vectors"/"robustness-v1.tsv"
dest.write_text("\n".join(out)+"\n",encoding="utf-8")
print(f"PASS SymPy seeded {len(out)-1} cases seed={seed}, elapsed_s={perf_counter()-start:.3f}")
