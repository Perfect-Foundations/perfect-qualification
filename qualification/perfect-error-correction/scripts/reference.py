#!/usr/bin/env python3
"""Frozen independent GF(5)/GF(16) and Hamming mathematical reference."""
from pathlib import Path
from hashlib import sha256
import sys
ROOT=Path(__file__).resolve().parent.parent
def bits(code):
    return "".join("1" if b else "0" for b in code)
def hamming(value,ext):
    msg=[bool(value&(1<<i)) for i in range(4)]
    out=[False]*7
    index=0
    for position in range(1,8):
        if position&(position-1):
            out[position-1]=msg[index]
            index+=1
    for power in (1,2,4):
        out[power-1]=bool(sum(out[j-1] for j in range(1,8)
            if (j&power) and j!=power)%2)
    if ext:out.append(bool(sum(out)%2))
    return bits(out)
def gf16mul(a,b):
    result=0
    while b:
        if b&1:result^=a
        a<<=1
        if a&16:a^=0x13
        b>>=1
    return result
def gf16eval(poly,x):
    out=0
    for a in poly[::-1]:
        out=gf16mul(out,x)^a
    return out
def corpus():
    h=["# value code7 code8"]
    for value in range(16):
        h.append(f"{value} {hamming(value,False)} {hamming(value,True)}")
    rs5=["# a b c0 c1 c2 c3"]
    for a in range(5):
        for b in range(5):
            rs5.append(" ".join(map(str,[a,b]+[(a+b*x)%5 for x in range(4)])))
    rs16=["# m0 m1 m2 m3 c0 c1 c2 c3 c4 c5 c6 c7"]
    for message in ((1,3,5,7),(2,4,8,15),(0,1,0,2),(15,14,13,12)):
        rs16.append(" ".join(map(str, list(message)+[
            gf16eval(message,x) for x in range(8)])))
    return {"hamming_v1.tsv":"\n".join(h)+"\n",
            "rs_gf5_v1.tsv":"\n".join(rs5)+"\n",
            "rs_gf16_v1.tsv":"\n".join(rs16)+"\n"}
def main():
    for name,data in corpus().items():
        path=ROOT/"vectors"/name
        raw=data.encode("ascii")
        if "--generate" in sys.argv:
            if path.exists():raise SystemExit(f"REFUSE_OVERWRITE {path}")
            path.write_bytes(raw)
        elif path.read_bytes()!=raw:
            raise SystemExit(f"FROZEN_MISMATCH {path}")
        print("FEC_PYTHON_ORACLE_PASS",name,len(data.splitlines())-1,sha256(raw).hexdigest())
if __name__=="__main__":main()
