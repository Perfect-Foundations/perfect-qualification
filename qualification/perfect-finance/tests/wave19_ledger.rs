use perfect_finance::{
    CashFlow, CurrencyCode, Date, DateBoundary, DateRange, Decimal, DecimalLimits, FinanceError,
    LedgerOperation, Money, Month, OwnedCashFlowLedger, aggregate_cash_flows_in_range,
    net_cash_flows_by_currency_in_range,
};
fn flow(day: u8, currency: &str, amount: &str, limits: DecimalLimits) -> CashFlow {
    CashFlow::new(
        Date::from_calendar_date(2024, Month::February, day).unwrap(),
        Money::new(
            Decimal::parse(amount, limits).unwrap(),
            CurrencyCode::new(currency).unwrap(),
        ),
    )
}
#[test]
fn wave19_atomic_operations_exact_totals_and_filtered_invalid_refusal() {
    let lim = DecimalLimits::new(6).unwrap();
    let seed = [
        flow(28, "USD", "1.00", lim),
        flow(29, "USD", "2.00", lim),
        flow(29, "EUR", "3.500", lim),
    ];
    let mut ledger = OwnedCashFlowLedger::new(&seed, 5, lim).unwrap();
    let id = ledger.ids().to_vec();
    let new_a = flow(28, "USD", "3.00", lim);
    let add_a = flow(29, "USD", "-1.00", lim);
    let add_b = flow(29, "EUR", "-0.500", lim);
    let created = ledger
        .apply_operations(
            &[
                LedgerOperation::Replace(id[0], &new_a),
                LedgerOperation::Remove(id[1]),
                LedgerOperation::Append(&add_a),
                LedgerOperation::Append(&add_b),
            ],
            4,
        )
        .unwrap();
    assert_eq!(created.iter().map(|x| x.get()).collect::<Vec<_>>(), [3, 4]);
    assert_eq!(ledger.ids(), &[id[0], id[2], created[0], created[1]]);
    let range = DateRange::new(
        Date::from_calendar_date(2024, Month::February, 28).unwrap(),
        Date::from_calendar_date(2024, Month::February, 29).unwrap(),
        DateBoundary::Inclusive,
        DateBoundary::Inclusive,
    )
    .unwrap();
    let grouped = aggregate_cash_flows_in_range(ledger.flows(), &range, 5, lim).unwrap();
    assert_eq!(grouped.len(), 3);
    let mut view = ledger.indexed_view().unwrap();
    assert_eq!(view.aggregate(Some(&range)).unwrap(), grouped);
    let totals = view.currency_totals(Some(&range)).unwrap();
    assert_eq!(
        totals,
        net_cash_flows_by_currency_in_range(ledger.flows(), &range, 5, lim).unwrap()
    );
    assert_eq!(totals.len(), 2);
    assert_eq!(totals[0].currency(), CurrencyCode::new("EUR").unwrap());
    assert_eq!(totals[0].amount(), &Decimal::parse("3.000", lim).unwrap());
    assert_eq!(totals[1].amount(), &Decimal::parse("2.00", lim).unwrap());
    drop(view);
    let snapshot = ledger.flows().to_vec();
    assert!(matches!(
        ledger.apply_operations(
            &[
                LedgerOperation::Remove(id[0]),
                LedgerOperation::Remove(id[0]),
            ],
            2
        ),
        Err(FinanceError::DuplicateCashFlowOperation { id: 0 })
    ));
    assert!(matches!(
        ledger.apply_operations(
            &[
                LedgerOperation::Append(&add_a),
                LedgerOperation::Append(&add_a)
            ],
            1
        ),
        Err(FinanceError::QueryLimit {
            dimension: "ledger_operations",
            limit: 1
        })
    ));
    assert_eq!(ledger.flows(), snapshot);
    let next = ledger
        .apply_operations(&[LedgerOperation::Append(&add_a)], 1)
        .unwrap();
    assert_eq!(next[0].get(), 5);

    let narrow = DecimalLimits::new(1).unwrap();
    let broader = DecimalLimits::new(5).unwrap();
    let invalid_outside = [
        flow(28, "USD", "1", narrow),
        flow(29, "USD", "123", broader),
    ];
    let only_28 = DateRange::new(
        Date::from_calendar_date(2024, Month::February, 28).unwrap(),
        Date::from_calendar_date(2024, Month::February, 28).unwrap(),
        DateBoundary::Inclusive,
        DateBoundary::Inclusive,
    )
    .unwrap();
    assert!(aggregate_cash_flows_in_range(&invalid_outside, &only_28, 2, narrow).is_err());
    assert!(net_cash_flows_by_currency_in_range(&invalid_outside, &only_28, 2, narrow).is_err());
}
