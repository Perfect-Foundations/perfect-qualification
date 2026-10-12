use std::hint::black_box;
use std::time::Instant;

#[derive(Clone, Copy)]
struct Case {
    label: &'static str,
    n: usize,
    rho: f64,
    family: u8,
}
const CASES: [Case; 4] = [
    Case { label: "4d-independent", n: 4, rho: 0.0, family: 0 },
    Case { label: "8d-half-correlation", n: 8, rho: 0.5, family: 1 },
    Case { label: "4d-near-singular", n: 4, rho: f64::from_bits(0x3fefffffffffffff), family: 2 },
    Case { label: "8d-near-singular", n: 8, rho: 1.0 - 1.0 / 1099511627776.0, family: 3 },
];
fn make_contributions(case: Case) -> Vec<f64> {
    match case.family {
        0 => vec![3.0, -4.0, 5.0, -2.0],
        1 => (0..case.n).map(|i| if i % 2 == 0 { (i+1) as f64 } else { -((i+1) as f64) }).collect(),
        2 => vec![1e8, -1e8, -1.0, 0.0],
        _ => (0..case.n).map(|i| if i % 2 == 0 { 1e8 } else { -1e8 }).collect(),
    }
}
fn matrix(case: Case) -> Vec<f64> {
    (0..case.n).flat_map(|i| (0..case.n).map(move |j| if i == j { 1.0 } else { case.rho })).collect()
}
fn median_nanos(mut samples: Vec<f64>) -> f64 {
    samples.sort_by(|a, b| a.total_cmp(b));
    samples[samples.len()/2]
}
fn run_old(case: Case, iterations: usize) -> (f64, f64) {
    let matrix=previous::CorrelationMatrix::new(case.n, matrix(case), previous::CorrelationPolicy {max_dimension:8,psd_tolerance:1e-12}).unwrap();
    let input=make_contributions(case);
    let mut variance=0.0;
    let mut trials=Vec::new();
    for pass in 0..6 {
        let start=Instant::now();
        for _ in 0..iterations {
            variance = black_box(previous::propagate_first_order(black_box(&input), black_box(&matrix)).unwrap()).variance();
        }
        let elapsed=start.elapsed().as_secs_f64()*1e9/iterations as f64;
        if pass>0 {trials.push(elapsed);}
    }
    (median_nanos(trials),variance)
}
fn run_new(case: Case, iterations: usize) -> (f64, f64) {
    let matrix=corrected::CorrelationMatrix::new(case.n, matrix(case), corrected::CorrelationPolicy {max_dimension:8,psd_tolerance:1e-12}).unwrap();
    let input=make_contributions(case);
    let mut variance=0.0;
    let mut trials=Vec::new();
    for pass in 0..6 {
        let start=Instant::now();
        for _ in 0..iterations {
            variance = black_box(corrected::propagate_first_order(black_box(&input), black_box(&matrix)).unwrap()).variance();
        }
        let elapsed=start.elapsed().as_secs_f64()*1e9/iterations as f64;
        if pass>0 {trials.push(elapsed);}
    }
    (median_nanos(trials),variance)
}
fn main() {
    let iterations=20_000;
    println!("cases={} iterations={} repeated_samples=5 release=true",CASES.len(),iterations);
    for case in CASES {
        let (baseline_ns, baseline_variance)=run_old(case,iterations);
        let (current_ns,current_variance)=run_new(case,iterations);
        println!("{} old_ns={:.2} new_ns={:.2} ratio_new_over_old={:.3} old_var={:.17e} new_var={:.17e}",
            case.label,baseline_ns,current_ns,current_ns/baseline_ns,baseline_variance,current_variance);
    }
}
