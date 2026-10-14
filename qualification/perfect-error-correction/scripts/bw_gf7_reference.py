#!/usr/bin/env python3
"""Independent GF(7) evaluation-code BW correction vectors, no Rust oracle."""
from pathlib import Path
from hashlib import sha256
import sys

ROOT = Path(__file__).resolve().parent.parent
DEST = ROOT / "vectors" / "rs_bw_gf7_v1.tsv"

def render():
    messages = [(0, 0, 0), (0, 1, 0), (1, 2, 3), (6, 0, 6)]
    # Unknown errors, known erasures, and their independent GF(7) additives.
    patterns = [
        ((), (), {}),
        ((0,), (), {0: 1}),
        ((5,), (2,), {5: 2, 2: 3}),
        ((), (0, 3, 5), {0: 1, 3: 2, 5: 3}),
    ]
    lines = ["# GF7 n=6 k=3: coeff(3) codeword(6) received(6) erasure_mask(6) corrected_mask(6)"]
    for coeff in messages:
        expected = [sum(c * pow(x, j, 7) for j, c in enumerate(coeff)) % 7
                    for x in range(6)]
        for errors, erasures, additives in patterns:
            assert 2 * len(errors) + len(erasures) <= 3
            assert len(set(errors) | set(erasures)) == len(errors) + len(erasures)
            received = expected[:]
            for position, delta in additives.items():
                received[position] = (received[position] + delta) % 7
            erased = [int(i in erasures) for i in range(6)]
            corrected = [int(i in errors or i in erasures) for i in range(6)]
            row = list(coeff) + expected + received + erased + corrected
            assert len(row) == 27
            lines.append(" ".join(str(value) for value in row))
    return ("\n".join(lines) + "\n").encode("ascii"), len(messages)*len(patterns)

def main():
    data, count = render()
    if "--generate" in sys.argv:
        if DEST.exists():
            raise SystemExit("REFUSE_EXISTING_VECTOR_OVERWRITE")
        DEST.write_bytes(data)
    elif DEST.read_bytes() != data:
        raise SystemExit("BW_GF7_VECTOR_CHANGED")
    print("BW_INDEPENDENT_GF7_PASS", count, sha256(data).hexdigest())

if __name__ == "__main__":
    main()
