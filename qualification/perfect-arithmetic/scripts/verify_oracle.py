#!/usr/bin/env python3
"""Independent stdlib arbitrary-precision oracle for Perfect Arithmetic qualification."""

from __future__ import annotations

import math
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
NATURAL = ROOT / "vectors" / "natural.tsv"
INTEGER = ROOT / "vectors" / "integer.tsv"
GCD = ROOT / "vectors" / "gcd.tsv"
FINGERPRINT = ROOT / "semantic-fingerprint.txt"

MASK64 = (1 << 64) - 1
FNV_OFFSET = 0xCBF29CE484222325
FNV_PRIME = 0x00000100000001B3
MIX_MULTIPLIER = 0x9E3779B185EBCA87


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


def feed_fnv(state: int, byte: int) -> int:
    return ((state ^ byte) * FNV_PRIME) & MASK64


def hash_natural(value: int) -> int:
    state = FNV_OFFSET
    for byte in b"perfect-arithmetic:natural:v1\x00":
        state = feed_fnv(state, byte)
    for byte in value.bit_length().to_bytes(8, "little", signed=False):
        state = feed_fnv(state, byte)
    remaining = value
    while remaining:
        limb = remaining & 0xFFFF_FFFF
        for byte in limb.to_bytes(4, "little", signed=False):
            state = feed_fnv(state, byte)
        remaining >>= 32
    return state


def hash_integer(value: int) -> int:
    state = FNV_OFFSET
    for byte in b"perfect-arithmetic:integer:v1\x00":
        state = feed_fnv(state, byte)
    state = feed_fnv(state, int(value < 0))
    magnitude = abs(value)
    for byte in magnitude.bit_length().to_bytes(8, "little", signed=False):
        state = feed_fnv(state, byte)
    while magnitude:
        limb = magnitude & 0xFFFF_FFFF
        for byte in limb.to_bytes(4, "little", signed=False):
            state = feed_fnv(state, byte)
        magnitude >>= 32
    return state


def rotate_left_64(value: int, bits: int) -> int:
    return ((value << bits) & MASK64) | (value >> (64 - bits))


def mix_byte(state: int, value: int) -> int:
    state ^= value + 0x9E
    state = rotate_left_64(state, 13)
    return (state * MIX_MULTIPLIER) & MASK64


def mix_u64(state: int, value: int) -> int:
    for byte in value.to_bytes(8, "little", signed=False):
        state = mix_byte(state, byte)
    return state


def trailing_zeros(value: int) -> int | None:
    if value == 0:
        return None
    return (value & -value).bit_length() - 1


def mix_natural(state: int, value: int) -> int:
    state = mix_u64(state, value.bit_length())
    trailing = trailing_zeros(value)
    if trailing is None:
        state = mix_byte(state, 0)
    else:
        state = mix_byte(state, 1)
        state = mix_u64(state, trailing)

    for probe in (257, 65_537, 1_000_003):
        state = mix_u64(state, value % probe)

    return mix_u64(state, hash_natural(value))


def mix_integer(state: int, value: int) -> int:
    state = mix_byte(state, int(value < 0))
    state = mix_natural(state, abs(value))
    return mix_u64(state, hash_integer(value))


def natural_with_bits(bits: int, low: int) -> int:
    high = 1 << (bits - 1)
    if bits == 1:
        mask = 0
    elif bits <= 64:
        mask = (1 << (bits - 1)) - 1
    else:
        mask = MASK64
    return high + (low & mask)


def semantic_fingerprint() -> str:
    state = 0x243F6A8885A308D3

    state = mix_natural(state, 0)
    state = mix_natural(state, 1)
    state = mix_integer(state, 0)

    for left in range(32):
        for right in range(32):
            state = mix_natural(state, left + right)
            state = mix_natural(state, left * right)
            state = mix_natural(state, math.gcd(left, right))
            state = mix_byte(state, int(left >= right))
            if right != 0:
                quotient, remainder = divmod(left, right)
                state = mix_natural(state, quotient)
                state = mix_natural(state, remainder)

    for left in range(-16, 17):
        for right in range(-16, 17):
            state = mix_integer(state, left + right)
            state = mix_integer(state, left - right)
            state = mix_integer(state, left * right)
            if right != 0:
                quotient, remainder = trunc_divrem(left, right)
                state = mix_integer(state, quotient)
                state = mix_integer(state, remainder)

    boundary_pairs = (
        (2_047, 1_024),
        (2_048, 1_024),
        (8_191, 2_048),
        (8_192, 2_048),
        (16_384, 4_096),
        (32_768, 8_192),
    )
    for index, (left_bits, right_bits) in enumerate(boundary_pairs):
        salt = index
        left = natural_with_bits(left_bits, 0x9E37_79B9_7F4A_7C15 ^ salt)
        right = natural_with_bits(right_bits, 0xC2B2_AE3D_27D4_EB4F ^ salt)
        product = left * right
        quotient, remainder = divmod(left, right)

        for value in (left, right, product, quotient, remainder):
            state = mix_natural(state, value)

        for left_negative in (False, True):
            for right_negative in (False, True):
                signed_left = -left if left_negative else left
                signed_right = -right if right_negative else right
                state = mix_integer(state, signed_left * signed_right)
                quotient, remainder = trunc_divrem(signed_left, signed_right)
                state = mix_integer(state, quotient)
                state = mix_integer(state, remainder)

    return f"{state:016x}"


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
                f"natural row {row} mismatch: actual={actual} "
                f"expected={(expected_r, expected_q_probe)}"
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
                f"integer row {row} mismatch: actual={actual} "
                f"expected={(expected_r, expected_q_probe)}"
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


def verify_fingerprint() -> None:
    expected = semantic_fingerprint()
    retained = FINGERPRINT.read_text(encoding="utf-8").strip()
    if retained != expected:
        raise SystemExit(
            f"qualification fingerprint mismatch: retained={retained} expected={expected}"
        )
    print(f"PASS: independent Arithmetic semantic/hash fingerprint {expected}")


def main() -> int:
    verify_natural()
    verify_integer()
    verify_gcd()
    verify_fingerprint()
    print("PASS: Perfect Arithmetic independent qualification oracle")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
