//! Independent Python Fraction/Decimal exact cash-flow reference fixtures.
use perfect_finance::{
    CashFlow, CurrencyCode, Date, Decimal, DecimalLimits, Money, Month, aggregate_cash_flows,
};
const DATA: &str = include_str!("../vectors/cashflow_cancel_v1.tsv");
#[test]
fn cashflow_representable_balanced_sums_match_fraction_oracle_in_all_cyclic_orders() {
    let code = CurrencyCode::new("USD").unwrap();
    let date = Date::from_calendar_date(2026, Month::October, 11).unwrap();
    let mut count = 0;
    for line in DATA
        .lines()
        .filter(|x| !x.is_empty() && !x.starts_with('#'))
    {
        let v: Vec<_> = line.split_whitespace().collect();
        let maxdigits: usize = v[0].parse().unwrap();
        let n: usize = v[1].parse().unwrap();
        assert_eq!(v.len(), n + 3);
        let limit = DecimalLimits::new(maxdigits as u64).unwrap();
        let expected = Decimal::parse(v[n + 2], limit).unwrap();
        let amounts: Vec<Decimal> = v[2..2 + n]
            .iter()
            .map(|x| Decimal::parse(x, limit).unwrap())
            .collect();
        for rotation in 0..n {
            let flows: Vec<_> = (0..n)
                .map(|i| CashFlow::new(date, Money::new(amounts[(i + rotation) % n].clone(), code)))
                .collect();
            for reverse in [false, true] {
                let mut attempt = flows.clone();
                if reverse {
                    attempt.reverse();
                }
                let sum=aggregate_cash_flows(&attempt,n,limit)
                    .unwrap_or_else(|e|panic!("valid exact reference row {count}, rot={rotation}, reverse={reverse}: {e:?}"));
                assert_eq!(sum.len(), 1);
                assert_eq!(
                    sum[0].money().amount(),
                    &expected,
                    "reference row {count}, rotation {rotation}, reverse {reverse}"
                );
                assert_eq!(
                    sum[0].money().amount().exponent().get(),
                    expected.exponent().get()
                );
            }
        }
        count += 1;
    }
    assert_eq!(count, 16);
}
