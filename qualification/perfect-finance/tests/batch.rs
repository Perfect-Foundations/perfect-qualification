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
    let mut reusable_totals = Vec::with_capacity(8);
    let mut reusable_grouped = Vec::with_capacity(8);
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
        batch
            .currency_totals_into(Some(&range), &mut reusable_totals)
            .unwrap();
        assert_eq!(reusable_totals, result);
        let expected = Decimal::parse(fields[6], limits).unwrap();
        let currency = CurrencyCode::new(fields[5]).unwrap();
        let found = result.iter().find(|m| m.currency() == currency).unwrap();
        assert_eq!(found.amount(), &expected, "oracle {count}");
        let dated = batch.aggregate(Some(&range)).unwrap();
        batch
            .aggregate_into(Some(&range), &mut reusable_grouped)
            .unwrap();
        assert_eq!(reusable_grouped, dated);
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

#[test]
fn caller_owned_outputs_commit_only_after_exact_success() {
    let limits = DecimalLimits::new(1).unwrap();
    let rows = [
        row("2024-02-29", "EUR", "1", limits),
        row("2024-02-29", "USD", "9", limits),
        row("2024-02-29", "USD", "9", limits),
    ];
    let mut batch = CashFlowBatch::new(&rows, 3, limits).unwrap();
    let marker = row("2024-02-28", "JPY", "3", limits);
    let mut dated = vec![marker.clone()];
    let mut totals = vec![marker.money().clone()];
    assert!(batch.aggregate_into(None, &mut dated).is_err());
    assert!(batch.currency_totals_into(None, &mut totals).is_err());
    assert_eq!(dated.as_slice(), core::slice::from_ref(&marker));
    assert_eq!(totals[0], *marker.money());
    let replacement = [row("2024-02-29", "EUR", "2", limits)];
    batch.reset(&replacement, 1, limits).unwrap();
    for _ in 0..5 {
        batch.aggregate_into(None, &mut dated).unwrap();
        batch.currency_totals_into(None, &mut totals).unwrap();
        assert_eq!(dated.as_slice(), replacement.as_slice());
        assert_eq!(totals[0], *replacement[0].money());
    }
}

#[test]
fn narrow_1024_flow_range_matches_independent_python_decimal_sums() {
    let limits = DecimalLimits::new(18).unwrap();
    let currencies = ["USD", "EUR", "JPY", "GBP"];
    let mut flows = Vec::new();
    for i in 0..1024 {
        let day = format!("2024-02-{:02}", i % 29 + 1);
        let amount = if i % 2 == 0 {
            "1000000.007"
        } else {
            "-999999.992"
        };
        flows.push(row(&day, currencies[i % 4], amount, limits));
    }
    let day = date("2024-02-15");
    let range = DateRange::new(day, day, DateBoundary::Inclusive, DateBoundary::Inclusive).unwrap();
    let expected = [
        ("EUR", "-7999999.936"),
        ("GBP", "-8999999.928"),
        ("JPY", "9000000.063"),
        ("USD", "9000000.063"),
    ];
    let mut batch = CashFlowBatch::new(&flows, 1024, limits).unwrap();
    let mut result = Vec::with_capacity(8);
    let mut dated = Vec::with_capacity(8);
    for _ in 0..8 {
        batch
            .currency_totals_into(Some(&range), &mut result)
            .unwrap();
        batch.aggregate_into(Some(&range), &mut dated).unwrap();
        assert_eq!(result.len(), 4);
        for (actual, (code, amount)) in result.iter().zip(expected.iter()) {
            assert_eq!(actual.currency(), CurrencyCode::new(code).unwrap());
            assert_eq!(actual.amount(), &Decimal::parse(amount, limits).unwrap());
        }
        assert_eq!(dated.len(), 4);
        assert!(dated.iter().all(|x| x.date() == day));
        for item in &dated {
            let matching = result
                .iter()
                .find(|r| r.currency() == item.money().currency())
                .unwrap();
            assert_eq!(item.money(), matching);
        }
    }
}
