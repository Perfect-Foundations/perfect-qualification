use std::{hint::black_box, time::Instant};

#[derive(Clone, Copy)]
struct Case {
    label: &'static str,
    digits: u64,
    kind: u8,
}
const CASES: [Case; 4] = [
    Case {
        label: "3d-common",
        digits: 3,
        kind: 0,
    },
    Case {
        label: "4d-cancellation",
        digits: 3,
        kind: 1,
    },
    Case {
        label: "128d-sparse-exponents",
        digits: 3,
        kind: 2,
    },
    Case {
        label: "3d-pre-fix-failure",
        digits: 2,
        kind: 3,
    },
];
impl Case {
    fn values(self) -> Vec<&'static str> {
        match self.kind {
            0 => vec!["900", "-900", "900"],
            1 => vec!["0.005", "0.005", "-0.005", "0.005"],
            2 => {
                let mut v = Vec::with_capacity(128);
                for _ in 0..63 {
                    v.extend(["1e10000", "-1e10000"]);
                }
                v.extend(["0.005", "-0.001"]);
                v
            }
            _ => vec!["0.035", "-0.23", "0.097"],
        }
    }
}
fn median(mut samples: Vec<f64>) -> f64 {
    samples.sort_by(f64::total_cmp);
    samples[samples.len() / 2]
}
macro_rules! measure {
    ($package:ident,$case:expr,$repetitions:expr) => {{
        let limits = $package::DecimalLimits::new($case.digits).unwrap();
        let date = $package::Date::from_calendar_date(2026, $package::Month::October, 11).unwrap();
        let code = $package::CurrencyCode::new("USD").unwrap();
        let flows: Vec<_> = $case
            .values()
            .iter()
            .map(|value| {
                $package::CashFlow::new(
                    date,
                    $package::Money::new($package::Decimal::parse(value, limits).unwrap(), code),
                )
            })
            .collect();
        let mut samples = Vec::new();
        let mut accepted = false;
        for pass in 0..6 {
            let start = Instant::now();
            for _ in 0..$repetitions {
                let outcome = $package::aggregate_cash_flows(
                    black_box(&flows),
                    flows.len(),
                    black_box(limits),
                );
                accepted = black_box(outcome.is_ok());
                let _ = black_box(outcome);
            }
            if pass > 0 {
                samples.push(start.elapsed().as_secs_f64() * 1e9 / $repetitions as f64);
            }
        }
        let expected = $package::aggregate_cash_flows(&flows, flows.len(), limits);
        let final_amount = match expected {
            Ok(values) => values[0].money().amount().to_string(),
            Err(error) => format!("{error:?}"),
        };
        (median(samples), accepted, final_amount)
    }};
}
fn main() {
    let count = 1000;
    println!(
        "compiler=rustc-1.99 optimized=true calls_per_workload={count} warmup=1 repetitions=5"
    );
    for case in CASES {
        let (before, old_ok, old_value) = measure!(baseline, case, count);
        let (after, new_ok, new_value) = measure!(corrected, case, count);
        println!(
            "{} n={} old_ns={:.2} new_ns={:.2} ratio={:.3} old_ok={} new_ok={} old_value={} new_value={}",
            case.label,
            case.values().len(),
            before,
            after,
            after / before,
            old_ok,
            new_ok,
            old_value,
            new_value
        );
    }
}
