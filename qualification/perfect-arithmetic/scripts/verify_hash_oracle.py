#!/usr/bin/env python3
"""Independent Python integer-magnitude oracle for Arithmetic public Hash streams."""
from pathlib import Path
import sys

ROOT=Path(__file__).resolve().parents[1] / "vectors"
PATH=ROOT/"canonical_hash.tsv"
CASES=[
 ("N","+",-1,0), ("N","+",-1,1), ("N","+",-1,255),
 ("N","+",-1,4294967295), ("N","+",-1,4294967296),
 ("N","+",31,0), ("N","+",32,0), ("N","+",33,3),
 ("N","+",63,23), ("N","+",64,0), ("N","+",65,7),
 ("N","+",127,13), ("N","+",256,0), ("N","+",1024,37),
 ("N","+",2048,0), ("I","+",-1,0), ("I","-",-1,0),
 ("I","+",-1,1), ("I","-",-1,1), ("I","+",31,0),
 ("I","-",32,4294967295), ("I","+",127,123),
 ("I","-",256,7), ("I","+",1024,37), ("I","-",2048,3)
]
def stream(kind,sign,shift,low):
    value=(1<<shift if shift>=0 else 0)+low
    magnitude=value
    prefix=b"perfect-arithmetic:"+ (b"natural" if kind=="N" else b"integer")+b":v1\0"
    data=bytearray(prefix)
    if kind=="I":
        data.append(int(sign=="-" and value!=0))
    bitlen=magnitude.bit_length()
    data.extend(bitlen.to_bytes(8,"little"))
    for off in range(0,bitlen,32):
        data.extend(((magnitude>>off)&0xFFFFFFFF).to_bytes(4,"little"))
    return data.hex()
def content():
    result=["# kind sign power_of_two_shift (-1=none) added_unsigned_u64 expected_hash_write_stream_hex"]
    for kind,sign,shift,low in CASES:
        result.append(f"{kind} {sign} {shift} {low} {stream(kind,sign,shift,low)}")
    return "\n".join(result)+"\n"
if __name__=="__main__":
    expected=content()
    if sys.argv[1:]==["--write"]:
        PATH.write_text(expected,encoding="ascii")
    else:
        assert PATH.read_text(encoding="ascii")==expected,"canonical hash vectors differ"
    print("INDEPENDENT_ARITHMETIC_HASH_VECTORS_PASS",len(CASES))
