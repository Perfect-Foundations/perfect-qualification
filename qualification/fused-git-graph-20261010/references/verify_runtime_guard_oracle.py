#!/usr/bin/env python3
"""Independent Fraction + SymPy exact M2 runtime-growth and zero-domain oracle."""
from fractions import Fraction as F
import sympy as sp

def mul(a,b):
    result=[F(0)]*(len(a)+len(b)-1)
    for i,u in enumerate(a):
        for j,v in enumerate(b):
            result[i+j]+=u*v
    return result

C=(1<<96)+3
a=[F(0)]*60+[F(1)]
b=[F(C),F(1)]
q=[F((-C)**(59-k)) for k in range(60)]
r=[F((-C)**60)]
reconstructed=mul(b,q)
reconstructed[0]+=r[0]
assert reconstructed==a
assert max(abs(v.numerator).bit_length() for v in q)>4096
assert r[0].numerator.bit_length()>4096
z=sp.Symbol("z")
integer_p=sp.Poly(-6+12*z-18*z**2,z,domain="ZZ")
assert list(reversed(sp.gcd(integer_p,sp.Poly(0,z,domain="ZZ")).all_coeffs()))==[6,-12,18]
rational_p=sp.Poly(F(-3,2)+F(9,4)*z-F(15,2)*z**2,z,domain="QQ")
actual=sp.gcd(rational_p,sp.Poly(0,z,domain="QQ"))
assert [F(str(v)) for v in reversed(actual.all_coeffs())]==[F(1,5),F(-3,10),F(1)]
assert q[0]==F((-C)**59) and q[59]==1
print("INDEPENDENT_RUNTIME_QR_GCD_ORACLE_PASS", "q_len",len(q),"r_bits",abs(r[0].numerator).bit_length(),"sympy",sp.__version__)
