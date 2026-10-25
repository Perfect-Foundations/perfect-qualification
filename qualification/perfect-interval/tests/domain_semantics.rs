//! Independently motivated bare-set and signed-zero regression fixtures.
//! These are mathematical structural checks; no MPFR/Arb special-value
//! conventions are imported into Perfect Interval's documented contract.
use perfect_arithmetic::Integer;
use perfect_interval::{
    Ball, EnclosureContext, EnclosureError, Float, Interval, Precision, RoundingMode,
};

fn p() -> Precision {
    Precision::new(8).unwrap()
}
fn c() -> EnclosureContext {
    EnclosureContext::new(p(), 128).unwrap()
}
fn f(n: i64) -> Float {
    Float::from_integer_round(&Integer::from(n), p(), RoundingMode::NearestTiesToEven)
        .unwrap()
        .into_value()
}
fn i(a: i64, b: i64) -> Interval {
    Interval::new(f(a), f(b)).unwrap()
}
#[test]
fn signed_zero_is_mathematical_zero_with_canonical_interval_endpoints() {
    let plus = Float::zero(p(), false);
    let minus = Float::zero(p(), true);
    let point = Interval::new(plus.clone(), minus.clone()).unwrap();
    let (lo, hi) = point.endpoints().unwrap();
    assert!(lo.is_negative());
    assert!(!hi.is_negative());
    assert!(point.contains(&plus) && point.contains(&minus));
    for result in [
        point.add(&point, c()).unwrap(),
        point.sub(&point, c()).unwrap(),
        point.mul(&point, c()).unwrap(),
        point.neg(c()).unwrap(),
    ] {
        assert!(result.same_set(&point));
        let (low, high) = result.endpoints().unwrap();
        assert!(low.is_negative());
        assert!(!high.is_negative());
    }
    let ball = Ball::new(minus.clone(), minus).unwrap();
    let (mid, radius) = ball.components().unwrap();
    assert!(mid.is_negative());
    assert!(!radius.is_negative());
    assert!(ball.to_interval(c()).unwrap().contains_zero());
}
#[test]
fn empty_entire_and_unbounded_lattice_semantics() {
    let minus_inf = Float::infinity(p(), true);
    let plus_inf = Float::infinity(p(), false);
    let left = Interval::new(minus_inf, f(0)).unwrap();
    let right = Interval::new(f(0), plus_inf).unwrap();
    let union_hull = left.hull(&right);
    assert!(union_hull.is_entire());
    let intersect = left.intersection(&right);
    assert!(intersect.same_set(&i(0, 0)));
    let finite = i(-3, 4);
    assert!(Interval::entire().intersection(&finite).same_set(&finite));
    assert!(Interval::empty().intersection(&finite).is_empty());
    assert!(Interval::empty().hull(&finite).same_set(&finite));
    assert!(Interval::entire().add(&finite, c()).unwrap().is_entire());
    assert!(Interval::empty().div(&finite, c()).unwrap().is_empty());
    assert!(finite.mul(&Interval::entire(), c()).unwrap().is_entire());
    assert!(finite.contains(&f(4)));
    assert!(!finite.contains(&f(5)));
    assert!(!Interval::empty().contains(&f(0)));
}
#[test]
fn singularity_and_infinite_corner_enclosures() {
    let one = i(1, 1);
    let zero = i(0, 0);
    assert!(one.div(&zero, c()).unwrap().is_entire());
    assert!(zero.div(&zero, c()).unwrap().is_entire());
    assert!(i(-2, 2).reciprocal(c()).unwrap().is_entire());
    assert!(one.div(&i(0, 2), c()).unwrap().is_entire());
    assert!(one.div(&i(-2, 0), c()).unwrap().is_entire());
    let positive = Interval::new(f(1), Float::infinity(p(), false)).unwrap();
    let reciprocal = positive.reciprocal(c()).unwrap();
    assert!(reciprocal.contains(&f(0)));
    assert!(reciprocal.contains(&f(1)));
    let zero_times_infinity =
        Interval::new(Float::infinity(p(), false), Float::infinity(p(), false)).unwrap();
    assert!(zero.mul(&zero_times_infinity, c()).unwrap().is_entire());
    assert!(
        i(-4, -2).reciprocal(c()).unwrap().contains(
            &f(-1)
                .div_round(&f(2), p(), RoundingMode::NearestTiesToEven)
                .unwrap()
                .into_value()
        )
    );
}
#[test]
fn malformed_ball_and_enclosure_limits_are_typed() {
    assert!(matches!(
        Ball::new(f(1), f(-1)),
        Err(EnclosureError::InvalidBall)
    ));
    assert!(matches!(
        Ball::new(Float::nan(p()), f(1)),
        Err(EnclosureError::NaNValue)
    ));
    assert!(matches!(
        Interval::new(f(2), f(1)),
        Err(EnclosureError::ReversedBounds)
    ));
    assert!(matches!(
        EnclosureContext::new(Precision::new(32).unwrap(), 8),
        Err(EnclosureError::PrecisionLimitExceeded {
            requested: 32,
            maximum: 8
        })
    ));
    let source = Interval::new(
        Float::zero(Precision::new(64).unwrap(), false),
        Float::zero(Precision::new(64).unwrap(), false),
    )
    .unwrap();
    assert!(matches!(
        source.enclose_at(EnclosureContext::new(p(), 32).unwrap()),
        Err(EnclosureError::PrecisionLimitExceeded {
            requested: 64,
            maximum: 32
        })
    ));
}
