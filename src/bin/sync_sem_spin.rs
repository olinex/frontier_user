#![no_std]
#![no_main]

#[macro_use]
extern crate frontier_user;

#[macro_use]
extern crate alloc;

use frontier_user::{create_semaphore, down_semaphore, up_semaphore};
use frontier_user::{exit, thread_create, wait_tid, sleep};

const SEM_SYNC: usize = 0;

unsafe fn first() -> ! {
    sleep(10);
    println!("First work and wakeup Second");
    up_semaphore(SEM_SYNC);
    exit(0)
}

unsafe fn second() -> ! {
    println!("Second want to continue,but need to wait first");
    down_semaphore(SEM_SYNC);
    println!("Second can work now");
    exit(0)
}

#[no_mangle]
pub fn main() -> i32 {
    // create semaphores
    assert_eq!(create_semaphore(false, 0) as usize, SEM_SYNC);
    // create threads
    let threads = vec![
        thread_create(first as usize, 0),
        thread_create(second as usize, 0),
    ];
    let mut exit_code: i32 = 0;
    // wait for all threads to complete
    for thread in threads.iter() {
        wait_tid(*thread as usize, &mut exit_code);
    }
    println!("sync_sem passed!");
    0
}