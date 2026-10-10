"""M2 long exact field-division source-independent fixed-vector generator.

Run Python 3.11+: python references/m2_long_fraction_oracle.py
Uses Python stdlib fractions.Fraction; independent from Rust arithmetic.
"""
from fractions import Fraction as F

b = [F(-42,5), F(84,7), F(-126,11), F(42,13)]
q = [F(5)] + [F(0)]*2 + [F(-3)] + [F(0)]*5 + [F(2)] + [F(0)]*5 + [F(1)]
r = [F(1,17), F(-4,19), F(3,23)]
a = [F(0)]*(len(b)+len(q)-1)
for i,bc in enumerate(b):
    for j,qc in enumerate(q):
        a[i+j] += bc*qc
for i,rc in enumerate(r):
    a[i] += rc
expected = [
    (-713,17),(1136,19),(-14457,253),(2688,65),(-36,1),
    (378,11),(-126,13),(0,1),(0,1),(-84,5),(24,1),(-252,11),
    (84,13),(0,1),(0,1),(-42,5),(12,1),(-126,11),(42,13)
]
assert [(c.numerator,c.denominator) for c in a] == expected
assert len(q)==16 and len(a)==19 and len(b)==4
assert r[-1] != 0 and len(r) < len(b)
print("FRACTION_LONG_DIVISION_REFERENCE_PASS A_len=19 B_len=4 Q_len=16 R_len=3")
