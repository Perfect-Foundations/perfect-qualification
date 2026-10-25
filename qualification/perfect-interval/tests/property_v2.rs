//! Seeded independent Fraction/MPFR reference plus metamorphic set properties.
//! The previous 226 and 330-case baselines are never modified.
use perfect_arithmetic::{Integer, Natural};
use perfect_interval::{Ball, EnclosureContext, Float, Interval, Precision, RoundingMode};
use perfect_rational::Rational;

const CASES: &str = include_str!("../vectors/property_v2.tsv");
fn val(n: i64, e: i32, bits: u32) -> Float {
    let num = Integer::from(n);
    let sh = u64::from(e.unsigned_abs());
    let rat = if e >= 0 {
        Rational::new(
            num.mul(&Integer::from(Natural::one().shl_bits(sh))),
            Integer::one(),
        )
    } else {
        Rational::new(num, Integer::from(Natural::one().shl_bits(sh)))
    }
    .unwrap();
    let f = Float::from_rational_round(
        &rat,
        Precision::new(bits).unwrap(),
        RoundingMode::NearestTiesToEven,
    )
    .unwrap();
    assert_eq!(f.ordering(), core::cmp::Ordering::Equal);
    f.into_value()
}
fn iv(n: i64, e: i32, m: i64, f: i32, index: usize) -> Interval {
    // Each dyadic operand is exactly representable at its chosen precision.
    Interval::new(
        val(n, e, if index.is_multiple_of(2) { 32 } else { 64 }),
        val(m, f, if index.is_multiple_of(3) { 48 } else { 64 }),
    )
    .unwrap()
}
struct Case {
    high_lo: Float,
    high_hi: Float,
    op: String,
    p: u32,
    a: Interval,
    b: Interval,
    lo: Float,
    hi: Float,
}
fn case(line: &str, k: usize) -> Case {
    let c: Vec<_> = line.split_ascii_whitespace().collect();
    assert_eq!(c.len(), 18);
    let i = |x: usize| -> i64 { c[x].parse().unwrap() };
    let e = |x: usize| -> i32 { c[x].parse().unwrap() };
    Case {
        op: c[0].to_string(),
        p: c[1].parse().unwrap(),
        a: iv(i(2), e(3), i(4), e(5), k),
        b: iv(i(6), e(7), i(8), e(9), k + 1),
        lo: val(i(10), e(11), 128),
        hi: val(i(12), e(13), 128),
        high_lo: val(i(14), e(15), 128),
        high_hi: val(i(16), e(17), 128),
    }
}
fn context(p: u32) -> EnclosureContext {
    EnclosureContext::new(Precision::new(p).unwrap(), 128).unwrap()
}
fn operation(x: &Interval, y: &Interval, op: &str, c: EnclosureContext) -> Interval {
    match op {
        "add" => x.add(y, c),
        "sub" => x.sub(y, c),
        "mul" => x.mul(y, c),
        "div" => x.div(y, c),
        "recip" => x.reciprocal(c),
        "neg" => x.neg(c),
        _ => unreachable!(),
    }
    .unwrap()
}
fn enclosed(x: &Interval, lower: &Float, upper: &Float, k: usize, label: &str) {
    assert!(
        !x.is_empty(),
        "EMPTY EXCLUDES FINITE MATHEMATICAL SET case {k} {label}"
    );
    if let Some((lo, hi)) = x.endpoints() {
        assert!(!lo.is_nan() && !hi.is_nan(), "NaN bound case {k} {label}");
        assert!(lo <= lower, "LOWER EXCLUSION case {k} {label}");
        assert!(hi >= upper, "UPPER EXCLUSION case {k} {label}");
    }
}
#[test]
fn seeded_fraction_mpfr_576_strict_soundness_and_metamorphic_properties() {
    let mut seen = 0usize;
    let mut entire = 0usize;
    for (k, line) in CASES
        .lines()
        .filter(|l| !l.starts_with('#') && !l.is_empty())
        .enumerate()
    {
        let t = case(line, k);
        let ctx = context(t.p);
        let out = operation(&t.a, &t.b, &t.op, ctx);
        enclosed(&out, &t.lo, &t.hi, k, &t.op);
        if out.is_entire() {
            entire += 1;
        }
        let neg = t.a.neg(ctx).unwrap();
        let double = neg.neg(ctx).unwrap();
        // Mathematically exact double negation of the original *stored* bounds.
        if let Some((low, high)) = t.a.endpoints() {
            assert!(
                double.contains(low) && double.contains(high),
                "double-negation lost input case {k}"
            );
        }
        let hull = t.a.hull(&t.b);
        for original in [&t.a, &t.b] {
            if let Some((low, high)) = original.endpoints() {
                assert!(
                    hull.contains(low) && hull.contains(high),
                    "hull lost an endpoint case {k}"
                );
            }
        }
        let intersection = t.a.intersection(&t.b);
        if let Some((lo, hi)) = intersection.endpoints() {
            assert!(t.a.contains(lo) && t.a.contains(hi));
            assert!(t.b.contains(lo) && t.b.contains(hi));
        }
        if t.op == "add" || t.op == "mul" {
            let widened = operation(&hull, &t.b, &t.op, ctx);
            enclosed(&widened, &t.lo, &t.hi, k, "widened-input soundness");
        }
        let higher = context((t.p + 11).min(53));
        let refined = operation(&t.a, &t.b, &t.op, higher);
        enclosed(
            &refined,
            &t.high_lo,
            &t.high_hi,
            k,
            "higher-precision soundness",
        );
        // Exactly representable mathematical zeros must remain members after
        // this widening or repeated outward conversion.
        seen += 1;
    }
    assert_eq!(seen, 576);
    println!("PROPERTY_V2_576_PASS={seen} ENTIRE={entire}");
}
#[test]
fn bounded_twelve_round_ball_containment_and_width_monotonicity() {
    let mut selected = 0usize;
    let mut widen = 0usize;
    let mut entire = 0usize;
    for (k, line) in CASES
        .lines()
        .filter(|l| !l.starts_with('#') && !l.is_empty())
        .enumerate()
    {
        if k % 4 != 0 {
            continue;
        }
        let t = case(line, k);
        let ctx = context(t.p);
        let mut current = t.a;
        for step in 0..12 {
            let next = Ball::from_interval(&current, ctx)
                .unwrap()
                .to_interval(ctx)
                .unwrap();
            if let Some((lo, hi)) = current.endpoints() {
                assert!(
                    next.contains(lo) && next.contains(hi),
                    "Ball 12-round exclusion case {k} step {step}"
                );
            }
            if let (Some(a), Some(b)) = (current.width(ctx).unwrap(), next.width(ctx).unwrap()) {
                assert!(b >= a, "Width decreased case {k} step {step}");
                if b > a {
                    widen += 1;
                }
            }
            if next.is_entire() {
                entire += 1;
            }
            current = next;
        }
        selected += 1;
    }
    assert_eq!(selected, 144);
    println!(
        "BALL_12_ROUND_CASES={selected} WIDTH_INCREASE_STEPS={widen} ENTIRE_TRANSITIONS_OR_STEPS={entire}"
    );
}
