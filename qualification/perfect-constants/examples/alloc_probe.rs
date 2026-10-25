//! A bounded, independent allocation probe for lookup only.
//! This does not prove no allocation in every downstream context.
use perfect_constants::math::ALL;
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

struct Counting;
static ALLOCATIONS: AtomicUsize = AtomicUsize::new(0);
#[global_allocator]
static COUNTER: Counting = Counting;

// Safety: the wrapper faithfully delegates every allocator operation to System.
// All counting is advisory; pointer/size/deallocation behavior is unchanged.
unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOCATIONS.fetch_add(1, Ordering::Relaxed);
        unsafe { System.alloc(layout) }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) };
    }
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        ALLOCATIONS.fetch_add(1, Ordering::Relaxed);
        unsafe { System.alloc_zeroed(layout) }
    }
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        ALLOCATIONS.fetch_add(1, Ordering::Relaxed);
        unsafe { System.realloc(ptr, layout, new_size) }
    }
}

fn main() {
    let baseline = ALLOCATIONS.load(Ordering::SeqCst);
    for _ in 0..1000 {
        for id in ALL {
            std::hint::black_box(id.as_f32().to_bits());
            std::hint::black_box(id.as_f64().to_bits());
            std::hint::black_box(id.name().len());
        }
    }
    let count = ALLOCATIONS.load(Ordering::SeqCst) - baseline;
    assert_eq!(count, 0, "7,000 constant lookups allocated");
    println!("CONSTANTS_INDEPENDENT_LOOKUP_ALLOCATIONS=0");
}
