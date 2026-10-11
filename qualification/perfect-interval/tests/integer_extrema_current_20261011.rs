//! Independent exact integer extrema with coarse destination precision.
//! Expected endpoints are primitive i128, not Float/Interval arithmetic.
use perfect_arithmetic::Integer;
use perfect_interval::{EnclosureContext, Float, Interval, Precision, RoundingMode};
fn prec(bits: u32) -> Precision {
    Precision::new(bits).unwrap()
}
fn exact(n: i128) -> Float {
    Float::from_integer_round(
        &Integer::from(n),
        prec(128),
        RoundingMode::NearestTiesToEven,
    )
    .unwrap()
    .into_value()
}
fn interval(a: i128, b: i128) -> Interval {
    Interval::new(exact(a), exact(b)).unwrap()
}
#[test]
fn public_interval_results_enclose_exact_integer_corner_extrema() {
    let mut state = 0x8691_125b_02c4_441e_u64;
    let mut next = || {
        state = state
            .wrapping_mul(2862933555777941757)
            .wrapping_add(3037000493);
        ((state >> 20) % 200_001) as i128 - 100_000
    };
    for case in 0..320_usize {
        let a = next();
        let b = next();
        let c = next();
        let d = next();
        let (al, au) = (a.min(b), a.max(b));
        let (bl, bu) = (c.min(d), c.max(d));
        let lhs = interval(al, au);
        let rhs = interval(bl, bu);
        let context = EnclosureContext::new(prec(if case % 2 == 0 { 3 } else { 11 }), 128).unwrap();
        let sum = lhs.add(&rhs, context).unwrap();
        let diff = lhs.sub(&rhs, context).unwrap();
        let prod = lhs.mul(&rhs, context).unwrap();
        for val in [al + bl, au + bu] {
            assert!(sum.contains(&exact(val)), "sum {case}");
        }
        for val in [al - bu, au - bl] {
            assert!(diff.contains(&exact(val)), "diff {case}");
        }
        let corners = [al * bl, al * bu, au * bl, au * bu];
        assert!(
            prod.contains(&exact(*corners.iter().min().unwrap())),
            "prod low {case}"
        );
        assert!(
            prod.contains(&exact(*corners.iter().max().unwrap())),
            "prod high {case}"
        );
        let hull = lhs.hull(&rhs);
        for v in [al, au, bl, bu] {
            assert!(hull.contains(&exact(v)), "hull {case}");
        }
        let overlap = lhs.intersection(&rhs);
        if al.max(bl) > au.min(bu) {
            assert!(overlap.is_empty(), "disjoint {case}");
        } else {
            assert!(
                overlap.contains(&exact(al.max(bl))),
                "intersection low {case}"
            );
            assert!(
                overlap.contains(&exact(au.min(bu))),
                "intersection high {case}"
            );
        }
    }
}
