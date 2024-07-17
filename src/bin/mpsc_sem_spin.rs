#![no_std]
#![no_main]

#[macro_use]
extern crate frontier_user;
extern crate alloc;

use alloc::vec::Vec;
use frontier_user::{create_semaphore, up_semaphore, down_semaphore};
use frontier_user::{exit, thread_create, wait_tid};

const SEM_MUTEX: usize = 0;
const SEM_EMPTY: usize = 1;
const SEM_AVAIL: usize = 2;
const BUFFER_SIZE: usize = 8;
static mut BUFFER: [usize; BUFFER_SIZE] = [0; BUFFER_SIZE];
static mut FRONT: usize = 0;
static mut TAIL: usize = 0;
const PRODUCER_COUNT: usize = 4;
const NUMBER_PER_PRODUCER: usize = 100;

unsafe fn producer(id: *const usize) -> ! {
    let id = *id;
    for _ in 0..NUMBER_PER_PRODUCER {
        down_semaphore(SEM_EMPTY);
        down_semaphore(SEM_MUTEX);
        BUFFER[TAIL] = id;
        TAIL = (TAIL + 1) % BUFFER_SIZE;
        up_semaphore(SEM_MUTEX);
        up_semaphore(SEM_AVAIL);
    }
    exit(0)
}

unsafe fn consumer() -> ! {
    for _ in 0..PRODUCER_COUNT * NUMBER_PER_PRODUCER {
        down_semaphore(SEM_AVAIL);
        down_semaphore(SEM_MUTEX);
        print!("{} ", BUFFER[FRONT]);
        FRONT = (FRONT + 1) % BUFFER_SIZE;
        up_semaphore(SEM_MUTEX);
        up_semaphore(SEM_EMPTY);
    }
    println!("");
    exit(0)
}

#[no_mangle]
pub fn main() -> i32 {
    // create semaphores
    assert_eq!(create_semaphore(false, 1) as usize, SEM_MUTEX);
    assert_eq!(create_semaphore(false, BUFFER_SIZE as isize) as usize, SEM_EMPTY);
    assert_eq!(create_semaphore(false, 0) as usize, SEM_AVAIL);
    // create threads
    let ids: Vec<_> = (0..PRODUCER_COUNT).collect();
    let mut threads = Vec::new();
    for i in 0..PRODUCER_COUNT {
        threads.push(thread_create(
            producer as usize,
            &ids.as_slice()[i] as *const _ as usize,
        ));
    }
    threads.push(thread_create(consumer as usize, 0));
    // wait for all threads to complete
    let mut exit_code = 0;
    for thread in threads.iter() {
        wait_tid(*thread as usize, &mut exit_code);
    }
    println!("mpsc_sem passed!");
    0
}
