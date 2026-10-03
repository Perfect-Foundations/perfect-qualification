#!/usr/bin/env python3
"""Independent stdlib oracle for the Perfect Numeric qualification bundle."""

from __future__ import annotations

from decimal import (
    Decimal,
    ROUND_CEILING,
    ROUND_DOWN,
    ROUND_FLOOR,
    ROUND_HALF_EVEN,
    ROUND_HALF_UP,
)
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[1]
ROUNDING = ROOT / "vectors" / "rounding.tsv"
INTEGER = ROOT / "vectors" / "integer-summary.txt"

MODES = (
    ROUND_FLOOR,
    ROUND_CEILING,
    ROUND_DOWN,
    ROUND_HALF_EVEN,
    ROUND_HALF_UP,
)


def data_lines(path: Path) -> list[str]:
    return [
        line.strip()
        for line in path.read_text(encoding="utf-8").splitlines()
        if line.strip() and not line.startswith("#")
    ]


def verify_rounding() -> None:
    checked = 0
    for row, line in enumerate(data_lines(ROUNDING), 1):
        fields = line.split()
        if len(fields) != 6:
            raise SystemExit(f"rounding row {row} has {len(fields)} fields")
        value = Decimal(fields[0])
        expected = [Decimal(x) for x in fields[1:]]
        for mode, want in zip(MODES, expected, strict=True):
            actual = value.quantize(Decimal(1), rounding=mode)
            if actual != want:
                raise SystemExit(
                    f"rounding mismatch row={row} value={value} actual={actual} expected={want}"
                )
            checked += 1
    if checked != 60:
        raise SystemExit(f"expected 60 rounding observations, got {checked}")
    print(f"PASS: {checked} independent Decimal rounding observations")


def classify_u16(value: int) -> str:
    return "ok" if value <= 255 else "above"


def classify_i16(value: int) -> str:
    if value < 0:
        return "below"
    if value > 255:
        return "above"
    return "ok"


def verify_integer_summary() -> None:
    retained = {}
    for line in data_lines(INTEGER):
        name, ok, below, above = line.split()
        retained[name] = (int(ok), int(below), int(above))

    u = {"ok": 0, "below": 0, "above": 0}
    for value in range(1 << 16):
        u[classify_u16(value)] += 1

    i = {"ok": 0, "below": 0, "above": 0}
    for value in range(-(1 << 15), 1 << 15):
        i[classify_i16(value)] += 1

    actual = {
        "u16_to_u8": (u["ok"], u["below"], u["above"]),
        "i16_to_u8": (i["ok"], i["below"], i["above"]),
    }
    if actual != retained:
        raise SystemExit(f"integer summary mismatch: actual={actual} retained={retained}")
    print("PASS: exhaustive independent integer classification summaries")


def main() -> int:
    verify_rounding()
    verify_integer_summary()
    print("PASS: Perfect Numeric independent qualification oracle")
    return 0


if __name__ == "__main__":
    sys.exit(main())
