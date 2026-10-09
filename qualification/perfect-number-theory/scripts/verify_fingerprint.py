#!/usr/bin/env python3
"""Independent semantic fingerprint over stdlib-calculated public API vectors."""
from __future__ import annotations
from pathlib import Path
from verify_oracle import modular_rows, crt_rows, primality_rows
ROOT = Path(__file__).resolve().parent.parent
OFFSET = 0xCBF29CE484222325
PRIME = 0x100000001B3
def reference() -> tuple[int, int]:
    state = OFFSET
    count = 0
    for generator in (modular_rows, crt_rows, primality_rows):
        rows = generator()
        for line in rows[1:]:
            for value in (line + "\n").encode("ascii"):
                state = ((state ^ value) * PRIME) & 0xFFFFFFFFFFFFFFFF
            count += 1
    return state, count
def main() -> None:
    state, count = reference()
    assert count == 300 + 189 + 4103, count
    expected = (ROOT / "semantic-fingerprint.txt").read_text(encoding="ascii").strip()
    actual = f"{state:016x}"
    if actual != expected:
        raise SystemExit(f"Fingerprint mismatch: reference {actual}, retained {expected}")
    print(f"INDEPENDENT_NT_FINGERPRINT_PASS {actual} ROWS={count}")
if __name__ == "__main__":
    main()
