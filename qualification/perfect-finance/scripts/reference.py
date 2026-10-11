#!/usr/bin/env python3
"""Independent Python datetime, Decimal and Fraction reference corpus."""
from pathlib import Path
from datetime import date,timedelta
from decimal import Decimal,ROUND_HALF_EVEN,ROUND_HALF_UP,ROUND_DOWN
from fractions import Fraction
from hashlib import sha256
import sys
ROOT=Path(__file__).resolve().parent.parent
DATES=[((2024,2,29),(2024,3,31)),((2024,2,28),(2024,3,1)),
       ((2023,2,28),(2023,3,31)),((2024,1,31),(2024,2,29)),
       ((2024,2,29),(2024,2,29)),((2023,1,1),(2024,1,1)),
       ((2024,1,1),(2025,1,1)),((2024,12,31),(2025,1,31)),
       ((2024,3,1),(2024,2,29))]
def last_feb(d):
    return d.month==2 and (d+timedelta(days=1)).month!=2
def day_count(name,a,b):
    if name=="ACT360":return Fraction((b-a).days,360)
    if name=="ACT365":return Fraction((b-a).days,365)
    d1,d2=a.day,b.day
    if name=="EU30360":
        d1=min(30,d1);d2=min(30,d2)
    else:
        feb=last_feb(a)
        if d1==31 or feb:d1=30
        if (last_feb(b) and feb) or (d2==31 and d1>=30):d2=30
    days=360*(b.year-a.year)+30*(b.month-a.month)+d2-d1
    return Fraction(days,360)
def corpus():
    dates=["# convention Y1 M1 D1 Y2 M2 D2 fraction_n fraction_d"]
    for a,b in DATES:
        for name in ("ACT360","ACT365","US30360","EU30360"):
            start,end=date(*a),date(*b)
            result=day_count(name,start,end)
            dates.append(" ".join(map(str,(name,*a,*b,result.numerator,result.denominator))))
    money=["# amount target_mode quantum expected"]
    amounts=["-2.345","2.345","1.205","-1.205","0.000","-0.000",
             "2.355","-2.355","0.015","-0.015","999999999999999999.005",
             "12345678901234567890.00015","1.000005","-0.000009",
             "999999999999999999.999","-88888888888888.004"]
    for value in amounts:
        for mode,decimal_mode in [
            ("EVEN",ROUND_HALF_EVEN),("AWAY",ROUND_HALF_UP),("ZERO",ROUND_DOWN)]:
            out=Decimal(value).quantize(Decimal("0.01"),rounding=decimal_mode)
            money.append(f"{value} {mode} -2 {out}")
    return {"day_count_v1.tsv":"\n".join(dates)+"\n",
            "money_quantize_v1.tsv":"\n".join(money)+"\n"}
def main():
    for name,text in corpus().items():
        path=ROOT/"vectors"/name
        raw=text.encode("ascii")
        if "--generate" in sys.argv:
            if path.exists():raise SystemExit("REFUSE_OVERWRITE "+str(path))
            path.write_bytes(raw)
        elif path.read_bytes()!=raw:
            raise SystemExit("ORACLE_MISMATCH "+str(path))
        print("FINANCE_PYTHON_ORACLE_PASS",name,len(text.splitlines())-1,sha256(raw).hexdigest())
if __name__=="__main__":main()
