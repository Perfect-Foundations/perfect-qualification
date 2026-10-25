use perfect_finance::{
    CashFlow, CurrencyCode, Date, DateBoundary, DateRange, Decimal, DecimalLimits, FinanceError,
    LedgerOperation, Money, Month, OwnedCashFlowLedger, RangeGrouping, RangeQueryLimits,
};
fn f(day: u8, cur: &str, value: &str, lim: DecimalLimits) -> CashFlow {
    CashFlow::new(
        Date::from_calendar_date(2024, Month::February, day).unwrap(),
        Money::new(
            Decimal::parse(value, lim).unwrap(),
            CurrencyCode::new(cur).unwrap(),
        ),
    )
}
fn r(a: u8, b: u8, sa: DateBoundary, sb: DateBoundary) -> DateRange {
    DateRange::new(
        Date::from_calendar_date(2024, Month::February, a).unwrap(),
        Date::from_calendar_date(2024, Month::February, b).unwrap(),
        sa,
        sb,
    )
    .unwrap()
}
#[test]
fn public_transaction_and_bounded_group_planning_exact_currencies() {
    use DateBoundary::{Exclusive, Inclusive};
    let lim = DecimalLimits::new(10).unwrap();
    let initial = [
        f(28, "USD", "1.00", lim),
        f(28, "EUR", "-2.000", lim),
        f(29, "USD", "0.015", lim),
        f(29, "USD", "-0.005", lim),
        f(29, "EUR", "2.500", lim),
    ];
    let mut book = OwnedCashFlowLedger::new(&initial, 9, lim).unwrap();
    let ids = book.ids().to_vec();
    let replace = f(28, "USD", "10.00", lim);
    let add_usd = f(29, "USD", "3.000", lim);
    let add_eur = f(29, "EUR", "-1.500", lim);
    let created = book
        .apply_operations(
            &[
                LedgerOperation::Replace(ids[0], &replace),
                LedgerOperation::Remove(ids[3]),
                LedgerOperation::Append(&add_usd),
                LedgerOperation::Append(&add_eur),
            ],
            4,
        )
        .unwrap();
    assert_eq!(created.iter().map(|x| x.get()).collect::<Vec<_>>(), [5, 6]);
    assert_eq!(book.get(ids[0]), Some(&replace));
    assert_eq!(book.get(ids[3]), None);
    let snapshot = book.flows().to_vec();
    assert!(matches!(
        book.apply_operations(
            &[
                LedgerOperation::Replace(ids[0], &replace),
                LedgerOperation::Remove(ids[0])
            ],
            2
        ),
        Err(FinanceError::DuplicateCashFlowOperation { .. })
    ));
    assert_eq!(book.flows(), snapshot);
    let ranges = [
        r(28, 29, Inclusive, Inclusive),
        r(28, 29, Exclusive, Inclusive),
        r(29, 29, Inclusive, Exclusive),
        r(28, 29, Inclusive, Inclusive),
    ];
    let limits = RangeQueryLimits {
        max_ranges: 4,
        max_total_groups: 10,
    };
    let mut view = book.indexed_view().unwrap();
    let mut counts = vec![555];
    view.group_counts_into(&ranges, RangeGrouping::DateAndCurrency, limits, &mut counts)
        .unwrap();
    assert_eq!(counts, [4, 2, 0, 4]);
    let mut groups = vec![vec![initial[0].clone()]];
    view.aggregate_ranges_into(&ranges, limits, &mut groups)
        .unwrap();
    assert_eq!(groups.iter().map(Vec::len).collect::<Vec<_>>(), counts);
    assert_eq!(
        groups[0][0].money().currency(),
        CurrencyCode::new("EUR").unwrap()
    );
    assert_eq!(
        groups[0][0].money().amount(),
        &Decimal::parse("-2.000", lim).unwrap()
    );
    assert_eq!(
        groups[0][1].money().amount(),
        &Decimal::parse("10.00", lim).unwrap()
    );
    assert_eq!(
        groups[0][2].money().amount(),
        &Decimal::parse("1.000", lim).unwrap()
    );
    assert_eq!(
        groups[0][3].money().amount(),
        &Decimal::parse("3.015", lim).unwrap()
    );
    view.group_counts_into(&ranges, RangeGrouping::Currency, limits, &mut counts)
        .unwrap();
    assert_eq!(counts, [2, 2, 0, 2]);
    let mut totals = Vec::new();
    view.currency_totals_ranges_into(&ranges, limits, &mut totals)
        .unwrap();
    assert_eq!(totals.iter().map(Vec::len).collect::<Vec<_>>(), counts);
    assert_eq!(
        totals[0][0].amount(),
        &Decimal::parse("-1.000", lim).unwrap()
    );
    assert_eq!(
        totals[0][1].amount(),
        &Decimal::parse("13.015", lim).unwrap()
    );
    let retained = counts.clone();
    assert!(matches!(
        view.group_counts_into(
            &ranges,
            RangeGrouping::DateAndCurrency,
            RangeQueryLimits {
                max_ranges: 4,
                max_total_groups: 9
            },
            &mut counts
        ),
        Err(FinanceError::QueryLimit {
            dimension: "result_groups",
            limit: 9
        })
    ));
    assert_eq!(counts, retained);
    drop(view);
    let extra = book
        .apply_operations(&[LedgerOperation::Append(&add_usd)], 1)
        .unwrap();
    assert_eq!(extra[0].get(), 7);
    assert!(matches!(
        book.apply_operations(
            &[
                LedgerOperation::Append(&add_usd),
                LedgerOperation::Append(&add_usd),
            ],
            1
        ),
        Err(FinanceError::QueryLimit {
            dimension: "ledger_operations",
            limit: 1
        })
    ));
}
