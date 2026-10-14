use perfect_finance::{
    CashFlow, CurrencyCode, Date, DayCount, Decimal, DecimalExponent, DecimalLimits, FinanceError,
    Money, Month, Rational, RoundingMode, aggregate_cash_flows,
};
const DAY: &str = include_str!("../vectors/day_count_v1.tsv");
const MONEY: &str = include_str!("../vectors/money_quantize_v1.tsv");
fn lim() -> DecimalLimits {
    DecimalLimits::new(100).unwrap()
}
fn dec(value: &str) -> Decimal {
    Decimal::parse(value, lim()).unwrap()
}
fn day(y: &str, m: &str, d: &str) -> Date {
    Date::from_calendar_date(
        y.parse().unwrap(),
        Month::try_from(m.parse::<u8>().unwrap()).unwrap(),
        d.parse().unwrap(),
    )
    .unwrap()
}
#[test]
fn independent_fraction_day_count_all_conventions_and_reversals() {
    let mut checked = 0;
    for line in DAY.lines().filter(|l| !l.starts_with('#')) {
        let col: Vec<&str> = line.split_whitespace().collect();
        assert_eq!(col.len(), 9);
        let convention = match col[0] {
            "ACT360" => DayCount::Actual360,
            "ACT365" => DayCount::Actual365Fixed,
            "US30360" => DayCount::Thirty360Us,
            "EU30360" => DayCount::ThirtyE360,
            _ => panic!("invalid fixture"),
        };
        let first = day(col[1], col[2], col[3]);
        let last = day(col[4], col[5], col[6]);
        let numerator: Rational = Rational::from(col[7].parse::<i64>().unwrap());
        let denominator: Rational = Rational::from(col[8].parse::<i64>().unwrap());
        assert_eq!(
            convention.year_fraction(first, last).unwrap(),
            numerator.div(&denominator).unwrap(),
            "{line}"
        );
        checked += 1;
    }
    assert_eq!(checked, 36);
}
#[test]
fn independent_decimal_quantize_signed_zero_huge_magnitudes_and_ties() {
    let code = CurrencyCode::new("ZZZ").unwrap();
    let mut checked = 0;
    for line in MONEY.lines().filter(|l| !l.starts_with('#')) {
        let col: Vec<&str> = line.split_whitespace().collect();
        assert_eq!(col.len(), 4);
        let mode = match col[1] {
            "EVEN" => RoundingMode::NearestTiesToEven,
            "AWAY" => RoundingMode::NearestTiesAway,
            "ZERO" => RoundingMode::TowardZero,
            _ => panic!("invalid rounding fixture"),
        };
        let money = Money::new(dec(col[0]), code);
        let result = money
            .quantize(DecimalExponent::new(col[2].parse().unwrap()), mode, lim())
            .unwrap()
            .into_value();
        assert_eq!(result.amount(), &dec(col[3]), "{line}");
        assert_eq!(result.amount().exponent().get(), -2);
        if result.amount().is_zero() {
            assert_eq!(
                result.amount().is_negative(),
                col[0].starts_with('-'),
                "{line}"
            );
        }
        checked += 1;
    }
    assert_eq!(checked, 48);
}
#[test]
fn explicit_mismatch_cashflow_aggregation_determinism_and_resource_refusal() {
    let usd = CurrencyCode::new("USD").unwrap();
    let eur = CurrencyCode::new("EUR").unwrap();
    let date = Date::from_calendar_date(2026, Month::October, 11).unwrap();
    let a = Money::new(dec("1.25"), usd);
    let b = Money::new(dec("-0.25"), usd);
    assert_eq!(a.add(&b, lim()).unwrap().amount(), &dec("1.00"));
    assert!(matches!(
        a.add(&Money::new(dec("1"), eur), lim()),
        Err(FinanceError::CurrencyMismatch { .. })
    ));
    let flows = [
        CashFlow::new(date, a),
        CashFlow::new(date, b),
        CashFlow::new(date, Money::new(dec("3.50"), eur)),
    ];
    let mut reverse = flows.clone();
    reverse.reverse();
    let normal = aggregate_cash_flows(&flows, 3, lim()).unwrap();
    assert_eq!(normal, aggregate_cash_flows(&reverse, 3, lim()).unwrap());
    assert_eq!(normal.len(), 2);
    assert!(matches!(
        aggregate_cash_flows(&flows, 2, lim()),
        Err(FinanceError::ResourceLimit { max_flows: 2 })
    ));
}
