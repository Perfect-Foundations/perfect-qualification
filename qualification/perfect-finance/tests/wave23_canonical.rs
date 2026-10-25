use perfect_finance::{
    CashFlow, CurrencyCode, Date, Decimal, DecimalLimits, Money, Month, OwnedCashFlowLedger,
};
fn flow(day: u8, currency: &str, value: i64, lim: DecimalLimits) -> CashFlow {
    CashFlow::new(
        Date::from_calendar_date(2024, Month::February, day).unwrap(),
        Money::new(
            Decimal::parse(&value.to_string(), lim).unwrap(),
            CurrencyCode::new(currency).unwrap(),
        ),
    )
}
#[test]
fn canonical_exact_finance_values() {
    let lim = DecimalLimits::new(8).unwrap();
    let (eur, usd) = (
        CurrencyCode::new("EUR").unwrap(),
        CurrencyCode::new("USD").unwrap(),
    );
    for case in 0..48_i64 {
        let (a, b, c) = (case - 21, case % 13 - 6, 9 - case % 17);
        let flows = [
            flow(28, "USD", a, lim),
            flow(29, "USD", b, lim),
            flow(28, "EUR", c, lim),
        ];
        let ledger = OwnedCashFlowLedger::new(&flows, 16, lim).unwrap();
        let mut view = ledger.indexed_view().unwrap();
        let totals = view.currency_totals(None).unwrap();
        assert_eq!(totals.len(), 2);
        let euro = totals.iter().find(|m| m.currency() == eur).unwrap();
        let dollar = totals.iter().find(|m| m.currency() == usd).unwrap();
        assert_eq!(euro.amount(), &Decimal::parse(&c.to_string(), lim).unwrap());
        assert_eq!(
            dollar.amount(),
            &Decimal::parse(&(a + b).to_string(), lim).unwrap()
        );
        println!(
            "CANONICAL|FIN|{case:02}|{}|{}",
            euro.amount(),
            dollar.amount()
        );
    }
}
