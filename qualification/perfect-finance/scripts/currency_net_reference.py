#!/usr/bin/env python3
"""Independent exact decimal currency-netting reference; no Rust outputs."""
from pathlib import Path
from decimal import Decimal, localcontext
from hashlib import sha256
import sys
DEST=Path(__file__).resolve().parent.parent/"vectors"/"currency_net_v1.tsv"
CASES=[
 [(1,"USD","1.20"),(2,"EUR","4.50"),(2,"USD","3.10"),(1,"USD","-0.10")],
 [(1,"USD","900.00"),(2,"USD","-900.00"),(3,"USD","0.005"),(1,"JPY","-0.00")],
 [(1,"EUR","0.05"),(2,"EUR","0.05"),(3,"EUR","-0.10"),
  (2,"USD","2.500"),(1,"USD","-0.125")],
]
def render():
    lines=["# scenario currency exact_decimal_total"]
    with localcontext() as ctx:
        ctx.prec=80
        for scenario, flows in enumerate(CASES):
            curr=sorted({c for _,c,_ in flows})
            for c in curr:
                values=[Decimal(s) for _,cc,s in flows if cc==c]
                fine=min(v.as_tuple().exponent for v in values)
                result=sum(values,Decimal(0)).quantize(Decimal(1).scaleb(fine))
                if result.is_zero() and len(values)==1:
                    result=values[0]
                lines.append(f"{scenario} {c} {result}")
    return ("\n".join(lines)+"\n").encode("ascii"),sum(len(set(c for _,c,_ in f)) for f in CASES)
def main():
    data,n=render()
    if "--generate" in sys.argv:
        if DEST.exists():raise SystemExit("REFUSE_EXISTING_VECTOR_OVERWRITE")
        DEST.write_bytes(data)
    elif DEST.read_bytes()!=data:
        raise SystemExit("CURRENCY_NET_REFERENCE_CHANGED")
    print("INDEPENDENT_CURRENCY_NET_PASS",n,sha256(data).hexdigest())
if __name__=="__main__":main()
