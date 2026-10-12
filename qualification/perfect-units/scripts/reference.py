#!/usr/bin/env python3
"""Independent stdlib Fraction oracle; no Perfect source or binary imported.
--generate creates immutable V1 fixture once. Default recomputes and verifies.
"""
from pathlib import Path
from fractions import Fraction as F
from hashlib import sha256
import sys
ROOT=Path(__file__).resolve().parent.parent
RATIOS={"km_to_m":F(1000),"m_to_km":F(1,1000),
        "ms_to_s":F(1,1000),"s_to_ms":F(1000)}
CASES=[F(x,y) for x,y in [(-100,1),(-40,1),(0,1),(1,3),
    (1,1),(32,1),(100,1),(212,1),(1234567,891),(987,32)]]
def source():
    lin=["# conversion source_n source_d expected_n expected_d"]
    for kind,factor in RATIOS.items():
        for value in CASES:
            expected=value*factor
            lin.append(f"{kind} {value.numerator} {value.denominator} "
                f"{expected.numerator} {expected.denominator}")
    affine=["# scale source_n source_d kelvin_n kelvin_d"]
    for scale,factor,offset in [
        ("C",F(1),F(27315,100)),("F",F(5,9),F(45967,180))]:
        for value in CASES:
            kelvin=value*factor+offset
            affine.append(f"{scale} {value.numerator} {value.denominator} "
                f"{kelvin.numerator} {kelvin.denominator}")
    return {"linear_v1.tsv":"\n".join(lin)+"\n",
            "affine_v1.tsv":"\n".join(affine)+"\n"}
def main():
    generate="--generate" in sys.argv
    for name,data in source().items():
        file=ROOT/"vectors"/name
        raw=data.encode("ascii")
        if generate:
            if file.exists():raise SystemExit(f"Refuse overwrite {file}")
            file.write_bytes(raw)
        elif file.read_bytes()!=raw:
            raise SystemExit(f"ORACLE_MISMATCH {name}")
        print("FRACTION_ORACLE",name,len(data.splitlines())-1,
            sha256(raw).hexdigest())
    print("INDEPENDENT_UNITS_REFERENCE_PASS")
if __name__=="__main__":main()
