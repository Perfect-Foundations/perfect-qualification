#!/usr/bin/env python3
"""Exact Fraction adversarial GUM oracle, separately derived from production."""
from fractions import Fraction as F
from hashlib import sha256
from pathlib import Path
import sys
ROOT = Path(__file__).resolve().parent.parent
MATRICES = (
    ((1, 1, 1), (1, 1, 1), (1, 1, 1)),
    ((1, 1, 0), (1, 1, 0), (0, 0, 1)),
    ((1, 0, 1), (0, 1, 0), (1, 0, 1)),
    ((1, -1, 1), (-1, 1, -1), (1, -1, 1)),
)
def exact(matrix, values):
    # Independent diagonal plus paired cross-term GUM formula.
    result = sum((F(values[i]**2) for i in range(3)), F(0))
    for i in range(3):
        for j in range(i+1, 3):
            result += 2 * F(values[i]) * F(matrix[i][j]) * F(values[j])
    return result
def generate():
    rows = ["# matrix_id contribution1 contribution2 contribution3 exact_variance"]
    for scale in (10**9, 10**12, 10**16, 10**18):
        for small in (1, 3, 7, 128):
            for sign in (-1, 1):
                cases = (
                    (0, (scale, -scale, sign*small)),
                    (0, (scale, sign*small, -scale)),
                    (1, (scale, -scale, sign*small)),
                    (2, (scale, sign*small, -scale)),
                    (3, (scale, scale, sign*small)),
                )
                for family, values in cases:
                    result = exact(MATRICES[family], values)
                    assert result == small**2, (family, values, result)
                    rows.append(" ".join(map(str, (family, *values, int(result)))))
    assert len(rows)-1 == 160
    return ("\n".join(rows)+"\n").encode("ascii")
def main():
    expected = generate()
    path = ROOT/"vectors"/"adversarial_gum_v1.tsv"
    if "--generate" in sys.argv:
        if path.exists():
            raise SystemExit("REFUSE_TO_REPLACE_FROZEN_CORPUS")
        path.write_bytes(expected)
    elif path.read_bytes() != expected:
        raise SystemExit("ADVERSARIAL_ORACLE_MISMATCH")
    print("INDEPENDENT_ADVERSARIAL_FRACTION_PASS", 160, sha256(expected).hexdigest())
if __name__ == "__main__":
    main()
