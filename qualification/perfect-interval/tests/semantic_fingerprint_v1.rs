//! PF-SEMANTIC-FINGERPRINT-V1.
//! Canonical dyadic outputs recovered using public exact comparisons only;
//! NEVER hashes Rust Debug, architecture byte order, or map iteration.
use perfect_arithmetic::{Integer, Natural};
use perfect_interval::{Ball, EnclosureContext, Float, Interval, Precision, RoundingMode};
use perfect_rational::Rational;

const DATA: &str = include_str!("../vectors/property_v2.tsv");
const OFFSET: u64 = 0xcbf29ce484222325;
const PRIME: u64 = 0x100000001b3;
fn rational(n: i64, e: i32) -> Rational {
    let num = Integer::from(n);
    let shift = u64::from(e.unsigned_abs());
    if e >= 0 {
        Rational::new(
            num.mul(&Integer::from(Natural::one().shl_bits(shift))),
            Integer::one(),
        )
        .unwrap()
    } else {
        Rational::new(num, Integer::from(Natural::one().shl_bits(shift))).unwrap()
    }
}
fn dyadic(n: i64, e: i32, bits: u32) -> Float {
    let x = Float::from_rational_round(
        &rational(n, e),
        Precision::new(bits).unwrap(),
        RoundingMode::NearestTiesToEven,
    )
    .unwrap();
    assert_eq!(x.ordering(), core::cmp::Ordering::Equal);
    x.into_value()
}
fn canonical(f: &Float) -> String {
    let p = f.precision().bits();
    let sign = if f.is_negative() { "-" } else { "+" };
    if f.is_nan() {
        return format!("N:{p}");
    }
    if f.is_infinite() {
        return format!("I:{sign}:{p}");
    }
    if f.is_zero() {
        return format!("Z:{sign}:{p}");
    }
    assert!(p <= 53, "canonical significand supports <=53 result bits");
    let abs = if f.is_negative() {
        Float::zero(f.precision(), false)
            .sub_round(f, f.precision(), RoundingMode::NearestTiesToEven)
            .unwrap()
            .into_value()
    } else {
        f.clone()
    };
    // Binary-search floor(log2(abs)). All corpus outputs are bounded to +/-8192.
    let (mut bottom, mut top) = (-8192i32, 8192i32);
    assert!(
        abs >= dyadic(1, bottom, 128) && abs < dyadic(1, top, 128),
        "unrepresentable exponent range"
    );
    while top - bottom > 1 {
        let mid = bottom + (top - bottom) / 2;
        if abs >= dyadic(1, mid, 128) {
            bottom = mid;
        } else {
            top = mid;
        }
    }
    let scale = bottom - (p as i32 - 1);
    let mut low = 1_u64 << (p - 1);
    let mut high = (1_u64 << p) - 1;
    while low < high {
        let mid = low + (high - low) / 2;
        if dyadic(mid as i64, scale, 128) < abs {
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    assert_eq!(
        dyadic(low as i64, scale, 128),
        abs,
        "numeric fingerprint cannot reconstruct endpoint"
    );
    format!("F:{sign}:{p}:{low}:{scale}")
}
fn interval_key(v: &Interval) -> String {
    if v.is_empty() {
        return "E".to_owned();
    }
    if v.is_entire() {
        return "A".to_owned();
    }
    let (lo, hi) = v.endpoints().unwrap();
    format!("B:{}:{}", canonical(lo), canonical(hi))
}
fn ball_key(v: &Ball) -> String {
    if v.is_empty() {
        return "E".to_owned();
    }
    if v.is_entire() {
        return "A".to_owned();
    }
    let (mid, r) = v.components().unwrap();
    format!("B:{}:{}", canonical(mid), canonical(r))
}
fn hash(data: &[u8]) -> u64 {
    let mut h = OFFSET;
    for b in data {
        h = (h ^ u64::from(*b)).wrapping_mul(PRIME);
    }
    h
}
#[test]
fn fingerprint_is_canonical_and_math_checked() {
    let mut records = String::from("PF-SEMANTIC-FINGERPRINT-V1\n");
    let mut count = 0usize;
    for (idx, line) in DATA
        .lines()
        .filter(|l| !l.starts_with('#') && !l.is_empty())
        .take(64)
        .enumerate()
    {
        let v: Vec<_> = line.split_ascii_whitespace().collect();
        assert_eq!(v.len(), 18);
        let n = |i: usize| -> i64 { v[i].parse().unwrap() };
        let e = |i: usize| -> i32 { v[i].parse().unwrap() };
        let p: u32 = v[1].parse().unwrap();
        let c = EnclosureContext::new(Precision::new(p).unwrap(), 128).unwrap();
        let a = Interval::new(dyadic(n(2), e(3), 64), dyadic(n(4), e(5), 64)).unwrap();
        let b = Interval::new(dyadic(n(6), e(7), 64), dyadic(n(8), e(9), 64)).unwrap();
        let reference_lo = dyadic(n(10), e(11), 128);
        let reference_hi = dyadic(n(12), e(13), 128);
        let result = match v[0] {
            "add" => a.add(&b, c),
            "sub" => a.sub(&b, c),
            "mul" => a.mul(&b, c),
            "div" => a.div(&b, c),
            "recip" => a.reciprocal(c),
            "neg" => a.neg(c),
            _ => unreachable!(),
        }
        .unwrap();
        if let Some((lo, hi)) = result.endpoints() {
            assert!(
                lo <= &reference_lo && hi >= &reference_hi,
                "fingerprint source mathematically unsound case {idx}"
            );
        } else {
            assert!(
                result.is_entire(),
                "fingerprint source must enclose true result case {idx}"
            );
        }
        let aa = Ball::from_interval(&a, c).unwrap();
        let bb = Ball::from_interval(&b, c).unwrap();
        let br = match v[0] {
            "add" => aa.add(&bb, c),
            "sub" => aa.sub(&bb, c),
            "mul" => aa.mul(&bb, c),
            "div" => aa.div(&bb, c),
            "neg" => aa.neg(c),
            "recip" => Ball::from_interval(&aa.to_interval(c).unwrap().reciprocal(c).unwrap(), c),
            _ => unreachable!(),
        }
        .unwrap();
        let rounded = br.to_interval(c).unwrap();
        if let Some((lo, hi)) = rounded.endpoints() {
            assert!(
                lo <= &reference_lo && hi >= &reference_hi,
                "fingerprint Ball mathematically unsound case {idx}"
            );
        } else {
            assert!(rounded.is_entire());
        }
        records.push_str(&format!(
            "{idx:04}|{}|p={p}|I={}|B={}|B_INTERVAL={}\n",
            v[0],
            interval_key(&result),
            ball_key(&br),
            interval_key(&rounded)
        ));
        count += 1;
    }
    assert_eq!(count, 64);
    let fingerprint = hash(records.as_bytes());
    println!(
        "PF-SEM-V1-FNV64={fingerprint:016x} CASES={count} BYTES={}",
        records.len()
    );
    // Stable transcript available to runners using grep PF-SEM-RECORD, but
    // digest itself is computed from the exact canonical text bytes above.
    for line in records.lines() {
        println!("PF-SEM-RECORD {line}");
    }
}
