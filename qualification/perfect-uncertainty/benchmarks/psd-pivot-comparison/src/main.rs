use std::{hint::black_box, time::Instant};
#[derive(Clone, Copy)]
struct Case {
    label: &'static str,
    n: usize,
    kind: u8,
}
const CASES: [Case; 4] = [
    Case {
        label: "4d-independent",
        n: 4,
        kind: 0,
    },
    Case {
        label: "8d-half-correlation",
        n: 8,
        kind: 1,
    },
    Case {
        label: "16d-rank-one",
        n: 16,
        kind: 2,
    },
    Case {
        label: "18d-previous-false-rejection",
        n: 18,
        kind: 3,
    },
];
fn correlation(case: Case) -> Vec<f64> {
    let n = case.n;
    let mut result = vec![0.0; n * n];
    let masks: [u16; 18] = [
        24810, 32994, 42583, 34078, 14413, 18809, 11555, 51663, 56833, 28018, 424, 29403, 47595,
        16982, 55178, 23491, 35799, 33643,
    ];
    for i in 0..n {
        for j in 0..n {
            result[i * n + j] = match case.kind {
                0 => {
                    if i == j {
                        1.0
                    } else {
                        0.0
                    }
                }
                1 => {
                    if i == j {
                        1.0
                    } else {
                        0.5
                    }
                }
                2 => 1.0,
                _ => f64::from(16 - 2 * (masks[i] ^ masks[j]).count_ones() as i32) / 16.0,
            };
        }
    }
    result
}
fn median(mut samples: Vec<f64>) -> f64 {
    samples.sort_by(f64::total_cmp);
    samples[samples.len() / 2]
}
fn old(case: Case, matrix: &[f64], repetitions: usize) -> (f64, bool) {
    let mut samples = Vec::new();
    let mut accepted = false;
    for pass in 0..6 {
        let start = Instant::now();
        for _ in 0..repetitions {
            let res = baseline::CorrelationMatrix::new(
                case.n,
                black_box(matrix).to_vec(),
                baseline::CorrelationPolicy {
                    max_dimension: 32,
                    psd_tolerance: 1e-12,
                },
            );
            accepted = black_box(res.is_ok());
            let _ = black_box(res);
        }
        if pass != 0 {
            samples.push(start.elapsed().as_secs_f64() * 1e9 / repetitions as f64);
        }
    }
    (median(samples), accepted)
}
fn new(case: Case, matrix: &[f64], repetitions: usize) -> (f64, bool) {
    let mut samples = Vec::new();
    let mut accepted = false;
    for pass in 0..6 {
        let start = Instant::now();
        for _ in 0..repetitions {
            let res = pivoted::CorrelationMatrix::new(
                case.n,
                black_box(matrix).to_vec(),
                pivoted::CorrelationPolicy {
                    max_dimension: 32,
                    psd_tolerance: 1e-12,
                },
            );
            accepted = black_box(res.is_ok());
            let _ = black_box(res);
        }
        if pass != 0 {
            samples.push(start.elapsed().as_secs_f64() * 1e9 / repetitions as f64);
        }
    }
    (median(samples), accepted)
}
fn main() {
    let repetitions = 1000;
    println!("rustc=1.99 release=true repetitions={repetitions} measured_passes=5 warmups=1");
    for case in CASES {
        let values = correlation(case);
        let (before, old_ok) = old(case, &values, repetitions);
        let (after, new_ok) = new(case, &values, repetitions);
        println!(
            "{} n={} baseline_ns={:.2} pivoted_ns={:.2} ratio={:.3} baseline_ok={} pivoted_ok={}",
            case.label,
            case.n,
            before,
            after,
            after / before,
            old_ok,
            new_ok
        );
    }
}
