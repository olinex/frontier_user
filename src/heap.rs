// @author:    olinex
// @time:      2023/11/02

// self mods

// use other mods
use buddy_system_allocator as allocator;

// use self mods
use crate::configs;

static mut HEAP_SPACE: [u8; configs::USER_HEAP_SIZE] = [0; configs::USER_HEAP_SIZE];

#[global_allocator]
static HEAP: allocator::LockedHeap<32> = allocator::LockedHeap::<32>::empty();

#[alloc_error_handler]
pub fn handle_alloc_error(layout: core::alloc::Layout) -> ! {
    panic!("Heap allocation error, layout = {:?}", layout);
}

pub fn init_heap() {
    unsafe {
        HEAP.lock()
            .init(HEAP_SPACE.as_ptr() as usize, configs::USER_HEAP_SIZE);
    }
}
