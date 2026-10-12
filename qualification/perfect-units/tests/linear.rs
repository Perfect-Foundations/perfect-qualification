use perfect_arithmetic::Integer;
use perfect_rational::Rational;
use perfect_units::{Unit, si};
const DATA: &str = include_str!("../vectors/linear_v1.tsv");
fn frac(a: i64, b: i64) -> Rational {
    Rational::new(Integer::from(a), Integer::from(b)).unwrap()
}
#[test]
fn exact_python_fraction_linear_conversion_reference() {
    let mut n = 0_usize;
    for row in DATA.lines().filter(|x| !x.starts_with('#')) {
        let col: Vec<&str> = row.split_whitespace().collect();
        let numbers: Vec<i64> = col[1..].iter().map(|x| x.parse().unwrap()).collect();
        let value = frac(numbers[0], numbers[1]);
        let expected = frac(numbers[2], numbers[3]);
        let actual = match col[0] {
            "km_to_m" => Unit::convert_value(&value, &si::kilometre(), &si::metre()),
            "m_to_km" => Unit::convert_value(&value, &si::metre(), &si::kilometre()),
            "ms_to_s" => Unit::convert_value(&value, &si::millisecond(), &si::second()),
            "s_to_ms" => Unit::convert_value(&value, &si::second(), &si::millisecond()),
            _ => panic!("unknown oracle ID"),
        }
        .unwrap();
        assert_eq!(actual, expected, "{row}");
        n += 1;
    }
    assert_eq!(n, 40);
}
