#!/usr/bin/env python3
"""Independent exact Fraction oracle for finite Decimal aggregation."""
from decimal import Decimal
from fractions import Fraction
from hashlib import sha256
from pathlib import Path
import random
import sys

ROOT=Path(__file__).resolve().parent.parent
PATH=ROOT/"vectors"/"multioperand_v1.tsv"
rng_seed=26101105

def samples():
    result=[
      (2,["0.035","-0.23","0.097"]),
      (3,["1e10000","-1e10000","0.005"]),
      (3,["99","99","-1"]),
      (2,["0.009","0.009","-0.001"]),
      (3,["999","-999","0.000"]),
      (3,["1e250","-10e249","-0.005"])]
    r=random.Random(rng_seed)
    for n in (3,4,8,16,32,64,128):
        for trial in range(6):
            exponent=10000 if trial==0 else 11+13*trial
            pairs=(n-1)//2 if n%2 else (n-2)//2
            flows=[x for _ in range(pairs) for x in
                (f"1e{exponent}",f"-1e{exponent}")]
            if n%2:
                flows+=["0.005"]
            else:
                flows+=["0.035","-0.23"]
            assert len(flows)==n
            r.shuffle(flows)
            result.append((3,flows))
    for n in (3,4,8,16):
        for trial in range(10):
            d=3
            flows=[str(Decimal(r.choice((-1,1))*r.randrange(1,1000))
                   .scaleb(r.randrange(-3,3))) for _ in range(n)]
            result.append((d,flows))
    return result

def make():
    lines=["# digits count amounts... expected_coefficient_at_minimum_exponent_or_LIMIT"]
    yes=no=0
    for digits,values in samples():
        total=sum((Fraction(Decimal(v)) for v in values),Fraction())
        exponent=min(Decimal(v).as_tuple().exponent for v in values)
        coefficient=total*Fraction(10)**(-exponent)
        assert coefficient.denominator==1
        number=coefficient.numerator
        if abs(number) >= 10**digits:
            outcome="LIMIT";no+=1
        else:
            outcome=f"{number}e{exponent}";yes+=1
        lines.append(" ".join((str(digits),str(len(values)),*values,outcome)))
    return ("\n".join(lines)+"\n").encode("ascii"),yes,no

def main():
    raw,positive,refused=make()
    if "--generate" in sys.argv:
        if PATH.exists():raise SystemExit("REFUSE_OVERWRITE")
        PATH.write_bytes(raw)
    elif PATH.read_bytes()!=raw:
        raise SystemExit("FROZEN_MULTI-OPERAND_ORACLE_MISMATCH")
    print("EXACT_MULTI_OPERAND_ORACLE_PASS",positive,refused,
        "SHA256",sha256(raw).hexdigest())
if __name__=="__main__":main()
