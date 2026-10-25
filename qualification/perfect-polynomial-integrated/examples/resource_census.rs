//! Benchmark-only allocator accounting; NOT production Polynomial code.
//! Peak is incremental live requested allocator bytes during one call,
//! not resident set size, malloc physical footprint, or a stack bound.
use perfect_arithmetic::Integer;
use perfect_polynomial::{IntegerPolynomial as Z, RationalPolynomial as Q};
use perfect_rational::Rational;
use std::alloc::{GlobalAlloc, Layout, System};
use std::hint::black_box;
use std::sync::atomic::{AtomicBool, AtomicI64, AtomicU64, Ordering::Relaxed};

struct Meter;
static ACTIVE: AtomicBool = AtomicBool::new(false);
static ALLOCS: AtomicU64 = AtomicU64::new(0);
static TOTAL: AtomicU64 = AtomicU64::new(0);
static LIVE: AtomicI64 = AtomicI64::new(0);
static PEAK: AtomicI64 = AtomicI64::new(0);
static REALLOCS: AtomicU64 = AtomicU64::new(0);
fn record(delta: i64, additional: u64, alloc: bool, realloc: bool) {
    if !ACTIVE.load(Relaxed) {
        return;
    }
    if alloc {
        ALLOCS.fetch_add(1, Relaxed);
    }
    if realloc {
        REALLOCS.fetch_add(1, Relaxed);
    }
    TOTAL.fetch_add(additional, Relaxed);
    let now = LIVE.fetch_add(delta, Relaxed) + delta;
    PEAK.fetch_max(now, Relaxed);
}
unsafe impl GlobalAlloc for Meter {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr = unsafe { System.alloc(layout) };
        if !ptr.is_null() {
            record(layout.size() as i64, layout.size() as u64, true, false);
        }
        ptr
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        record(-(layout.size() as i64), 0, false, false);
        unsafe {
            System.dealloc(ptr, layout);
        }
    }
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        let p = unsafe { System.realloc(ptr, layout, new_size) };
        if !p.is_null() {
            record(
                new_size as i64 - layout.size() as i64,
                new_size as u64,
                false,
                true,
            );
        }
        p
    }
}
#[global_allocator]
static GLOBAL: Meter = Meter;
fn measure(label: &str, mut f: impl FnMut()) {
    f(); // warmup, input setup is outside measurement
    ALLOCS.store(0, Relaxed);
    TOTAL.store(0, Relaxed);
    REALLOCS.store(0, Relaxed);
    LIVE.store(0, Relaxed);
    PEAK.store(0, Relaxed);
    ACTIVE.store(true, Relaxed);
    f();
    ACTIVE.store(false, Relaxed);
    let (alloc, total, peak, realloc, live) = (
        ALLOCS.load(Relaxed),
        TOTAL.load(Relaxed),
        PEAK.load(Relaxed),
        REALLOCS.load(Relaxed),
        LIVE.load(Relaxed),
    );
    println!("{label},{alloc},{total},{peak},{realloc},{live}");
}
fn z(v: Vec<i64>) -> Z {
    Z::new(v.into_iter().map(Integer::from).collect())
}
fn q(v: Vec<(i64, i64)>) -> Q {
    Q::new(
        v.into_iter()
            .map(|(n, d)| Rational::new(Integer::from(n), Integer::from(d)).unwrap())
            .collect(),
    )
}
fn q_from_z(poly: &Z, den: i64) -> Q {
    Q::new(
        poly.coefficients()
            .iter()
            .cloned()
            .map(|c| Rational::new(c, Integer::from(den)).unwrap())
            .collect(),
    )
}
fn main() {
    let factor = z(vec![1, 1]);
    let a = z(vec![-1, 3, 0, 0, 0, 0, 0, 0, 1]);
    let b = z(vec![2, -4, 0, 0, 0, 0, 0, 0, 1]);
    let shared_a = a.mul(&factor);
    let shared_b = b.mul(&factor);
    let qa = q_from_z(&shared_a, 17);
    let qb = q_from_z(&shared_b, 19);
    let r1 = q(vec![(1, 3), (-3, 7), (5, 11), (13, 17), (-9, 19)]);
    let r2 = q(vec![(7, 5), (-5, 13), (2, 3)]);
    let product = r1.mul(&r2);
    let exact_dividend = shared_a.mul(&b);
    println!(
        "case,allocations,total_requested_bytes,peak_incremental_live_bytes,reallocations,net_live_delta"
    );
    measure("integer_shared_gcd", || {
        let result = black_box(&shared_a).gcd(black_box(&shared_b)).unwrap();
        black_box(&result);
        drop(result);
    });
    measure("rational_shared_gcd", || {
        let result = black_box(&qa).gcd(black_box(&qb)).unwrap();
        black_box(&result);
        drop(result);
    });
    measure("rational_mul", || {
        let result = black_box(&r1).mul(black_box(&r2));
        black_box(&result);
        drop(result);
    });
    measure("rational_div_rem", || {
        let result = black_box(&product).div_rem(black_box(&r1)).unwrap();
        black_box(&result);
        drop(result);
    });
    measure("integer_exact_div", || {
        let result = black_box(&exact_dividend).div_exact(black_box(&b)).unwrap();
        black_box(&result);
        drop(result);
    });
    measure("integer_coprime_gcd", || {
        let result = black_box(&a).gcd(black_box(&b)).unwrap();
        black_box(&result);
        drop(result);
    });
}
