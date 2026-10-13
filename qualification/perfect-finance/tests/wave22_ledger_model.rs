//! Seeded deterministic ledger reference model and exact query checks.
use perfect_finance::{
    CashFlow, CurrencyCode, Date, DateBoundary, DateRange, Decimal, DecimalLimits, FinanceError,
    LedgerOperation, Money, Month, OwnedCashFlowLedger, aggregate_cash_flows_in_range,
};
use std::collections::BTreeMap;
const SEED: u64 = 0x220fface77dd3311;
fn next(s: &mut u64) -> u64 {
    *s ^= *s >> 12;
    *s ^= *s << 25;
    *s ^= *s >> 27;
    s.wrapping_mul(0x2545f4914f6cdd1d)
}
fn flow(day: u8, currency: &str, value: i64, limits: DecimalLimits) -> CashFlow {
    CashFlow::new(
        Date::from_calendar_date(2024, Month::February, day).unwrap(),
        Money::new(
            Decimal::parse(&value.to_string(), limits).unwrap(),
            CurrencyCode::new(currency).unwrap(),
        ),
    )
}
#[test]
fn seeded_400_step_ledger_model_and_exact_date_currency_sums() {
    let limits = DecimalLimits::new(8).unwrap();
    let first = flow(28, "USD", 1, limits);
    let mut book = OwnedCashFlowLedger::new(core::slice::from_ref(&first), 40, limits).unwrap();
    let mut reference = vec![(book.ids()[0], first)];
    let mut next_id = 1_u64;
    let mut seed = SEED;
    for step in 0..400 {
        let operation = next(&mut seed) % 9;
        let date = if next(&mut seed).is_multiple_of(2) {
            28
        } else {
            29
        };
        let currency = if next(&mut seed).is_multiple_of(2) {
            "USD"
        } else {
            "EUR"
        };
        let amount = (next(&mut seed) % 21) as i64 - 10;
        let value = flow(date, currency, amount, limits);
        let idx = if reference.is_empty() {
            0
        } else {
            (next(&mut seed) as usize) % reference.len()
        };
        match operation {
            0..=2 => {
                let before = book.flows().to_vec();
                let ids = book.ids().to_vec();
                let outcome = book.append(core::slice::from_ref(&value));
                if reference.len() < 40 {
                    let id = outcome.unwrap()[0];
                    assert_eq!(id.get(), next_id);
                    next_id += 1;
                    reference.push((id, value));
                } else {
                    assert!(matches!(outcome, Err(FinanceError::ResourceLimit { .. })));
                    assert_eq!(book.flows(), before);
                    assert_eq!(book.ids(), ids);
                }
            }
            3 | 4 if !reference.is_empty() => {
                let id = reference[idx].0;
                book.replace(id, value.clone()).unwrap();
                reference[idx].1 = value;
            }
            5 if !reference.is_empty() => {
                let (id, removed) = reference.remove(idx);
                assert_eq!(book.remove(id).unwrap(), removed);
            }
            6 if !reference.is_empty() => {
                let id = reference[idx].0;
                let old = book.flows().to_vec();
                let ids = book.ids().to_vec();
                assert!(matches!(
                    book.apply_operations(
                        &[
                            LedgerOperation::Replace(id, &value),
                            LedgerOperation::Remove(id)
                        ],
                        2
                    ),
                    Err(FinanceError::DuplicateCashFlowOperation { .. })
                ));
                assert_eq!(book.flows(), old);
                assert_eq!(book.ids(), ids);
            }
            7 if !reference.is_empty() => {
                let id = reference[idx].0;
                let outcome = book.apply_operations(
                    &[
                        LedgerOperation::Replace(id, &value),
                        LedgerOperation::Remove(id),
                    ],
                    1,
                );
                assert!(matches!(outcome, Err(FinanceError::QueryLimit { .. })));
            }
            _ => {
                let entries = [value.clone()];
                let old = book.flows().to_vec();
                let outcome = book.reset(&entries);
                assert!(outcome.is_ok());
                assert_ne!(book.ids()[0].get(), 0);
                assert_eq!(book.flows(), entries);
                reference = book.ids().iter().copied().zip(entries).collect();
                assert_eq!(reference.len(), 1);
                assert!(old.len() <= 40);
                next_id = reference[0].0.get() + 1;
            }
        }
        assert_eq!(
            book.ids().len(),
            reference.len(),
            "seed={SEED:x}, step={step}"
        );
        assert_eq!(book.flows().len(), reference.len());
        for (index, (id, v)) in reference.iter().enumerate() {
            assert_eq!(book.ids()[index], *id, "step={step}");
            assert_eq!(&book.flows()[index], v, "step={step}");
            assert_eq!(book.get(*id), Some(v));
        }
        let day28 = Date::from_calendar_date(2024, Month::February, 28).unwrap();
        let day29 = Date::from_calendar_date(2024, Month::February, 29).unwrap();
        let range = DateRange::new(
            day28,
            day29,
            DateBoundary::Inclusive,
            DateBoundary::Inclusive,
        )
        .unwrap();
        let actual = aggregate_cash_flows_in_range(book.flows(), &range, 40, limits).unwrap();
        let mut expected = BTreeMap::<(Date, CurrencyCode), i64>::new();
        for (_, entry) in &reference {
            let key = (entry.date(), entry.money().currency());
            let amount = entry.money().amount();
            let integer = (-10..=10)
                .find(|&n| amount == &Decimal::parse(&n.to_string(), limits).unwrap())
                .unwrap();
            *expected.entry(key).or_default() += integer;
        }
        assert_eq!(actual.len(), expected.len(), "step={step}: distinct groups");
        for (item, ((date, currency), sum)) in actual.iter().zip(expected.iter()) {
            assert_eq!(item.date(), *date);
            assert_eq!(item.money().currency(), *currency);
            assert_eq!(
                item.money().amount(),
                &Decimal::parse(&sum.to_string(), limits).unwrap(),
                "step={step}"
            );
        }
    }
}
