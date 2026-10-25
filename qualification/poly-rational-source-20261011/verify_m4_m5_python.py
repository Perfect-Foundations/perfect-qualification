#!/usr/bin/env python3
"""Independent CPython exact arithmetic check of M4/M5 regression vectors."""
from fractions import Fraction
from hashlib import sha256
import sys
sys.set_int_max_str_digits(0)

cases = []
for bits in (256, 1024, 4096, 8192, 16384):
    d = (1 << bits) + 17
    left = Fraction(17, d)
    right = Fraction(29, d)
    assert left + right == Fraction(46, d)
    assert -left + right == Fraction(12, d)
    assert left + -left == 0
    assert Fraction(-((1 << bits) + 17), -1) == (1 << bits) + 17
    assert Fraction((1 << bits) + 17, -1) == -((1 << bits) + 17)
    cases.extend((str(left + right), str(-left + right)))
for a_bits, b_bits in ((256, 128), (4096, 4096), (8192, 2048), (16384, 4096)):
    a = (1 << a_bits) + 17
    b = (1 << b_bits) + 29
    expected = (1 << (a_bits + b_bits)) + (29 << a_bits) + (17 << b_bits) + 493
    assert a * b == expected
    assert 0 - a * -b == expected
    cases.append(str(expected))
digest = sha256("\n".join(cases).encode("ascii")).hexdigest()
print("M4_M5_INDEPENDENT_PYTHON_EXACT_PASS", len(cases), "vectors", digest)
