#!/usr/bin/env python3
"""Independent exact Python Decimal + Gregorian date range oracle; no Rust outputs."""
from pathlib import Path
from hashlib import sha256
from decimal import Decimal, localcontext
from datetime import date
import sys

DEST=Path(__file__).resolve().parent.parent/"vectors"/"date_range_v1.tsv"
FLOWS=[
 ("2024-02-28","USD","1.00"),("2024-02-28","EUR","-0.100"),
 ("2024-02-29","EUR","2.500"),("2024-02-29","USD","0.015"),
 ("2024-02-29","USD","-0.005"),("2024-02-29","USD","0.010"),
 ("2024-03-01","USD","3.100"),("2024-03-01","EUR","0.500")
]
CASES=[
 ("2024-02-28","2024-02-29","I","E"),
 ("2024-02-28","2024-02-29","E","I"),
 ("2024-02-28","2024-02-29","I","I"),
 ("2024-02-29","2024-03-01","I","E"),
 ("2024-02-29","2024-03-01","E","I"),
 ("2024-02-28","2024-03-01","I","I"),
 ("2024-02-29","2024-02-29","I","I"),
 ("2024-02-29","2024-02-29","I","E"),
]
def render():
    lines=["# scenario start end start_bound end_bound currency exact_total"]
    with localcontext() as ctx:
        ctx.prec=80
        for i,(start,end,first,last) in enumerate(CASES):
            selected=[(currency,Decimal(value)) for dated,currency,value in FLOWS
                if (date.fromisoformat(dated)>date.fromisoformat(start)
                    or (first=="I" and dated==start))
                and (date.fromisoformat(dated)<date.fromisoformat(end)
                    or (last=="I" and dated==end))]
            for currency in sorted(set(c for c,_ in selected)):
                amounts=[a for c,a in selected if c==currency]
                quantum=min(a.as_tuple().exponent for a in amounts)
                total=sum(amounts,Decimal(0)).quantize(Decimal(1).scaleb(quantum))
                lines.append(f"{i} {start} {end} {first} {last} {currency} {total}")
    data=("\n".join(lines)+"\n").encode("ascii")
    return data, len(lines)-1
def main():
    data,count=render()
    if "--generate" in sys.argv:
        if DEST.exists(): raise SystemExit("REFUSE_OVERWRITE")
        DEST.write_bytes(data)
    elif DEST.read_bytes()!=data:
        raise SystemExit("DATE_RANGE_VECTOR_CHANGED")
    print("DATE_RANGE_ORACLE_PASS",count,sha256(data).hexdigest())
if __name__=="__main__":main()
