#![no_std]
#![no_main]

#[macro_use]
extern crate frontier_user;

extern crate alloc;

use alloc::vec;
use frontier_user::exit;
use frontier_user::{
    create_mutex, lock_mutex, unlock_mutex, create_semaphore, down_semaphore, up_semaphore,
};
use frontier_user::{sleep, thread_create, wait_tid};

static mut A: usize = 0;

const SEM_ID: usize = 0;
const MUTEX_ID: usize = 0;

unsafe fn first() -> ! {
    sleep(10);
    println!("First work, Change A --> 1 and wakeup Second");
    lock_mutex(MUTEX_ID);
    A = 1;
    up_semaphore(SEM_ID);
    unlock_mutex(MUTEX_ID);
    exit(0)
}

unsafe fn second() -> ! {
    println!("Second want to continue,but need to wait A=1");
    loop {
        lock_mutex(MUTEX_ID);
        if A == 0 {
            println!("Second: A is {}", A);
            unlock_mutex(MUTEX_ID);
            down_semaphore(SEM_ID);
        } else {
            unlock_mutex(MUTEX_ID);
            break;
        }
    }
    println!("A is {}, Second can work now", A);
    exit(0)
}

#[no_mangle]
pub fn main() -> i32 {
    // create semaphore & mutex
    assert_eq!(create_semaphore(true, 0) as usize, SEM_ID);
    assert_eq!(create_mutex(true) as usize, MUTEX_ID);
    // create threads
    let threads = vec![
        thread_create(first as usize, 0),
        thread_create(second as usize, 0),
    ];
    // wait for all threads to complete
    let mut exit_code = 0;
    for thread in threads.iter() {
        wait_tid(*thread as usize, &mut exit_code);
    }
    println!("test_condvar passed!");
    0
}
