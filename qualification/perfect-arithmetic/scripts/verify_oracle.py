#!/usr/bin/env python3
"""Independent stdlib arbitrary-precision oracle for Perfect Arithmetic qualification."""

from __future__ import annotations

import math
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
NATURAL = ROOT / "vectors" / "natural.tsv"
INTEGER = ROOT / "vectors" / "integer.tsv"
GCD = ROOT / "vectors" / "gcd.tsv"


def data_lines(path: Path) -> list[str]:
    return [
        line.strip()
        for line in path.read_text(encoding="utf-8").splitlines()
        if line.strip() and not line.startswith("#")
    ]


def trunc_divrem(value: int, divisor: int) -> tuple[int, int]:
    quotient = abs(value) // abs(divisor)
    if (value < 0) ^ (divisor < 0):
        quotient = -quotient
    return quotient, value - quotient * divisor


def verify_natural() -> None:
    checked = 0
    for row, line in enumerate(data_lines(NATURAL), 1):
        fields = [int(value) for value in line.split()]
        if len(fields) != 9:
            raise SystemExit(f"natural row {row} malformed")
        seed, mul, add, inc, steps, divisor, probe, expected_r, expected_q_probe = fields
        value = seed
        for step in range(steps):
            value = value * mul + add + step * inc
        quotient, remainder = divmod(value, divisor)
        actual = (remainder, quotient % probe)
        if actual != (expected_r, expected_q_probe):
            raise SystemExit(
                f"natural row {row} mismatch: actual={actual} expected={(expected_r, expected_q_probe)}"
            )
        checked += 1
    if checked != 24:
        raise SystemExit(f"expected 24 natural rows, got {checked}")
    print("PASS: 24 independent Natural qualification vectors")


def verify_integer() -> None:
    checked = 0
    for row, line in enumerate(data_lines(INTEGER), 1):
        fields = [int(value) for value in line.split()]
        if len(fields) != 9:
            raise SystemExit(f"integer row {row} malformed")
        seed, mul, add, inc, steps, divisor, probe, expected_r, expected_q_probe = fields
        value = seed
        for step in range(steps):
            value = value * mul + add + step * inc
        quotient, remainder = trunc_divrem(value, divisor)
        _, q_probe = trunc_divrem(quotient, probe)
        actual = (remainder, q_probe)
        if actual != (expected_r, expected_q_probe):
            raise SystemExit(
                f"integer row {row} mismatch: actual={actual} expected={(expected_r, expected_q_probe)}"
            )
        checked += 1
    if checked != 24:
        raise SystemExit(f"expected 24 integer rows, got {checked}")
    print("PASS: 24 independent Integer qualification vectors")


def verify_gcd() -> None:
    checked = 0
    for row, line in enumerate(data_lines(GCD), 1):
        fields = [int(value) for value in line.split()]
        if len(fields) != 7:
            raise SystemExit(f"GCD row {row} malformed")
        seed, mul, add, inc, steps, factor, expected = fields
        value = seed
        for step in range(steps):
            value = value * mul + add + step * inc
        actual = math.gcd(value * factor, (value + 1) * factor)
        if actual != expected:
            raise SystemExit(
                f"GCD row {row} mismatch: actual={actual} expected={expected}"
            )
        checked += 1
    if checked != 24:
        raise SystemExit(f"expected 24 GCD rows, got {checked}")
    print("PASS: 24 independent GCD qualification vectors")


def main() -> int:
    verify_natural()
    verify_integer()
    verify_gcd()
    print("PASS: Perfect Arithmetic independent qualification oracle")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
