//! Deterministic multi-seed ledger state-machine reference without a second
//! production aggregator. The model owns ID/amount/date/currency tuples and
//! computes group sums using bounded primitive i64 arithmetic.
use perfect_finance::{
    CashFlow, CashFlowId, CurrencyCode, Date, DateBoundary, DateRange, Decimal, DecimalLimits,
    FinanceError, LedgerOperation, Money, Month, OwnedCashFlowLedger, RangeQueryLimits,
};
use std::collections::BTreeMap;
#[derive(Clone)]
struct Row {
    id: u64,
    amount: i64,
    date: Date,
    currency: CurrencyCode,
    flow: CashFlow,
}
fn xorshift(s: &mut u64) -> u64 {
    *s ^= *s >> 12;
    *s ^= *s << 25;
    *s ^= *s >> 27;
    s.wrapping_mul(0x2545f4914f6cdd1d)
}
fn date(x: u64) -> Date {
    let (month, day) = match x % 5 {
        0 => (Month::February, 27),
        1 => (Month::February, 28),
        2 => (Month::February, 29),
        3 => (Month::March, 1),
        _ => (Month::March, 2),
    };
    Date::from_calendar_date(2024, month, day).unwrap()
}
fn currency(x: u64) -> CurrencyCode {
    CurrencyCode::new(match x % 3 {
        0 => "USD",
        1 => "EUR",
        _ => "JPY",
    })
    .unwrap()
}
fn amount(n: i64, lim: DecimalLimits) -> Decimal {
    Decimal::parse(&n.to_string(), lim).unwrap()
}
fn row(id: u64, n: i64, d: Date, c: CurrencyCode, lim: DecimalLimits) -> Row {
    Row {
        id,
        amount: n,
        date: d,
        currency: c,
        flow: CashFlow::new(d, Money::new(amount(n, lim), c)),
    }
}
fn assert_state(
    book: &OwnedCashFlowLedger,
    reference: &[Row],
    limits: DecimalLimits,
    range: DateRange,
    step: usize,
    seed: u64,
) {
    assert_eq!(
        book.len(),
        reference.len(),
        "seed={seed:x},step={step}: length"
    );
    for (i, r) in reference.iter().enumerate() {
        assert_eq!(book.ids()[i].get(), r.id, "seed={seed:x},step={step}: id");
        assert_eq!(book.flows()[i], r.flow, "seed={seed:x},step={step}: flow");
    }
    let mut grouped = BTreeMap::<(Date, CurrencyCode), i64>::new();
    let mut net = BTreeMap::<CurrencyCode, i64>::new();
    for r in reference.iter().filter(|r| range.contains(r.date)) {
        *grouped.entry((r.date, r.currency)).or_default() += r.amount;
        *net.entry(r.currency).or_default() += r.amount;
    }
    let mut view = book.indexed_view().unwrap();
    let actual = view.aggregate(Some(&range)).unwrap();
    assert_eq!(
        actual.len(),
        grouped.len(),
        "seed={seed:x},step={step}: groups"
    );
    for (a, ((d, c), n)) in actual.iter().zip(&grouped) {
        assert_eq!(a.date(), *d);
        assert_eq!(a.money().currency(), *c);
        assert_eq!(a.money().amount(), &amount(*n, limits));
    }
    let totals = view.currency_totals(Some(&range)).unwrap();
    assert_eq!(totals.len(), net.len(), "seed={seed:x},step={step}: totals");
    for (a, (c, n)) in totals.iter().zip(&net) {
        assert_eq!(a.currency(), *c);
        assert_eq!(a.amount(), &amount(*n, limits));
    }
    let ranges = [range, range];
    let mut output = vec![vec![reference.first().map_or_else(
        || row(0, 0, date(0), currency(0), limits).flow,
        |r| r.flow.clone(),
    )]];
    let before = output.clone();
    assert!(matches!(
        view.aggregate_ranges_into(
            &ranges,
            RangeQueryLimits {
                max_ranges: 1,
                max_total_groups: 100
            },
            &mut output
        ),
        Err(FinanceError::QueryLimit {
            dimension: "ranges",
            ..
        })
    ));
    assert_eq!(output, before);
    let limits = RangeQueryLimits {
        max_ranges: 2,
        max_total_groups: 100,
    };
    view.aggregate_ranges_into(&ranges, limits, &mut output)
        .unwrap();
    assert_eq!(output, vec![actual.clone(), actual]);
}
#[test]
fn three_seed_families_with_1800_atomic_and_query_states() {
    const SEEDS: [u64; 3] = [
        0x24a1_fade_7339_0041,
        0x2469_10fa_4400_0025,
        0x2499_900d_00ff_a110,
    ];
    let lim = DecimalLimits::new(9).unwrap();
    let max = 12usize;
    let range = DateRange::new(
        date(1),
        date(4),
        DateBoundary::Inclusive,
        DateBoundary::Inclusive,
    )
    .unwrap();
    for seed in SEEDS {
        let mut rng = seed;
        let first = row(0, 3, date(2), currency(1), lim);
        let mut book =
            OwnedCashFlowLedger::new(std::slice::from_ref(&first.flow), max, lim).unwrap();
        let mut model = vec![first];
        let mut next_id = 1_u64;
        let mut retired = Vec::<CashFlowId>::new();
        for step in 0..600 {
            let operation = xorshift(&mut rng) % 11;
            let n = (xorshift(&mut rng) % 41) as i64 - 20;
            let d = date(xorshift(&mut rng));
            let cur = currency(xorshift(&mut rng));
            let candidate = row(next_id, n, d, cur, lim);
            let index = if model.is_empty() {
                0
            } else {
                (xorshift(&mut rng) as usize) % model.len()
            };
            match operation {
                0..=2 => {
                    let prior = book.flows().to_vec();
                    let ids = book.ids().to_vec();
                    let r = book.append(std::slice::from_ref(&candidate.flow));
                    if model.len() == max {
                        assert!(matches!(r, Err(FinanceError::ResourceLimit { .. })));
                        assert_eq!(book.flows(), prior);
                        assert_eq!(book.ids(), ids);
                    } else {
                        let id = r.unwrap()[0];
                        assert_eq!(id.get(), next_id);
                        model.push(candidate);
                        next_id += 1;
                    }
                }
                3 if !model.is_empty() => {
                    let id = book.ids()[index];
                    book.replace(id, candidate.flow.clone()).unwrap();
                    model[index] = Row {
                        id: id.get(),
                        ..candidate
                    };
                }
                4 if !model.is_empty() => {
                    let id = book.ids()[index];
                    assert_eq!(book.remove(id).unwrap(), model.remove(index).flow);
                    retired.push(id);
                }
                5 if model.len() >= 2 && model.len() < max => {
                    let second = (index + 1) % model.len();
                    let first_id = book.ids()[index];
                    let second_id = book.ids()[second];
                    let added = row(next_id, n + 1, d, cur, lim);
                    let ids = book
                        .apply_operations(
                            &[
                                LedgerOperation::Append(&added.flow),
                                LedgerOperation::Replace(first_id, &candidate.flow),
                                LedgerOperation::Remove(second_id),
                            ],
                            3,
                        )
                        .unwrap();
                    assert_eq!(ids[0].get(), next_id);
                    model.push(added);
                    model[index] = Row {
                        id: first_id.get(),
                        ..candidate
                    };
                    model.remove(second);
                    retired.push(second_id);
                    next_id += 1;
                }
                6 if !model.is_empty() => {
                    let id = book.ids()[index];
                    let before = book.flows().to_vec();
                    let ids = book.ids().to_vec();
                    assert!(matches!(
                        book.apply_operations(
                            &[
                                LedgerOperation::Replace(id, &candidate.flow),
                                LedgerOperation::Remove(id),
                            ],
                            2
                        ),
                        Err(FinanceError::DuplicateCashFlowOperation { .. })
                    ));
                    assert_eq!(book.flows(), before);
                    assert_eq!(book.ids(), ids);
                }
                7 if !retired.is_empty() => {
                    let index = (xorshift(&mut rng) as usize) % retired.len();
                    let old = retired[index];
                    let before = book.flows().to_vec();
                    let ids = book.ids().to_vec();
                    // A stable retired ID cannot be constructed publicly;
                    // obtain it only while it remains live in this path.
                    assert!(matches!(
                        book.remove(old),
                        Err(FinanceError::UnknownCashFlowId { .. })
                    ));
                    assert!(
                        book.apply_operations(&[LedgerOperation::Append(&candidate.flow)], 0)
                            .is_err()
                    );
                    assert_eq!(book.flows(), before);
                    assert_eq!(book.ids(), ids);
                }
                8 => {
                    let n2 = (xorshift(&mut rng) % 41) as i64 - 20;
                    let replacement = row(next_id, n2, d, cur, lim);
                    let prior_ids = book.ids().to_vec();
                    let newids = book.reset(std::slice::from_ref(&replacement.flow)).unwrap();
                    assert_eq!(newids[0].get(), next_id);
                    retired.extend(prior_ids);
                    model = vec![replacement];
                    next_id += 1;
                }
                9 => {
                    let bad = CashFlow::new(
                        d,
                        Money::new(
                            Decimal::parse("12345678901", DecimalLimits::new(15).unwrap()).unwrap(),
                            cur,
                        ),
                    );
                    let before = book.flows().to_vec();
                    let ids = book.ids().to_vec();
                    assert!(book.append(std::slice::from_ref(&bad)).is_err());
                    assert_eq!(book.flows(), before);
                    assert_eq!(book.ids(), ids);
                }
                _ => {
                    let before = book.flows().to_vec();
                    assert!(
                        book.apply_operations(&[LedgerOperation::Append(&candidate.flow)], 0)
                            .is_err()
                    );
                    assert_eq!(book.flows(), before);
                }
            }
            assert_state(&book, &model, lim, range, step, seed);
        }
    }
}
