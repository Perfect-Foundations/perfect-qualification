use std::env;
#[cfg(feature = "alloc-count")]
use std::alloc::{GlobalAlloc, Layout, System};
use std::hint::black_box;
#[cfg(feature = "alloc-count")]
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::time::{Duration, Instant};

use num_bigint::BigUint;
use num_integer::Integer as NumInteger;
use perfect_arithmetic::Natural as PerfectNatural;

const BASE: u64 = (1_u64 << 61) - 1;
const TARGET_SAMPLE: Duration = Duration::from_millis(50);
const SAMPLES: usize = 7;
const SIZES: &[usize] = &[256, 2_048, 16_384];

#[cfg(feature = "alloc-count")]
struct CountingAllocator;

#[cfg(feature = "alloc-count")]
static COUNTING: AtomicBool = AtomicBool::new(false);
#[cfg(feature = "alloc-count")]
static ALLOCS: AtomicU64 = AtomicU64::new(0);
#[cfg(feature = "alloc-count")]
static DEALLOCS: AtomicU64 = AtomicU64::new(0);
#[cfg(feature = "alloc-count")]
static BYTES_ALLOCATED: AtomicU64 = AtomicU64::new(0);

#[cfg(feature = "alloc-count")]
#[global_allocator]
static GLOBAL: CountingAllocator = CountingAllocator;

#[cfg(feature = "alloc-count")]
unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // SAFETY: forwarded unchanged to the system allocator.
        let ptr = unsafe { System.alloc(layout) };
        if COUNTING.load(Ordering::Relaxed) && !ptr.is_null() {
            ALLOCS.fetch_add(1, Ordering::Relaxed);
            BYTES_ALLOCATED.fetch_add(layout.size() as u64, Ordering::Relaxed);
        }
        ptr
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        // SAFETY: forwarded unchanged to the system allocator.
        let ptr = unsafe { System.alloc_zeroed(layout) };
        if COUNTING.load(Ordering::Relaxed) && !ptr.is_null() {
            ALLOCS.fetch_add(1, Ordering::Relaxed);
            BYTES_ALLOCATED.fetch_add(layout.size() as u64, Ordering::Relaxed);
        }
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        if COUNTING.load(Ordering::Relaxed) && !ptr.is_null() {
            DEALLOCS.fetch_add(1, Ordering::Relaxed);
        }
        // SAFETY: forwarded unchanged to the system allocator.
        unsafe { System.dealloc(ptr, layout) };
    }

    unsafe fn realloc(&self, ptr: *mut u8, old: Layout, new_size: usize) -> *mut u8 {
        // SAFETY: forwarded unchanged to the system allocator.
        let new_ptr = unsafe { System.realloc(ptr, old, new_size) };
        if COUNTING.load(Ordering::Relaxed) && !new_ptr.is_null() {
            // Treat every successful realloc as one allocation plus one
            // deallocation event and charge the full destination allocation.
            // This reports allocation traffic consistently even for in-place
            // growth or shrinkage.
            ALLOCS.fetch_add(1, Ordering::Relaxed);
            DEALLOCS.fetch_add(1, Ordering::Relaxed);
            BYTES_ALLOCATED.fetch_add(new_size as u64, Ordering::Relaxed);
        }
        new_ptr
    }
}

trait Backend {
    type N: Clone;

    const NAME: &'static str;

    fn from_u64(value: u64) -> Self::N;
    fn shl_bits(value: &Self::N, bits: u64) -> Self::N;
    fn add(left: &Self::N, right: &Self::N) -> Self::N;
    fn mul(left: &Self::N, right: &Self::N) -> Self::N;
    fn div_rem(left: &Self::N, right: &Self::N) -> (Self::N, Self::N);
}

struct Perfect;
struct Direct;

impl Backend for Perfect {
    type N = PerfectNatural;
    const NAME: &'static str = "perfect-arithmetic";

    fn from_u64(value: u64) -> Self::N {
        PerfectNatural::from(value)
    }

    fn shl_bits(value: &Self::N, bits: u64) -> Self::N {
        value.shl_bits(bits)
    }

    fn add(left: &Self::N, right: &Self::N) -> Self::N {
        left.add(right)
    }

    fn mul(left: &Self::N, right: &Self::N) -> Self::N {
        left.mul(right)
    }

    fn div_rem(left: &Self::N, right: &Self::N) -> (Self::N, Self::N) {
        left.div_rem(right).expect("nonzero qualification divisor")
    }
}

impl Backend for Direct {
    type N = BigUint;
    const NAME: &'static str = "num-bigint-0.5.1-best";

    fn from_u64(value: u64) -> Self::N {
        BigUint::from(value)
    }

    fn shl_bits(value: &Self::N, bits: u64) -> Self::N {
        value << usize::try_from(bits).expect("qualification bit count fits usize")
    }

    fn add(left: &Self::N, right: &Self::N) -> Self::N {
        left + right
    }

    fn mul(left: &Self::N, right: &Self::N) -> Self::N {
        left * right
    }

    fn div_rem(left: &Self::N, right: &Self::N) -> (Self::N, Self::N) {
        NumInteger::div_rem(left, right)
    }
}

fn build_value<B: Backend>(target_bits: usize, seed: u64, salt: u64) -> B::N {
    assert!(target_bits >= 2);

    // Construct base-2^61 chunks so the declared target bit is set exactly.
    // This avoids labeling near-threshold operands with a size they do not have.
    let chunks = target_bits.div_ceil(61);
    let top_bits = target_bits - (chunks - 1) * 61;
    let top_high = 1_u64 << (top_bits - 1);
    let top_mask = top_high - 1;
    let mut value = B::from_u64(top_high | (seed & top_mask));

    for chunk in 1..chunks {
        value = B::shl_bits(&value, 61);
        let term = salt
            .wrapping_add((chunk as u64).wrapping_mul(1_000_003))
            .wrapping_add(17)
            & BASE;
        value = B::add(&value, &B::from_u64(term));
    }
    value
}

fn operands<B: Backend>(bits: usize) -> (B::N, B::N, B::N) {
    let left = build_value::<B>(bits, 0x9e37_79b9, 0x85eb_ca6b);
    let right = build_value::<B>(bits, 0xc2b2_ae35, 0x27d4_eb2f);
    let divisor = build_value::<B>((bits / 2).max(61), 0x1656_67b1, 0xd3a2_646c);
    (left, right, divisor)
}

#[derive(Clone, Copy)]
enum Operation {
    Add,
    Mul,
    DivRem,
}

impl Operation {
    fn name(self) -> &'static str {
        match self {
            Self::Add => "add",
            Self::Mul => "mul",
            Self::DivRem => "div_rem",
        }
    }
}

fn run_operation<B: Backend>(operation: Operation, left: &B::N, right: &B::N, divisor: &B::N) {
    match operation {
        Operation::Add => {
            black_box(B::add(black_box(left), black_box(right)));
        }
        Operation::Mul => {
            black_box(B::mul(black_box(left), black_box(right)));
        }
        Operation::DivRem => {
            black_box(B::div_rem(black_box(left), black_box(divisor)));
        }
    }
}

#[cfg(not(feature = "alloc-count"))]
fn calibrate<F>(mut operation: F) -> usize
where
    F: FnMut(),
{
    for _ in 0..3 {
        operation();
    }

    let mut iterations = 1_usize;
    loop {
        let start = Instant::now();
        for _ in 0..iterations {
            operation();
        }
        let elapsed = start.elapsed();
        if elapsed >= Duration::from_millis(8) || iterations >= 1_000_000 {
            let elapsed_ns = elapsed.as_nanos().max(1);
            let target_ns = TARGET_SAMPLE.as_nanos();
            return (((iterations as u128) * target_ns / elapsed_ns).clamp(1, 2_000_000))
                as usize;
        }
        iterations = iterations.saturating_mul(4).min(1_000_000);
    }
}

#[cfg(not(feature = "alloc-count"))]
fn measure_timing<F>(mut operation: F) -> f64
where
    F: FnMut(),
{
    let iterations = calibrate(&mut operation);
    let mut samples = Vec::with_capacity(SAMPLES);
    for _ in 0..SAMPLES {
        let start = Instant::now();
        for _ in 0..iterations {
            operation();
        }
        samples.push(start.elapsed().as_secs_f64() * 1e9 / iterations as f64);
    }
    samples.sort_by(f64::total_cmp);
    samples[SAMPLES / 2]
}

#[cfg(feature = "alloc-count")]
fn measure_allocations<F>(mut operation: F) -> (f64, f64, f64)
where
    F: FnMut(),
{
    for _ in 0..3 {
        operation();
    }
    let iterations = 128_u64;
    ALLOCS.store(0, Ordering::Relaxed);
    DEALLOCS.store(0, Ordering::Relaxed);
    BYTES_ALLOCATED.store(0, Ordering::Relaxed);
    COUNTING.store(true, Ordering::SeqCst);
    for _ in 0..iterations {
        operation();
    }
    COUNTING.store(false, Ordering::SeqCst);
    (
        ALLOCS.load(Ordering::Relaxed) as f64 / iterations as f64,
        DEALLOCS.load(Ordering::Relaxed) as f64 / iterations as f64,
        BYTES_ALLOCATED.load(Ordering::Relaxed) as f64 / iterations as f64,
    )
}

#[cfg(not(feature = "alloc-count"))]
fn emit<B: Backend>(bits: usize, operation: Operation) {
    let (left, right, divisor) = operands::<B>(bits);
    let value = measure_timing(|| run_operation::<B>(operation, &left, &right, &divisor));
    println!("{},{},{bits},{value:.3}", B::NAME, operation.name());
}

#[cfg(feature = "alloc-count")]
fn emit<B: Backend>(bits: usize, operation: Operation) {
    let (left, right, divisor) = operands::<B>(bits);
    let (allocs, deallocs, bytes) =
        measure_allocations(|| run_operation::<B>(operation, &left, &right, &divisor));
    println!(
        "{},{},{bits},{allocs:.3},{deallocs:.3},{bytes:.3}",
        B::NAME,
        operation.name()
    );
}

#[derive(Clone, Copy)]
struct Case {
    backend: u8,
    bits: usize,
    operation: Operation,
}

fn shuffle(cases: &mut [Case]) {
    let mut state = 0x9e37_79b9_7f4a_7c15_u64;
    for index in (1..cases.len()).rev() {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        cases.swap(index, (state as usize) % (index + 1));
    }
}

fn main() {
    if env::args().len() != 1 {
        panic!("qualification performance driver accepts no arguments");
    }

    #[cfg(not(feature = "alloc-count"))]
    println!("backend,operation,target_bits,median_ns_per_op");
    #[cfg(feature = "alloc-count")]
    println!("backend,operation,target_bits,allocs_per_op,deallocs_per_op,allocated_bytes_per_op");

    let mut cases = Vec::new();
    for &bits in SIZES {
        for operation in [Operation::Add, Operation::Mul, Operation::DivRem] {
            cases.push(Case { backend: 0, bits, operation });
            cases.push(Case { backend: 1, bits, operation });
        }
    }
    shuffle(&mut cases);

    for case in cases {
        match case.backend {
            0 => emit::<Perfect>(case.bits, case.operation),
            1 => emit::<Direct>(case.bits, case.operation),
            _ => unreachable!("unknown qualification backend"),
        }
    }
}
