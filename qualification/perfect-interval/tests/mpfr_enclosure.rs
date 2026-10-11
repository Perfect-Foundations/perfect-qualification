//! External MPFR 4.2.1 directed endpoints; generated from exact rational
//! mathematical extrema, never from Perfect Interval production output.
use perfect_arithmetic::{Integer, Natural};
use perfect_interval::{EnclosureContext, Float, Interval, Precision, RoundingMode};
use perfect_rational::Rational;

const VECTORS: &str = include_str!("../vectors/real_m1_mpfr_20261008.tsv");

fn exact_float(n: i64, exp: i32) -> Float {
    let numerator = Integer::from(n);
    let fraction = if exp >= 0 {
        Rational::new(
            numerator.mul(&Integer::from(
                Natural::one().shl_bits(u64::from(exp.unsigned_abs())),
            )),
            Integer::one(),
        )
        .unwrap()
    } else {
        Rational::new(
            numerator,
            Integer::from(Natural::one().shl_bits(u64::from(exp.unsigned_abs()))),
        )
        .unwrap()
    };
    // Inputs are dyadics whose odd significands have at most 53 bits.
    // The 128-bit test constructor is thus exact and is not the oracle.
    let converted = Float::from_rational_round(
        &fraction,
        Precision::new(128).unwrap(),
        RoundingMode::NearestTiesToEven,
    )
    .unwrap();
    assert_eq!(converted.ordering(), core::cmp::Ordering::Equal);
    converted.into_value()
}

fn interval(lo: (i64, i32), hi: (i64, i32)) -> Interval {
    Interval::new(exact_float(lo.0, lo.1), exact_float(hi.0, hi.1)).unwrap()
}

#[test]
fn mpfr_outward_reference_contains_exact_fraction_result_in_interval_output() {
    let mut passed = 0;
    let mut by_operation = [0_u32; 6];
    for (index, line) in VECTORS.lines().enumerate() {
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let parts: Vec<_> = line.split_ascii_whitespace().collect();
        assert_eq!(parts.len(), 14, "corrupt vector at line {}", index + 1);
        let n = |i: usize| -> i64 { parts[i].parse().unwrap() };
        let exp = |i: usize| -> i32 { parts[i].parse().unwrap() };
        let p: u32 = parts[1].parse().unwrap();
        let a = interval((n(2), exp(3)), (n(4), exp(5)));
        let b = interval((n(6), exp(7)), (n(8), exp(9)));
        let context = EnclosureContext::new(Precision::new(p).unwrap(), 128).unwrap();
        let reference_lo = exact_float(n(10), exp(11));
        let reference_hi = exact_float(n(12), exp(13));
        let result = match parts[0] {
            "add" => {
                by_operation[0] += 1;
                a.add(&b, context)
            }
            "sub" => {
                by_operation[1] += 1;
                a.sub(&b, context)
            }
            "mul" => {
                by_operation[2] += 1;
                a.mul(&b, context)
            }
            "div" => {
                by_operation[3] += 1;
                a.div(&b, context)
            }
            "recip" => {
                by_operation[4] += 1;
                a.reciprocal(context)
            }
            "neg" => {
                by_operation[5] += 1;
                a.neg(context)
            }
            other => panic!("unknown operation: {other}"),
        }
        .unwrap_or_else(|error| panic!("case {} {} failed: {error}", index + 1, parts[0]));
        let (lo, hi) = result
            .endpoints()
            .unwrap_or_else(|| panic!("case {} {} became Empty/Entire", index + 1, parts[0]));
        assert!(
            lo <= &reference_lo,
            "case {} {} lower bound excluded certified exact-rational result at {} bits",
            index + 1,
            parts[0],
            p
        );
        assert!(
            hi >= &reference_hi,
            "case {} {} upper bound excluded certified exact-rational result at {} bits",
            index + 1,
            parts[0],
            p
        );
        passed += 1;
    }
    assert_eq!(passed, 226, "corpus length changed");
    assert!(by_operation.iter().all(|n| *n > 20), "{by_operation:?}");
}

#[test]
fn ball_conversion_preserves_certified_mpfr_bounds() {
    use perfect_interval::Ball;
    let mut covered = 0_usize;
    let mut widened_entire = 0_usize;
    for (line_no, line) in VECTORS.lines().enumerate() {
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let fields: Vec<_> = line.split_ascii_whitespace().collect();
        let n = |i: usize| -> i64 { fields[i].parse().unwrap() };
        let exp = |i: usize| -> i32 { fields[i].parse().unwrap() };
        let bits: u32 = fields[1].parse().unwrap();
        let context = EnclosureContext::new(Precision::new(bits).unwrap(), 128).unwrap();
        let a = interval((n(2), exp(3)), (n(4), exp(5)));
        let b = interval((n(6), exp(7)), (n(8), exp(9)));
        let input_a = Ball::from_interval(&a, context)
            .unwrap_or_else(|error| panic!("line {line_no}: input a conversion {error}"));
        let input_b = Ball::from_interval(&b, context)
            .unwrap_or_else(|error| panic!("line {line_no}: input b conversion {error}"));
        let ia = input_a.to_interval(context).unwrap();
        let ib = input_b.to_interval(context).unwrap();
        for x in [n(2), n(4)] {
            assert!(
                ia.contains(&exact_float(x, exp(if x == n(2) { 3 } else { 5 }))),
                "line {line_no}: a ball excluded original endpoint"
            );
        }
        assert!(ib.contains(&exact_float(n(6), exp(7))));
        assert!(ib.contains(&exact_float(n(8), exp(9))));
        let result = match fields[0] {
            "add" => input_a.add(&input_b, context),
            "sub" => input_a.sub(&input_b, context),
            "mul" => input_a.mul(&input_b, context),
            "div" => input_a.div(&input_b, context),
            "recip" => {
                let inv = ia.reciprocal(context).unwrap();
                Ball::from_interval(&inv, context)
            }
            "neg" => input_a.neg(context),
            other => panic!("unknown op {other}"),
        }
        .unwrap_or_else(|error| panic!("line {line_no} ball op failed: {error}"));
        let enclosure = result.to_interval(context).unwrap();
        if enclosure.is_entire() {
            widened_entire += 1;
        } else {
            let (lo, hi) = enclosure
                .endpoints()
                .unwrap_or_else(|| panic!("line {line_no}: ball result unexpectedly empty"));
            let certified_lo = exact_float(n(10), exp(11));
            let certified_hi = exact_float(n(12), exp(13));
            assert!(
                lo <= &certified_lo,
                "line {line_no}: Ball excluded exact mathematical low"
            );
            assert!(
                hi >= &certified_hi,
                "line {line_no}: Ball excluded exact mathematical high"
            );
        }
        covered += 1;
    }
    assert_eq!(covered, 226);
    println!("MPFR_BALL_CASES={covered} CONSERVATIVE_ENTIRE={widened_entire}");
}
