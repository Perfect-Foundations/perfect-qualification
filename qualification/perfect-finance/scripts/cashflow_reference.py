#!/usr/bin/env python3
"""Exact source-independent cashflow totals using Python Fraction(Decimal).
Finite cases are chosen with representable final totals under caller bounds.
The expected values are mathematical sums, never read from Rust output.
"""
from fractions import Fraction
from decimal import Decimal
from pathlib import Path
from hashlib import sha256
import sys
ROOT=Path(__file__).resolve().parent.parent
FILE=ROOT/"vectors"/"cashflow_cancel_v1.tsv"
CASES=[
 (3,("900","900","-900")),
 (3,("900","-900","900")),
 (3,("-900","-900","900")),
 (3,("999","-999","1")),
 (3,("100","100","-100","-99")),
 (3,("1e20","-1e20","5")),
 (3,("5","-1e20","1e20")),
 (3,("1e50","-1e50","-7")),
 (5,("900.00","900.00","-900.00","-900.00")),
 (5,("500.00","700.00","-700.00","-499.99")),
 (5,("2.005","-2.000","0.001")),
 (5,("1.001","1.002","-1.003","-1.000")),
 (3,("9e20","-9e20","1")),
 (3,("100","-100","0.00")),
 (5,("8.005","-8.005","0.010")),
 (5,("-8.005","8.005","-0.010")),
]
def render():
    lines=["# max_coefficient_digits count decimal_inputs... expected_decimal"]
    for limit,values in CASES:
        rational=sum((Fraction(Decimal(v)) for v in values),Fraction())
        # All cases are finite decimal rationals; Decimal exact sum is
        # validated independently against the Fraction math.
        from decimal import localcontext
        with localcontext() as context:
            context.prec=256
            expected=sum((Decimal(v) for v in values),Decimal(0))
        assert Fraction(expected)==rational,(values,rational,expected)
        minimum=min(Decimal(v).as_tuple().exponent for v in values)
        quantum=Decimal(1).scaleb(minimum)
        with localcontext() as context:
            context.prec=256
            expected=expected.quantize(quantum)
        assert Fraction(expected)==rational
        lines.append(" ".join(map(str,(limit,len(values),*values,expected))))
    return ("\n".join(lines)+"\n").encode("ascii")
def main():
    raw=render()
    if "--generate" in sys.argv:
        if FILE.exists():raise SystemExit("REFUSE_OVERWRITE "+str(FILE))
        FILE.write_bytes(raw)
    elif FILE.read_bytes()!=raw:
        raise SystemExit("CASHFLOW_EXACT_REFERENCE_CHANGED")
    print("INDEPENDENT_EXACT_CASHFLOW_PASS",len(CASES),sha256(raw).hexdigest())
if __name__=="__main__":
    main()
