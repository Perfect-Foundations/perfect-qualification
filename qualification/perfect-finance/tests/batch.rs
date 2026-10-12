use perfect_finance::{
    CashFlow, CashFlowBatch, CurrencyCode, Date, DateBoundary, DateRange, Decimal, DecimalLimits,
    FinanceError, Money, Month,
};
const SOURCE: &str = include_str!("../vectors/date_range_v1.tsv");
fn date(s: &str) -> Date {
    let v: Vec<_> = s.split('-').map(|x| x.parse::<i32>().unwrap()).collect();
    Date::from_calendar_date(v[0], Month::try_from(v[1] as u8).unwrap(), v[2] as u8).unwrap()
}
fn row(day: &str, currency: &str, value: &str, l: DecimalLimits) -> CashFlow {
    CashFlow::new(
        date(day),
        Money::new(
            Decimal::parse(value, l).unwrap(),
            CurrencyCode::new(currency).unwrap(),
        ),
    )
}
#[test]
fn batched_range_queries_match_python_exact_references() {
    let limits = DecimalLimits::new(10).unwrap();
    let flows = [
        row("2024-02-28", "USD", "1.00", limits),
        row("2024-02-28", "EUR", "-0.100", limits),
        row("2024-02-29", "EUR", "2.500", limits),
        row("2024-02-29", "USD", "0.015", limits),
        row("2024-02-29", "USD", "-0.005", limits),
        row("2024-02-29", "USD", "0.010", limits),
        row("2024-03-01", "USD", "3.100", limits),
        row("2024-03-01", "EUR", "0.500", limits),
    ];
    let mut batch = CashFlowBatch::new(&flows, 8, limits).unwrap();
    let cap = batch.retained_index_capacity();
    let mut count = 0;
    for entry in SOURCE
        .lines()
        .filter(|l| !l.starts_with('#') && !l.is_empty())
    {
        let fields: Vec<_> = entry.split_whitespace().collect();
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
        let result = batch.currency_totals(Some(&range)).unwrap();
        let expected = Decimal::parse(fields[6], limits).unwrap();
        let currency = CurrencyCode::new(fields[5]).unwrap();
        let found = result.iter().find(|m| m.currency() == currency).unwrap();
        assert_eq!(found.amount(), &expected, "oracle {count}");
        let dated = batch.aggregate(Some(&range)).unwrap();
        assert!(dated.iter().all(|f| range.contains(f.date())));
        count += 1;
    }
    assert_eq!(count, 14);
    assert!(matches!(
        batch.reset(&flows, 7, limits),
        Err(FinanceError::ResourceLimit { max_flows: 7 })
    ));
    assert_eq!(batch.len(), 8);
    assert!(batch.retained_index_capacity() >= cap);
}
