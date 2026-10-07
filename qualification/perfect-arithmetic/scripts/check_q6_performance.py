#!/usr/bin/env python3
"""Fail-closed Q6 performance/resource regression check for Perfect Arithmetic.

The checker compares the exact candidate against direct num-bigint in the same
benchmark process and compares the candidate's opt-in high-throughput profile
against its own default profile. Absolute historical nanoseconds are deliberately
not used because host timings are environment-specific.
"""

from __future__ import annotations

import argparse
import csv
import math
from pathlib import Path
import sys

PERFECT = "perfect-arithmetic"
DIRECT = "num-bigint-0.5.1-best"
QUICK_BITS = (256, 2_048, 16_384)
OPS = ("add", "mul", "div_rem")
MAX_DEFAULT_OVER_DIRECT = 1.75
MAX_HIGH_THROUGHPUT_OVER_DEFAULT = 0.95


def blocker(message: str) -> None:
    print(f"Q6 PERFORMANCE BLOCKER: {message}", file=sys.stderr)
    raise SystemExit(1)


def fail(message: str) -> None:
    print(f"Q6 PERFORMANCE FAIL: {message}", file=sys.stderr)
    raise SystemExit(1)


def read_rows(path: Path) -> list[dict[str, str]]:
    try:
        with path.open(newline="", encoding="utf-8") as handle:
            return list(csv.DictReader(handle))
    except OSError as error:
        blocker(f"cannot read {path}: {error}")


def timing_index(path: Path) -> dict[tuple[str, str, int], float]:
    rows = read_rows(path)
    result: dict[tuple[str, str, int], float] = {}
    for row in rows:
        try:
            backend = row["backend"]
            operation = row["operation"]
            bits = int(row["target_bits"])
            value = float(row["median_ns_per_op"])
        except (KeyError, TypeError, ValueError) as error:
            blocker(f"malformed timing row in {path}: {error}")
        if not math.isfinite(value) or value <= 0:
            blocker(f"non-positive/non-finite timing in {path}: {row}")
        result[(backend, operation, bits)] = value
    return result


def allocation_index(path: Path) -> dict[tuple[str, str, int], tuple[float, float, float]]:
    rows = read_rows(path)
    result: dict[tuple[str, str, int], tuple[float, float, float]] = {}
    for row in rows:
        try:
            key = (row["backend"], row["operation"], int(row["target_bits"]))
            values = (
                float(row["allocs_per_op"]),
                float(row["deallocs_per_op"]),
                float(row["allocated_bytes_per_op"]),
            )
        except (KeyError, TypeError, ValueError) as error:
            blocker(f"malformed allocation row in {path}: {error}")
        if any(not math.isfinite(value) or value < 0 for value in values):
            blocker(f"invalid allocation measurement in {path}: {row}")
        result[key] = values
    return result


def required(index: dict, key: tuple[str, str, int], source: Path):
    if key not in index:
        blocker(f"missing required row {key!r} in {source}")
    return index[key]


def verify_default_timing(path: Path, index: dict[tuple[str, str, int], float]) -> None:
    for bits in QUICK_BITS:
        for operation in OPS:
            perfect = required(index, (PERFECT, operation, bits), path)
            direct = required(index, (DIRECT, operation, bits), path)
            ratio = perfect / direct
            if ratio > MAX_DEFAULT_OVER_DIRECT:
                fail(
                    f"default {operation} at {bits} bits regressed to "
                    f"{ratio:.3f}x direct num-bigint "
                    f"(limit {MAX_DEFAULT_OVER_DIRECT:.2f}x)"
                )


def verify_default_allocations(
    path: Path,
    index: dict[tuple[str, str, int], tuple[float, float, float]],
) -> None:
    for bits in QUICK_BITS:
        for operation in OPS:
            perfect = required(index, (PERFECT, operation, bits), path)
            direct = required(index, (DIRECT, operation, bits), path)
            if perfect != direct:
                fail(
                    f"default resource profile differs from direct num-bigint for "
                    f"{operation} at {bits} bits: Perfect={perfect}, direct={direct}"
                )


def verify_high_throughput(
    default_timing_path: Path,
    default_timing: dict[tuple[str, str, int], float],
    ht_timing_path: Path,
    ht_timing: dict[tuple[str, str, int], float],
    default_alloc_path: Path,
    default_alloc: dict[tuple[str, str, int], tuple[float, float, float]],
    ht_alloc_path: Path,
    ht_alloc: dict[tuple[str, str, int], tuple[float, float, float]],
) -> None:
    # Below all dispatch thresholds, the opt-in feature must not change resource
    # behavior for the public operations.
    for bits in (256, 2_048):
        for operation in OPS:
            base = required(default_alloc, (PERFECT, operation, bits), default_alloc_path)
            accelerated = required(ht_alloc, (PERFECT, operation, bits), ht_alloc_path)
            if accelerated != base:
                fail(
                    f"high-throughput changed below-threshold {operation} resources "
                    f"at {bits} bits: default={base}, high-throughput={accelerated}"
                )

    # At 16K, benchmark operands cross both retained acceleration thresholds
    # (mul: 8K/2K; div/rem: 16K/4K with an 8K divisor).
    for operation in ("mul", "div_rem"):
        base_time = required(
            default_timing, (PERFECT, operation, 16_384), default_timing_path
        )
        ht_time = required(ht_timing, (PERFECT, operation, 16_384), ht_timing_path)
        ratio = ht_time / base_time
        if ratio > MAX_HIGH_THROUGHPUT_OVER_DEFAULT:
            fail(
                f"high-throughput {operation} at 16384 bits is not a retained "
                f"measured win: {ratio:.3f}x default "
                f"(limit {MAX_HIGH_THROUGHPUT_OVER_DEFAULT:.2f}x)"
            )

        base_alloc = required(
            default_alloc, (PERFECT, operation, 16_384), default_alloc_path
        )
        ht_alloc_value = required(
            ht_alloc, (PERFECT, operation, 16_384), ht_alloc_path
        )
        if not (
            ht_alloc_value[0] < base_alloc[0]
            and ht_alloc_value[2] < base_alloc[2]
        ):
            fail(
                f"high-throughput {operation} at 16384 bits does not reduce both "
                f"allocation count and bytes: default={base_alloc}, "
                f"high-throughput={ht_alloc_value}"
            )


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--default-timing", type=Path, required=True)
    parser.add_argument("--default-alloc", type=Path, required=True)
    parser.add_argument("--high-throughput-timing", type=Path, required=True)
    parser.add_argument("--high-throughput-alloc", type=Path, required=True)
    args = parser.parse_args()

    default_timing = timing_index(args.default_timing)
    default_alloc = allocation_index(args.default_alloc)
    ht_timing = timing_index(args.high_throughput_timing)
    ht_alloc = allocation_index(args.high_throughput_alloc)

    verify_default_timing(args.default_timing, default_timing)
    verify_default_allocations(args.default_alloc, default_alloc)
    verify_high_throughput(
        args.default_timing,
        default_timing,
        args.high_throughput_timing,
        ht_timing,
        args.default_alloc,
        default_alloc,
        args.high_throughput_alloc,
        ht_alloc,
    )

    print("Q6 PERFORMANCE PASS: candidate-bound timing/resource regression gates passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
