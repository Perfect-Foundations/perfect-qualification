use perfect_rational::Rational;
use perfect_units::{DynamicQuantity, Quantity, UnitError, dimensions as d, si};
#[test]
fn independent_derived_type_and_dynamic_refusal() {
    let velocity: Quantity<Rational, d::Velocity> = (si::kilometre().from_value(Rational::one())
        / si::second().from_value(Rational::from(200)))
    .unwrap();
    assert_eq!(velocity.value(), &Rational::from(5));
    let force: Quantity<Rational, d::Force> = si::kilogram().from_value(Rational::from(3))
        * ((velocity / si::second().from_value(Rational::from(5))).unwrap());
    assert_eq!(force.value(), &Rational::from(3));
    let dynamic = DynamicQuantity::from_static(&force);
    assert!(matches!(
        dynamic.checked_static::<d::Time>(),
        Err(UnitError::IncompatibleDimension { .. })
    ));
    assert_eq!(dynamic.checked_static::<d::Force>().unwrap(), force);
    assert!(matches!(
        si::metre().from_value(Rational::one()) / si::second().from_value(Rational::zero()),
        Err(UnitError::DivisionByZero)
    ));
}
