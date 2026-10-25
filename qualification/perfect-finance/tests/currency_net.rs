use perfect_finance::{
    CashFlow, CurrencyCode, Date, Decimal, DecimalLimits, Money, Month, net_cash_flows_by_currency,
};
const SOURCE: &str = include_str!("../vectors/currency_net_v1.tsv");
fn cash(day: u8, currency: &str, amount: &str, l: DecimalLimits) -> CashFlow {
    CashFlow::new(
        Date::from_calendar_date(2026, Month::October, day).unwrap(),
        Money::new(
            Decimal::parse(amount, l).unwrap(),
            CurrencyCode::new(currency).unwrap(),
        ),
    )
}
#[test]
fn exact_python_decimal_oracle_currency_totals_across_dates() {
    let limits = DecimalLimits::new(5).unwrap();
    let cases: Vec<Vec<CashFlow>> = vec![
        vec![
            cash(1, "USD", "1.20", limits),
            cash(2, "EUR", "4.50", limits),
            cash(2, "USD", "3.10", limits),
            cash(1, "USD", "-0.10", limits),
        ],
        vec![
            cash(1, "USD", "900.00", limits),
            cash(2, "USD", "-900.00", limits),
            cash(3, "USD", "0.005", limits),
            cash(1, "JPY", "-0.00", limits),
        ],
        vec![
            cash(1, "EUR", "0.05", limits),
            cash(2, "EUR", "0.05", limits),
            cash(3, "EUR", "-0.10", limits),
            cash(2, "USD", "2.500", limits),
            cash(1, "USD", "-0.125", limits),
        ],
    ];
    let outputs: Vec<_> = cases
        .iter()
        .map(|f| net_cash_flows_by_currency(f, f.len(), limits).unwrap())
        .collect();
    let mut actual = 0;
    for line in SOURCE
        .lines()
        .filter(|s| !s.starts_with('#') && !s.is_empty())
    {
        let fields: Vec<_> = line.split_whitespace().collect();
        assert_eq!(fields.len(), 3);
        let scenario: usize = fields[0].parse().unwrap();
        let curr = CurrencyCode::new(fields[1]).unwrap();
        let expected = Decimal::parse(fields[2], limits).unwrap();
        let output = outputs[scenario]
            .iter()
            .find(|m| m.currency() == curr)
            .unwrap();
        assert_eq!(output.amount(), &expected, "reference line {actual}");
        assert_eq!(output.amount().exponent(), expected.exponent());
        if fields[2].starts_with("-0") && expected.is_zero() {
            assert_eq!(output.amount().is_negative(), expected.is_negative());
        }
        actual += 1;
    }
    assert_eq!(actual, 6);
    for (flows, amounts) in cases.iter().zip(outputs) {
        let mut reverse = flows.clone();
        reverse.reverse();
        assert_eq!(
            net_cash_flows_by_currency(&reverse, reverse.len(), limits).unwrap(),
            amounts
        );
    }
}
