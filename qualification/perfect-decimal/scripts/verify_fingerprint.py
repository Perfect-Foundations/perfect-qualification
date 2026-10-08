#!/usr/bin/env python3
"""Independent, stdlib-only decimal representation reference fingerprint."""
from decimal import Decimal
from pathlib import Path
import argparse
ROOT=Path(__file__).resolve().parents[1]
VECTORS=ROOT/'vectors'/'representation.tsv'
FINGERPRINT=ROOT/'semantic-fingerprint.txt'
INPUTS=[
'0','-0e-5','0e4','1','+1','10e-1','100e-2','-1200e-4','-5000e-3',
'12300e-5','999e-2','-75e1','1.25','-0.125','+.5','00123e-2',
'10e15','1e-15','-10000e-7','-900000e4','999999e-5',
]
def raw_tuple_display(sign:int, digits:tuple[int,...], exponent:int)->str:
    coefficient=''.join(map(str,digits)) or '0'
    return ('-' if sign else '')+coefficient+(('e'+str(exponent)) if exponent else '')
def row(s:str)->tuple[str,str,str]:
    parts=Decimal(s).as_tuple()
    exponent=int(parts.exponent)
    if not isinstance(parts.exponent,int): raise ValueError('nonfinite value')
    original=raw_tuple_display(parts.sign,parts.digits,exponent)
    coeff=list(parts.digits)
    if all(digit==0 for digit in coeff):
        normalized='0'
    else:
        while len(coeff)>1 and coeff[-1]==0:
            coeff.pop()
            exponent+=1
        normalized=raw_tuple_display(parts.sign,tuple(coeff),exponent)
    return (s,original,normalized)
def fnv64(text:str)->int:
    h=0xcbf29ce484222325
    for b in text.encode('ascii'):
        h=((h^b)*0x100000001b3)&0xffffffffffffffff
    return h
def expected():
    triples=[row(i) for i in INPUTS]
    lines=['# source expected_display expected_normalized']+[' '.join(row) for row in triples]
    stream=''.join(f'{d}|{n}\n' for _,d,n in triples)
    return '\n'.join(lines)+'\n',f'{fnv64(stream):016x}\n'
def main():
    p=argparse.ArgumentParser()
    p.add_argument('--write',action='store_true')
    args=p.parse_args()
    vectors,fingerprint=expected()
    if args.write:
        VECTORS.write_text(vectors,encoding='ascii')
        FINGERPRINT.write_text(fingerprint,encoding='ascii')
    else:
        assert VECTORS.read_text(encoding='ascii')==vectors,'representation vectors differ'
        assert FINGERPRINT.read_text(encoding='ascii')==fingerprint,'fingerprint differs'
    print('INDEPENDENT_REPRESENTATION_FINGERPRINT_PASS',len(INPUTS),'FNV64='+fingerprint.strip())
if __name__=='__main__':main()
