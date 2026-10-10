//! Bounded exploratory exact Polynomial resource matrix.
//! These counters measure requested allocator bytes during ONE complete public operation,
//! not RSS or a formal heap/stack upper bound. This is qualification-only unsafe.
use perfect_arithmetic::{Integer, Natural};
use perfect_polynomial::{IntegerPolynomial as Z, RationalPolynomial as Q};
use perfect_rational::Rational;
use std::alloc::{GlobalAlloc, Layout, System};
use std::hint::black_box;
use std::sync::atomic::{AtomicBool, AtomicI64, AtomicU64, Ordering::Relaxed};
use std::time::Instant;

struct Meter;
static ACTIVE: AtomicBool = AtomicBool::new(false);
static ALLOCS: AtomicU64 = AtomicU64::new(0);
static REALLOCS: AtomicU64 = AtomicU64::new(0);
static REQUESTED: AtomicU64 = AtomicU64::new(0);
static LIVE: AtomicI64 = AtomicI64::new(0);
static PEAK: AtomicI64 = AtomicI64::new(0);
fn record(delta: i64, request: u64, allocate: bool, reallocate: bool) {
    if !ACTIVE.load(Relaxed) {
        return;
    }
    if allocate {
        ALLOCS.fetch_add(1, Relaxed);
    }
    if reallocate {
        REALLOCS.fetch_add(1, Relaxed);
    }
    REQUESTED.fetch_add(request, Relaxed);
    PEAK.fetch_max(LIVE.fetch_add(delta, Relaxed) + delta, Relaxed);
}
unsafe impl GlobalAlloc for Meter {
    unsafe fn alloc(&self, l: Layout) -> *mut u8 {
        let p = unsafe { System.alloc(l) };
        if !p.is_null() {
            record(l.size() as i64, l.size() as u64, true, false);
        }
        p
    }
    unsafe fn dealloc(&self, p: *mut u8, l: Layout) {
        record(-(l.size() as i64), 0, false, false);
        unsafe { System.dealloc(p, l) }
    }
    unsafe fn realloc(&self, p: *mut u8, l: Layout, new_size: usize) -> *mut u8 {
        let v = unsafe { System.realloc(p, l, new_size) };
        if !v.is_null() {
            record(
                new_size as i64 - l.size() as i64,
                new_size as u64,
                false,
                true,
            );
        }
        v
    }
}
#[global_allocator]
static GLOBAL: Meter = Meter;

fn observed(label: &str, a_len: usize, b_len: usize, bits: u64, mut operation: impl FnMut()) {
    // Inputs are constructed outside the measured window.
    operation();
    ALLOCS.store(0, Relaxed);
    REALLOCS.store(0, Relaxed);
    REQUESTED.store(0, Relaxed);
    LIVE.store(0, Relaxed);
    PEAK.store(0, Relaxed);
    let now = Instant::now();
    ACTIVE.store(true, Relaxed);
    operation();
    ACTIVE.store(false, Relaxed);
    let elapsed = now.elapsed().as_nanos();
    let live = LIVE.load(Relaxed);
    assert_eq!(live, 0, "{label} leaked incremental requested bytes");
    println!(
        "{label},{a_len},{b_len},{bits},{elapsed},{},{},{},{},{}",
        ALLOCS.load(Relaxed),
        REALLOCS.load(Relaxed),
        REQUESTED.load(Relaxed),
        PEAK.load(Relaxed),
        live
    );
}
fn magnitude(bits: u64) -> Integer {
    assert!((1..=1024).contains(&bits));
    Integer::from(Natural::one().shl_bits(bits - 1))
}
fn integer_inputs(degree_a: usize, degree_b: usize, bits: u64, mode: &str) -> (Z, Z) {
    let v = magnitude(bits);
    let make = |degree: usize, flip: bool| {
        let c = (0..=degree)
            .map(|i| {
                let selected = match mode {
                    "dense" => true,
                    "sparse" => i == 0 || i == degree,
                    "monomial" => i == degree,
                    "cancellation" => true,
                    _ => panic!("unknown input mode"),
                };
                if !selected {
                    Integer::zero()
                } else if (mode == "cancellation" && i % 2 == 0) || (flip && i % 3 == 0) {
                    v.neg()
                } else {
                    v.clone()
                }
            })
            .collect();
        Z::new(c)
    };
    (make(degree_a, false), make(degree_b, true))
}
fn sum_coefficients(p: &Z) -> Integer {
    p.coefficients()
        .iter()
        .fold(Integer::zero(), |acc, x| acc.add(x))
}
fn measure_integer(degree_a: usize, degree_b: usize, bits: u64, mode: &str) {
    let (a, b) = integer_inputs(degree_a, degree_b, bits, mode);
    let expected = sum_coefficients(&a).mul(&sum_coefficients(&b));
    let label = format!("z_{mode}_d{degree_a}x{degree_b}_b{bits}");
    let reference = a.mul(&b);
    assert_eq!(reference.degree(), Some(degree_a + degree_b));
    assert_eq!(
        reference.evaluate(&Integer::one()),
        expected,
        "independent evaluation identity {label}"
    );
    observed(
        &label,
        a.coefficients().len(),
        b.coefficients().len(),
        bits,
        || {
            let product = black_box(&a).mul(black_box(&b));
            black_box(&product);
            drop(product);
        },
    );
}
fn make_rational(n: Integer, d: Natural) -> Rational {
    Rational::new(n, Integer::from(d)).unwrap()
}
fn rational_coeffs(bits: u64, mode: &str) -> (Q, Q) {
    let d = Natural::one().shl_bits(bits - 1);
    assert_eq!(d.bit_length(), bits);
    let one = Integer::one();
    let a = match mode {
        "uniform" => vec![
            make_rational(one.clone(), d.clone()),
            make_rational(one.neg(), d.clone()),
            make_rational(one.clone(), d.clone()),
            make_rational(one.clone(), d.clone()),
        ],
        "distinct" => vec![
            make_rational(one.clone(), d.clone()),
            make_rational(Integer::from(-1), Natural::from(2_u8)),
            Rational::zero(),
            make_rational(one.clone(), d.clone()),
        ],
        _ => panic!("bad rational mode"),
    };
    let b = if mode == "uniform" {
        vec![
            make_rational(Integer::from(1), Natural::from(3_u8)),
            make_rational(Integer::from(-2), Natural::from(3_u8)),
            make_rational(Integer::from(4), Natural::from(3_u8)),
            make_rational(Integer::from(5), Natural::from(3_u8)),
        ]
    } else {
        vec![
            make_rational(Integer::from(1), Natural::from(3_u8)),
            Rational::from(Integer::from(-1)),
            Rational::zero(),
            make_rational(Integer::from(1), Natural::from(5_u8)),
        ]
    };
    if mode == "uniform" {
        assert!(a.iter().all(|c| c.denominator() == a[0].denominator()));
        assert!(b.iter().all(|c| c.denominator() == b[0].denominator()));
    }
    (Q::new(a), Q::new(b))
}
fn sum_rational_coefficients(p: &Q) -> Rational {
    p.coefficients()
        .iter()
        .fold(Rational::zero(), |acc, x| acc.add(x))
}
fn measure_rational(bits: u64, mode: &str) {
    let (a, b) = rational_coeffs(bits, mode);
    let expected = sum_rational_coefficients(&a).mul(&sum_rational_coefficients(&b));
    let label = format!("q_{mode}_lcm_b{bits}");
    let reference = a.mul(&b);
    assert_eq!(reference.degree(), Some(6));
    assert_eq!(
        reference.evaluate(&Rational::one()),
        expected,
        "independent evaluation identity {label}"
    );
    observed(
        &label,
        a.coefficients().len(),
        b.coefficients().len(),
        bits,
        || {
            let product = black_box(&a).mul(black_box(&b));
            black_box(&product);
            drop(product);
        },
    );
}
fn measure_prs_guards() {
    let huge = Integer::from(Natural::one().shl_bits(2100));
    let early_a = Z::new(vec![
        huge.clone(),
        Integer::zero(),
        Integer::zero(),
        Integer::zero(),
        Integer::one(),
    ]);
    let early_b = Z::new(vec![Integer::one(), Integer::zero(), Integer::zero(), huge]);
    let one = Z::new(vec![Integer::one()]);
    assert_eq!(early_a.gcd(&early_b).unwrap(), one);
    observed(
        "gcd_early_4096_guard",
        early_a.coefficients().len(),
        early_b.coefficients().len(),
        2101,
        || {
            let result = black_box(&early_a).gcd(black_box(&early_b)).unwrap();
            black_box(&result);
            drop(result);
        },
    );
    let huge_coeff = Integer::from(Natural::one().shl_bits(4097));
    let width = Z::new(vec![
        Integer::one(),
        Integer::from(-3),
        Integer::from(2),
        huge_coeff,
    ]);
    assert_eq!(width.gcd(&width).unwrap(), width);
    observed(
        "gcd_input_width_4098",
        width.coefficients().len(),
        width.coefficients().len(),
        4098,
        || {
            let result = black_box(&width).gcd(black_box(&width)).unwrap();
            black_box(&result);
            drop(result);
        },
    );
    let mut dense = vec![Integer::zero(); 34];
    dense[0] = Integer::from(2);
    dense[33] = Integer::from(-6);
    let degree = Z::new(dense);
    let normalized = degree.neg();
    assert_eq!(degree.gcd(&degree).unwrap(), normalized);
    observed(
        "gcd_degree33_refusal",
        degree.coefficients().len(),
        degree.coefficients().len(),
        3,
        || {
            let result = black_box(&degree).gcd(black_box(&degree)).unwrap();
            black_box(&result);
            drop(result);
        },
    );
    const A: [i128; 10] = [
        29878539771007619802695925865,
        38337129496435065462512069061,
        26176432220010364463952772943,
        36455492137305517204742188554,
        -31583172706667982753055670592,
        24146646319036245156938719383,
        -33042054106061461237049169535,
        27587738088759190405566614755,
        39124099768788597505792466446,
        37241909747421405995232538844,
    ];
    const B: [i128; 9] = [
        -32510778393112134553770803457,
        22708147360007579159236093779,
        20811411071155743672883378584,
        29928754231223780915951098300,
        -34504213327522140517185037813,
        -36255739560194229515569692825,
        -32845031363358562485482133337,
        31138875340773139053657760992,
        -29878475348932783589686397830,
    ];
    let a = Z::new(A.into_iter().map(Integer::from).collect());
    let b = Z::new(B.into_iter().map(Integer::from).collect());
    assert_eq!(a.gcd(&b).unwrap(), one);
    observed(
        "gcd_late_prs_resume",
        a.coefficients().len(),
        b.coefficients().len(),
        106,
        || {
            let result = black_box(&a).gcd(black_box(&b)).unwrap();
            black_box(&result);
            drop(result);
        },
    );
    let qa = Q::new(
        a.coefficients()
            .iter()
            .cloned()
            .map(Rational::from)
            .collect(),
    );
    let qb = Q::new(
        b.coefficients()
            .iter()
            .cloned()
            .map(Rational::from)
            .collect(),
    );
    assert_eq!(qa.gcd(&qb).unwrap(), Q::new(vec![Rational::one()]));
    observed(
        "gcd_late_prs_q",
        qa.coefficients().len(),
        qb.coefficients().len(),
        106,
        || {
            let result = black_box(&qa).gcd(black_box(&qb)).unwrap();
            black_box(&result);
            drop(result);
        },
    );
}

fn main() {
    println!(
        "case,len_a,len_b,input_bits,elapsed_ns,allocs,reallocs,total_requested_bytes,peak_incremental_live_bytes,net_live_delta"
    );
    // Eight bounded shape/width points; worst-case dense 256-by-256 at just 32 bits.
    for &(a, b, bits, mode) in &[
        (8, 8, 32, "dense"),
        (32, 32, 128, "dense"),
        (64, 64, 256, "dense"),
        (128, 128, 32, "dense"),
        (256, 256, 32, "dense"),
        (512, 512, 32, "sparse"),
        (512, 512, 32, "monomial"),
        (8, 256, 128, "sparse"),
        (64, 64, 256, "cancellation"),
    ] {
        measure_integer(a, b, bits, mode);
    }
    // Rational exact eligibility boundary: 512 bits eligible; 513+ refused.
    for &bits in &[256, 511, 512, 513, 768, 1024] {
        measure_rational(bits, "distinct");
    }
    measure_rational(256, "uniform");
    measure_integer(8, 8, 1024, "dense");
    measure_integer(32, 32, 1024, "sparse");
    measure_prs_guards();
}
