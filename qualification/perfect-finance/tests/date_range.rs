use perfect_finance::{
    CashFlow, CurrencyCode, Date, DateBoundary, DateRange, Decimal, DecimalLimits, FinanceError,
    Money, Month, aggregate_cash_flows_in_range, net_cash_flows_by_currency_in_range,
};
const SOURCE: &str = include_str!("../vectors/date_range_v1.tsv");
fn date(s: &str) -> Date {
    let parts: Vec<_> = s.split('-').map(|s| s.parse::<i32>().unwrap()).collect();
    let month = Month::try_from(parts[1] as u8).unwrap();
    Date::from_calendar_date(parts[0], month, parts[2] as u8).unwrap()
}
fn flow(day: &str, currency: &str, amount: &str, l: DecimalLimits) -> CashFlow {
    CashFlow::new(
        date(day),
        Money::new(
            Decimal::parse(amount, l).unwrap(),
            CurrencyCode::new(currency).unwrap(),
        ),
    )
}
#[test]
fn independently_computed_exact_leap_day_date_range_totals() {
    let limits = DecimalLimits::new(9).unwrap();
    let inputs = [
        flow("2024-02-28", "USD", "1.00", limits),
        flow("2024-02-28", "EUR", "-0.100", limits),
        flow("2024-02-29", "EUR", "2.500", limits),
        flow("2024-02-29", "USD", "0.015", limits),
        flow("2024-02-29", "USD", "-0.005", limits),
        flow("2024-02-29", "USD", "0.010", limits),
        flow("2024-03-01", "USD", "3.100", limits),
        flow("2024-03-01", "EUR", "0.500", limits),
    ];
    let mut count = 0;
    let mut cases = [false; 8];
    for row in SOURCE
        .lines()
        .filter(|s| !s.is_empty() && !s.starts_with('#'))
    {
        let fields: Vec<_> = row.split_whitespace().collect();
        assert_eq!(fields.len(), 7);
        let case: usize = fields[0].parse().unwrap();
        cases[case] = true;
        let start = if fields[3] == "I" {
            DateBoundary::Inclusive
        } else {
            DateBoundary::Exclusive
        };
        let end = if fields[4] == "I" {
            DateBoundary::Inclusive
        } else {
            DateBoundary::Exclusive
        };
        let range = DateRange::new(date(fields[1]), date(fields[2]), start, end).unwrap();
        let currency = CurrencyCode::new(fields[5]).unwrap();
        let expected = Decimal::parse(fields[6], limits).unwrap();
        let actual =
            net_cash_flows_by_currency_in_range(&inputs, &range, inputs.len(), limits).unwrap();
        let present = actual
            .iter()
            .find(|money| money.currency() == currency)
            .unwrap();
        assert_eq!(present.amount(), &expected, "range oracle row {count}");
        assert_eq!(present.amount().exponent(), expected.exponent());
        let mut shuffled = inputs.to_vec();
        shuffled.reverse();
        assert_eq!(
            net_cash_flows_by_currency_in_range(&shuffled, &range, shuffled.len(), limits).unwrap(),
            actual
        );
        let grouped = aggregate_cash_flows_in_range(&inputs, &range, inputs.len(), limits).unwrap();
        assert!(grouped.iter().all(|f| range.contains(f.date())));
        count += 1;
    }
    assert_eq!(count, 14);
    assert_eq!(cases.iter().filter(|&&s| s).count(), 7); // eighth case is empty
    let empty = DateRange::new(
        date("2024-02-29"),
        date("2024-02-29"),
        DateBoundary::Inclusive,
        DateBoundary::Exclusive,
    )
    .unwrap();
    assert!(
        net_cash_flows_by_currency_in_range(&inputs, &empty, 8, limits)
            .unwrap()
            .is_empty()
    );
    assert!(matches!(
        net_cash_flows_by_currency_in_range(&inputs, &empty, 7, limits),
        Err(FinanceError::ResourceLimit { max_flows: 7 })
    ));
    assert!(matches!(
        DateRange::new(
            date("2024-03-01"),
            date("2024-02-29"),
            DateBoundary::Inclusive,
            DateBoundary::Inclusive
        ),
        Err(FinanceError::InvalidDateRange)
    ));
}
