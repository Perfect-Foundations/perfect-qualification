#!/usr/bin/env python3
"""Pure-Python independent packed Hamming/SECDED vectors for all M1 parity sizes."""
from pathlib import Path
from hashlib import sha256
import sys

DEST = Path(__file__).resolve().parent.parent / "vectors" / "hamming_packed_v1.tsv"

def pack(bits, msb):
    result = bytearray((len(bits) + 7) // 8)
    for i, bit in enumerate(bits):
        if bit:
            result[i // 8] |= 1 << (7 - i % 8 if msb else i % 8)
    return result.hex()

def hamming(data, r, extended):
    n = (1 << r) - 1
    word = [0] * n
    it = iter(data)
    for pos in range(1, n + 1):
        if pos & (pos - 1):
            word[pos - 1] = next(it)
    for j in range(r):
        mask = 1 << j
        word[mask - 1] = sum(word[pos - 1]
                             for pos in range(1, n + 1) if pos & mask and pos != mask) % 2
    if extended:
        word.append(sum(word) % 2)
    return word

def render():
    lines = ["# parity_bits extended msb pattern message_bits code_bits message_hex code_hex"]
    for r in range(3, 9):
        length = (1 << r) - 1 - r
        for ext in (0, 1):
            for msb in (0, 1):
                for pattern in (0, 1):
                    msg = [int((i * 7 + 3) % 11 < 5) if pattern == 0
                           else int(i % 3 == 0) for i in range(length)]
                    encoded = hamming(msg, r, ext)
                    lines.append(f"{r} {ext} {msb} {pattern} {len(msg)} "
                                 f"{len(encoded)} {pack(msg,msb)} {pack(encoded,msb)}")
    return ("\n".join(lines) + "\n").encode("ascii"), 48

def main():
    data, count = render()
    if "--generate" in sys.argv:
        if DEST.exists(): raise SystemExit("REFUSE_EXISTING_VECTOR_OVERWRITE")
        DEST.write_bytes(data)
    elif DEST.read_bytes() != data:
        raise SystemExit("HAMMING_PACKED_REFERENCE_MISMATCH")
    print("INDEPENDENT_HAMMING_PACKED_PASS", count, sha256(data).hexdigest())

if __name__ == "__main__": main()
