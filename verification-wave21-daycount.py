#!/usr/bin/env python3
"""Independently recompute Wave 21 exact day-count vectors; stdlib only."""
from datetime import date, timedelta
from fractions import Fraction
from hashlib import sha256
from pathlib import Path
import re

root = Path(__file__).resolve().parent
source = (root / "qualification" / "perfect-finance" / "tests" /
          "wave21_daycount_independent.rs").read_text(encoding="utf-8")
pattern = re.compile(
    r"\(\[(\d+),\s*(\d+),\s*(\d+)\],\s*"
    r"\[(\d+),\s*(\d+),\s*(\d+)\],\s*"
    r"(-?\d+),\s*(-?\d+),\s*(-?\d+)\)"
)
rows = pattern.findall(source)
if len(rows) != 24:
    raise SystemExit(f"REFERENCE_ROW_COUNT_MISMATCH: {len(rows)}")

def february_end(day):
    return day.month == 2 and (day + timedelta(days=1)).month == 3

fingerprints = []
for raw in rows:
    numbers = [int(v) for v in raw]
    start, end = date(*numbers[:3]), date(*numbers[3:6])
    actual = (end - start).days
    a, b = start.day, end.day
    first_feb = february_end(start)
    us_first = 30 if a == 31 or first_feb else a
    us_second = 30 if (february_end(end) and first_feb) or (
        b == 31 and us_first >= 30) else b
    common = 360 * (end.year - start.year) + 30 * (end.month - start.month)
    us = common + us_second - us_first
    european = common + min(30, b) - min(30, a)
    if (actual, us, european) != tuple(numbers[6:]):
        raise SystemExit(f"DAYCOUNT_ORACLE_MISMATCH: {start} {end} "
                         f"expected={numbers[6:]} computed={(actual, us, european)}")
    results = [Fraction(actual, 360), Fraction(actual, 365),
               Fraction(us, 360), Fraction(european, 360)]
    fingerprints.append(
        f"{start.isoformat()}|{end.isoformat()}|" +
        "|".join(f"{f.numerator}/{f.denominator}" for f in results)
    )
digest = sha256(("\n".join(fingerprints) + "\n").encode("ascii")).hexdigest()
print("WAVE21_FINANCE_DAYCOUNT_ORACLE_PASS", len(rows),
      len(rows) * 4, digest)
