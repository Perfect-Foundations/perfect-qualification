use perfect_finance::{
    CashFlow, CurrencyCode, Date, DateBoundary, DateRange, Decimal, DecimalLimits, FinanceError,
    Money, Month, OwnedCashFlowLedger, RangeQueryLimits, aggregate_cash_flows_in_range,
    net_cash_flows_by_currency_in_range,
};
const ORACLE: &str = include_str!("../vectors/date_range_v1.tsv");
fn date(s: &str) -> Date {
    let v = s
        .split('-')
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    Date::from_calendar_date(v[0], Month::try_from(v[1] as u8).unwrap(), v[2] as u8).unwrap()
}
fn flow(s: &str, c: &str, v: &str, l: DecimalLimits) -> CashFlow {
    CashFlow::new(
        date(s),
        Money::new(Decimal::parse(v, l).unwrap(), CurrencyCode::new(c).unwrap()),
    )
}
#[test]
fn independent_frozen_range_oracles_apply_to_owned_ledger_and_multirange() {
    let l = DecimalLimits::new(10).unwrap();
    let flows = [
        flow("2024-02-28", "USD", "1.00", l),
        flow("2024-02-28", "EUR", "-0.100", l),
        flow("2024-02-29", "EUR", "2.500", l),
        flow("2024-02-29", "USD", "0.015", l),
        flow("2024-02-29", "USD", "-0.005", l),
        flow("2024-02-29", "USD", "0.010", l),
        flow("2024-03-01", "USD", "3.100", l),
        flow("2024-03-01", "EUR", "0.500", l),
    ];
    let mut ledger = OwnedCashFlowLedger::new(&flows, 12, l).unwrap();
    let ids = ledger.ids().to_vec();
    assert_eq!(ids.len(), 8);
    let mut ranges = Vec::new();
    let mut targets = Vec::new();
    for line in ORACLE
        .lines()
        .filter(|s| !s.is_empty() && !s.starts_with('#'))
    {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        let edge = |v: &str| {
            if v == "I" {
                DateBoundary::Inclusive
            } else {
                DateBoundary::Exclusive
            }
        };
        ranges.push(
            DateRange::new(
                date(parts[1]),
                date(parts[2]),
                edge(parts[3]),
                edge(parts[4]),
            )
            .unwrap(),
        );
        targets.push((
            CurrencyCode::new(parts[5]).unwrap(),
            Decimal::parse(parts[6], l).unwrap(),
        ));
    }
    assert_eq!(ranges.len(), 14);
    let limits = RangeQueryLimits {
        max_ranges: 14,
        max_total_groups: 128,
    };
    {
        let mut view = ledger.indexed_view().unwrap();
        let mut dated = vec![vec![flows[0].clone()]];
        let mut totals = vec![vec![flows[0].money().clone()]];
        for _ in 0..3 {
            view.aggregate_ranges_into(&ranges, limits, &mut dated)
                .unwrap();
            view.currency_totals_ranges_into(&ranges, limits, &mut totals)
                .unwrap();
            assert_eq!(dated.len(), 14);
            assert_eq!(totals.len(), 14);
            for (i, range) in ranges.iter().enumerate() {
                assert_eq!(
                    dated[i],
                    aggregate_cash_flows_in_range(&flows, range, 12, l).unwrap()
                );
                assert_eq!(
                    totals[i],
                    net_cash_flows_by_currency_in_range(&flows, range, 12, l).unwrap()
                );
                let (currency, expected) = &targets[i];
                assert_eq!(
                    totals[i]
                        .iter()
                        .find(|x| x.currency() == *currency)
                        .unwrap()
                        .amount(),
                    expected
                );
            }
        }
        let original = dated.clone();
        assert!(matches!(
            view.aggregate_ranges_into(
                &ranges,
                RangeQueryLimits {
                    max_ranges: 13,
                    ..limits
                },
                &mut dated
            ),
            Err(FinanceError::QueryLimit {
                dimension: "ranges",
                ..
            })
        ));
        assert_eq!(dated, original);
    }
    let changed = flow("2024-02-29", "USD", "-3.125", l);
    ledger.replace(ids[0], changed.clone()).unwrap();
    assert_eq!(ledger.ids()[0], ids[0]);
    let added = ledger
        .append(&[flow("2024-03-01", "EUR", "-0.500", l)])
        .unwrap();
    assert_eq!(added.len(), 1);
    assert_ne!(added[0], ids[0]);
    let prior = ledger.ids().to_vec();
    assert!(matches!(
        ledger.append(&[
            flows[0].clone(),
            flows[1].clone(),
            flows[2].clone(),
            flows[3].clone()
        ]),
        Err(FinanceError::ResourceLimit { max_flows: 12 })
    ));
    assert_eq!(ledger.ids(), prior);
    ledger.remove(ids[1]).unwrap();
    assert!(ledger.get(ids[1]).is_none());
    let snapshot = ledger.flows().to_vec();
    let mut view = ledger.indexed_view().unwrap();
    let mut grouped = Vec::new();
    view.aggregate_ranges_into(&ranges, limits, &mut grouped)
        .unwrap();
    for (index, range) in ranges.iter().enumerate() {
        assert_eq!(
            grouped[index],
            aggregate_cash_flows_in_range(&snapshot, range, 12, l).unwrap()
        );
    }
}
