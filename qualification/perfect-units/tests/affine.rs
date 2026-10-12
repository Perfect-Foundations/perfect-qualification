use perfect_arithmetic::Integer;
use perfect_rational::Rational;
use perfect_units::si;
const DATA: &str = include_str!("../vectors/affine_v1.tsv");
fn frac(a: i64, b: i64) -> Rational {
    Rational::new(Integer::from(a), Integer::from(b)).unwrap()
}
#[test]
fn python_fraction_affine_reference() {
    let mut count = 0;
    for row in DATA.lines().filter(|x| !x.starts_with('#')) {
        let c: Vec<&str> = row.split_whitespace().collect();
        let values: Vec<i64> = c[1..].iter().map(|s| s.parse().unwrap()).collect();
        let source = frac(values[0], values[1]);
        let expected = frac(values[2], values[3]);
        let point = if c[0] == "C" {
            si::celsius().point(source.clone())
        } else {
            si::fahrenheit().point(source.clone())
        };
        assert_eq!(point.value(), &expected, "{row}");
        let back = if c[0] == "C" {
            si::celsius().value_of(&point).unwrap()
        } else {
            si::fahrenheit().value_of(&point).unwrap()
        };
        assert_eq!(back, source);
        count += 1;
    }
    assert_eq!(count, 20);
    assert_eq!(
        si::fahrenheit().difference(frac(180, 1)).value(),
        &Rational::from(100)
    );
}
