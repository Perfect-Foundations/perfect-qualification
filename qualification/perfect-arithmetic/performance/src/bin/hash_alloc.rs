use std::alloc::{GlobalAlloc, Layout, System};
use std::hash::{Hash, Hasher};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

use perfect_arithmetic::{Integer, Natural};

struct CountingAllocator;

static COUNTING: AtomicBool = AtomicBool::new(false);
static ALLOCS: AtomicU64 = AtomicU64::new(0);
static DEALLOCS: AtomicU64 = AtomicU64::new(0);
static BYTES_ALLOCATED: AtomicU64 = AtomicU64::new(0);

#[global_allocator]
static GLOBAL: CountingAllocator = CountingAllocator;

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

struct StableHasher(u64);

impl StableHasher {
    fn new() -> Self {
        Self(0xcbf2_9ce4_8422_2325)
    }
}

impl Hasher for StableHasher {
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.0 ^= u64::from(byte);
            self.0 = self.0.wrapping_mul(0x0000_0100_0000_01b3);
        }
    }
}

fn measure<T: Hash>(value: &T) -> (u64, u64, u64) {
    ALLOCS.store(0, Ordering::Relaxed);
    DEALLOCS.store(0, Ordering::Relaxed);
    BYTES_ALLOCATED.store(0, Ordering::Relaxed);
    COUNTING.store(true, Ordering::SeqCst);
    let mut hasher = StableHasher::new();
    value.hash(&mut hasher);
    std::hint::black_box(hasher.finish());
    COUNTING.store(false, Ordering::SeqCst);
    (
        ALLOCS.load(Ordering::Relaxed),
        DEALLOCS.load(Ordering::Relaxed),
        BYTES_ALLOCATED.load(Ordering::Relaxed),
    )
}

fn main() {
    let natural = Natural::one()
        .shl_bits(131_071)
        .add(&Natural::from(0xfeed_beef_u64));
    let integer = Integer::from(natural.clone()).neg();

    println!("kind,target_bits,allocs,deallocs,allocated_bytes");
    let (allocs, deallocs, bytes) = measure(&natural);
    println!("natural,131072,{allocs:.3},{deallocs:.3},{bytes:.3}");
    let (allocs, deallocs, bytes) = measure(&integer);
    println!("integer,131072,{allocs:.3},{deallocs:.3},{bytes:.3}");
}
