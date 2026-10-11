//! Newly generated independent exact Fraction extrema/MPFR RNDD-RNDU corpus.
//! Arb reference results are separately certified by verify_arb_output.py.
//! Prior 226-case consumer stays unchanged.
use perfect_arithmetic::{Integer, Natural};
use perfect_interval::{Ball, EnclosureContext, Float, Interval, Precision, RoundingMode};
use perfect_rational::Rational;

const CORPUS: &str = include_str!("../vectors/arb_adversarial_v1.tsv");

fn exact_float(n: i64, exponent: i32, bits: u32) -> Float {
    let numerator = Integer::from(n);
    let shift = u64::from(exponent.unsigned_abs());
    let value = if exponent >= 0 {
        Rational::new(
            numerator.mul(&Integer::from(Natural::one().shl_bits(shift))),
            Integer::one(),
        )
    } else {
        Rational::new(numerator, Integer::from(Natural::one().shl_bits(shift)))
    }
    .unwrap();
    let result = Float::from_rational_round(
        &value,
        Precision::new(bits).unwrap(),
        RoundingMode::NearestTiesToEven,
    )
    .unwrap();
    assert_eq!(result.ordering(), core::cmp::Ordering::Equal);
    result.into_value()
}

fn enclosure(n0: i64, e0: i32, n1: i64, e1: i32) -> Interval {
    // Differing stored precisions are deliberate, while each dyadic is exact.
    Interval::new(exact_float(n0, e0, 64), exact_float(n1, e1, 128)).unwrap()
}
fn parse(line: &str) -> (String, EnclosureContext, Interval, Interval, Float, Float) {
    let c: Vec<_> = line.split_ascii_whitespace().collect();
    assert_eq!(c.len(), 14);
    let op = c[0].to_owned();
    let p: u32 = c[1].parse().unwrap();
    let n = |i: usize| -> i64 { c[i].parse().unwrap() };
    let e = |i: usize| -> i32 { c[i].parse().unwrap() };
    (
        op,
        EnclosureContext::new(Precision::new(p).unwrap(), 128).unwrap(),
        enclosure(n(2), e(3), n(4), e(5)),
        enclosure(n(6), e(7), n(8), e(9)),
        exact_float(n(10), e(11), 128),
        exact_float(n(12), e(13), 128),
    )
}
#[test]
fn new_adversarial_exact_rational_mpfr_interval_enclosures() {
    let mut seen = 0_usize;
    let mut conservative_entire = 0_usize;
    for (line_no, line) in CORPUS.lines().enumerate() {
        if line.starts_with('#') || line.is_empty() {
            continue;
        }
        let (op, c, a, b, lo, hi) = parse(line);
        let result = match op.as_str() {
            "add" => a.add(&b, c),
            "sub" => a.sub(&b, c),
            "mul" => a.mul(&b, c),
            "div" => a.div(&b, c),
            "recip" => a.reciprocal(c),
            "neg" => a.neg(c),
            _ => unreachable!(),
        }
        .unwrap_or_else(|e| panic!("adversarial line {} {}: {e}", line_no + 1, op));
        if result.is_entire() {
            conservative_entire += 1;
        } else {
            let (bottom, top) = result
                .endpoints()
                .unwrap_or_else(|| panic!("unexpected Empty case {} {}", line_no + 1, op));
            assert!(bottom <= &lo, "LOW EXCLUDED case {} {op}", line_no + 1);
            assert!(top >= &hi, "HIGH EXCLUDED case {} {op}", line_no + 1);
        }
        seen += 1;
    }
    assert_eq!(seen, 330);
    println!("ADVERSARIAL_INTERVAL_330_CASES={seen} CONSERVATIVE_ENTIRE={conservative_entire}");
}
#[test]
fn adversarial_ball_operations_and_three_conversion_round_trips() {
    let mut seen = 0_usize;
    let mut widening_to_entire = 0_usize;
    let mut widening_width_increases = 0_usize;
    for (line_no, line) in CORPUS.lines().enumerate() {
        if line.starts_with('#') || line.is_empty() {
            continue;
        }
        let (op, c, a, b, lo, hi) = parse(line);
        let ba = Ball::from_interval(&a, c).unwrap();
        let bb = Ball::from_interval(&b, c).unwrap();
        let ai = ba.to_interval(c).unwrap();
        let bi = bb.to_interval(c).unwrap();
        for source in [&a, &b] {
            let converted = if core::ptr::eq(source, &a) { &ai } else { &bi };
            if let Some((lower, upper)) = source.endpoints() {
                assert!(
                    converted.contains(lower),
                    "roundtrip lost lower, case {}",
                    line_no + 1
                );
                assert!(
                    converted.contains(upper),
                    "roundtrip lost upper, case {}",
                    line_no + 1
                );
            }
        }
        let result = match op.as_str() {
            "add" => ba.add(&bb, c),
            "sub" => ba.sub(&bb, c),
            "mul" => ba.mul(&bb, c),
            "div" => ba.div(&bb, c),
            "neg" => ba.neg(c),
            "recip" => Ball::from_interval(&ai.reciprocal(c).unwrap(), c),
            _ => unreachable!(),
        }
        .unwrap_or_else(|e| panic!("Ball adversarial {} {op}: {e}", line_no + 1));
        let out = result.to_interval(c).unwrap();
        if out.is_entire() {
            widening_to_entire += 1
        } else {
            let (bottom, top) = out
                .endpoints()
                .unwrap_or_else(|| panic!("unexpected Ball Empty {} {op}", line_no + 1));
            assert!(bottom <= &lo, "Ball LOW excluded {} {op}", line_no + 1);
            assert!(top >= &hi, "Ball HIGH excluded {} {op}", line_no + 1);
        }
        let mut enclosure = ai;
        for _ in 0..3 {
            let next = Ball::from_interval(&enclosure, c)
                .unwrap()
                .to_interval(c)
                .unwrap();
            if let Some((lower, upper)) = enclosure.endpoints() {
                assert!(
                    next.contains(lower),
                    "three-roundtrip lower excluded {}",
                    line_no + 1
                );
                assert!(
                    next.contains(upper),
                    "three-roundtrip upper excluded {}",
                    line_no + 1
                );
            }
            let previous_width = enclosure.width(c).unwrap();
            let next_width = next.width(c).unwrap();
            if let (Some(old_width), Some(new_width)) = (previous_width, next_width) {
                assert!(
                    new_width >= old_width,
                    "roundtrip width decreased case {}",
                    line_no + 1
                );
                if new_width > old_width {
                    widening_width_increases += 1;
                }
            }
            enclosure = next;
        }
        seen += 1;
    }
    assert_eq!(seen, 330);
    println!(
        "ADVERSARIAL_BALL_330_CASES={seen} CONSERVATIVE_ENTIRE={widening_to_entire} WIDTH_INCREASE_STEPS={widening_width_increases}"
    );
}
