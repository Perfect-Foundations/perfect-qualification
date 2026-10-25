//! Exact Python Fraction oracle, independent of the Rust Decimal reduction.
use perfect_finance::{
    CashFlow, CurrencyCode, Date, Decimal, DecimalLimits, FinanceError, Money, Month,
    aggregate_cash_flows,
};
const SOURCE: &str = include_str!("../vectors/multioperand_v1.tsv");

#[test]
fn exact_bounded_multioperand_oracle_with_permutations() {
    let date = Date::from_calendar_date(2026, Month::October, 11).unwrap();
    let currency = CurrencyCode::new("USD").unwrap();
    let mut valid = 0;
    let mut refused = 0;
    for (index, line) in SOURCE
        .lines()
        .filter(|s| !s.starts_with('#') && !s.is_empty())
        .enumerate()
    {
        let fields: Vec<_> = line.split_whitespace().collect();
        let digits: u64 = fields[0].parse().unwrap();
        let n: usize = fields[1].parse().unwrap();
        assert_eq!(fields.len(), n + 3);
        let limit = DecimalLimits::new(digits).unwrap();
        let inputs: Vec<_> = fields[2..2 + n]
            .iter()
            .map(|text| Decimal::parse(text, limit).unwrap())
            .collect();
        let expected = if fields[n + 2] == "LIMIT" {
            None
        } else {
            Some(Decimal::parse(fields[n + 2], limit).unwrap())
        };
        for rotation in [0, n / 2, n - 1] {
            for reversed in [false, true] {
                let mut flows: Vec<_> = (0..n)
                    .map(|j| {
                        CashFlow::new(
                            date,
                            Money::new(inputs[(j + rotation) % n].clone(), currency),
                        )
                    })
                    .collect();
                if reversed {
                    flows.reverse();
                }
                let got = aggregate_cash_flows(&flows, n, limit);
                if let Some(ref value) = expected {
                    let actual=got.unwrap_or_else(|error|
                        panic!("exact representable case {index}, rotation={rotation}, reversed={reversed}, {error:?}"));
                    assert_eq!(actual.len(), 1);
                    assert_eq!(
                        actual[0].money().amount(),
                        value,
                        "case {index}, rotation={rotation}, reversed={reversed}"
                    );
                    assert_eq!(
                        actual[0].money().amount().exponent(),
                        value.exponent(),
                        "case {index}, output quantum"
                    );
                } else {
                    assert!(
                        matches!(got, Err(FinanceError::Decimal(_))),
                        "expected coefficient-limit refusal, case {index}, got {got:?}"
                    );
                }
            }
        }
        if expected.is_some() {
            valid += 1;
        } else {
            refused += 1;
        }
    }
    assert_eq!((valid, refused), (48, 40));
}
