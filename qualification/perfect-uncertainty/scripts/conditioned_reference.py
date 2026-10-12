#!/usr/bin/env python3
"""Independent exact binary64-input Fraction oracle for 4/6/8-input GUM."""
from fractions import Fraction as F
from hashlib import sha256
from pathlib import Path
import math
import struct
import sys

ROOT = Path(__file__).resolve().parent.parent
TARGET = ROOT / "vectors" / "conditioned_4_6_8_v1.tsv"

def bits(value):
    return struct.unpack(">Q", struct.pack(">d", float(value)))[0]

def compute():
    lines = ["# n rho_f64bits comma_separated_contribution_f64bits exact_variance_f64bits"]
    for n in (4, 6, 8):
        rhos = [
            1.0, math.nextafter(1.0, 0.0),
            1.0 - 2.0**-40, 1.0 - 2.0**-30,
            1.0 - 2.0**-10, 0.5, 0.0, -0.125,
        ]
        if n == 4:
            rhos.append(-0.25)
        for rho in rhos:
            rho_exact = F.from_float(rho)
            # All of these equicorrelation matrices are PSD:
            # -1/(n-1) <= rho <= 1. Their eigenvalues are
            # 1-rho with multiplicity n-1 and 1+(n-1)*rho.
            assert -F(1,n-1) <= rho_exact <= 1
            for exponent in (8, 16, 32, 64, 100, 140):
                scale = float(10**exponent)
                base = [scale if i % 2 == 0 else -scale for i in range(n)]
                for model in range(6):
                    if model == 0:
                        g = base
                    elif model == 1:
                        g = base[:-1] + [1.0]
                    elif model == 2:
                        g = [scale, -scale] + [float(i % 3 - 1) for i in range(n-2)]
                    elif model == 3:
                        g = [scale, scale, -scale, -scale] + [float(i % 3 - 1) for i in range(n-4)]
                    elif model == 4:
                        g = [scale, -scale] + [float(i+1)*scale*2.0**-30 for i in range(n-2)]
                    else:
                        g = [(-1.0 if i%2 else 1.0)*scale*(1.0+(i+1)*2.0**-44) for i in range(n)]
                    q = [F.from_float(x) for x in g]
                    # Exact Fraction quadratic form, not a floating summation
                    # or output from any production Perfect crate.
                    variance = (1-rho_exact)*sum((x*x for x in q),F(0)) + rho_exact*sum(q,F(0))**2
                    assert variance >= 0
                    expected = float(variance)
                    assert math.isfinite(expected)
                    lines.append(f"{n} {bits(rho):016x} " +
                                 ",".join(f"{bits(v):016x}" for v in g) +
                                 f" {bits(expected):016x}")
    assert len(lines)-1 == 900
    return ("\n".join(lines)+"\n").encode("ascii")

def main():
    data = compute()
    if "--generate" in sys.argv:
        if TARGET.exists():
            raise SystemExit("REFUSE_OVERWRITE_CONDITIONED_VECTORS")
        TARGET.write_bytes(data)
    elif TARGET.read_bytes() != data:
        raise SystemExit("CONDITIONED_FRACTION_ORACLE_MISMATCH")
    print("CONDITIONED_FRACTION_ORACLE_PASS",900,sha256(data).hexdigest())
if __name__ == "__main__":
    main()
