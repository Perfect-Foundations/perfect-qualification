#!/usr/bin/env python3
"""Independent Python Decimal oracle for the retained Perfect Decimal Q5 vector corpus."""
from decimal import Decimal, localcontext, ROUND_DOWN, ROUND_FLOOR, ROUND_CEILING, ROUND_HALF_EVEN, ROUND_HALF_UP
from pathlib import Path
import argparse
ROOT=Path(__file__).resolve().parents[1]
VECTORS=ROOT/'vectors'/'quantize.tsv'
VALUES=[
    '0','-0e-2','1','-1','1.25','-1.25','1.35','-1.35',
    '1.50','-1.50','0.045','-0.045','99.95','-99.95',
    '123.456','-123.456','0.0001','-0.0001',
    '1200e-3','-1200e-3','9999e-3','-9999e-3',
]
TARGETS=(-3,-1,0,2)
MODES={'ZERO':ROUND_DOWN,'NEG':ROUND_FLOOR,'POS':ROUND_CEILING,'EVEN':ROUND_HALF_EVEN,'AWAY':ROUND_HALF_UP}
def corpus():
    rows=['# input target mode expected_nonnegative_coefficient relation_to_exact']
    with localcontext() as ctx:
        ctx.prec=100
        for source_text in VALUES:
            source=Decimal(source_text)
            for target in TARGETS:
                quantum=Decimal((0,(1,),target))
                for key,rounding in MODES.items():
                    rounded=source.quantize(quantum,rounding=rounding)
                    coefficient=int(''.join(str(d) for d in rounded.as_tuple().digits))
                    relation='L' if rounded<source else 'G' if rounded>source else 'E'
                    rows.append(f'{source_text} {target} {key} {coefficient} {relation}')
    return '\n'.join(rows)+'\n'
def main():
    p=argparse.ArgumentParser()
    p.add_argument('--write-vectors',action='store_true')
    args=p.parse_args()
    expected=corpus()
    if args.write_vectors:
        VECTORS.write_text(expected,encoding='utf-8')
        print('WROTE',VECTORS,len(expected.splitlines())-1)
    elif VECTORS.read_text(encoding='utf-8')!=expected:
        raise SystemExit('FAIL: retained Python Decimal vectors differ from independently recalculated corpus')
    else:
        print('PASS: independently verified',len(expected.splitlines())-1,'Python Decimal quantum/rounding vectors')
if __name__=='__main__':
    main()
