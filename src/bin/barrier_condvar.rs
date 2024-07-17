#![no_std]
#![no_main]

#[macro_use]
extern crate frontier_user;
extern crate lazy_static;
extern crate alloc;

use alloc::vec::Vec;
use core::cell::UnsafeCell;
use frontier_user::{
    create_condvar, create_mutex, exit, lock_mutex, signal_condvar, thread_create, unlock_mutex,
    wait_condvar, wait_tid,
};
use lazy_static::*;

const THREAD_NUM: usize = 3;

struct Barrier {
    mutex_id: usize,
    condvar_id: usize,
    count: UnsafeCell<usize>,
}

impl Barrier {
    pub fn new() -> Self {
        Self {
            mutex_id: create_mutex(true) as usize,
            condvar_id: create_condvar() as usize,
            count: UnsafeCell::new(0),
        }
    }
    pub fn block(&self) {
        lock_mutex(self.mutex_id);
        let count = self.count.get();
        // SAFETY: Here, the accesses of the count is in the
        // critical section protected by the mutex.
        unsafe {
            *count = *count + 1;
        }
        if unsafe { *count } == THREAD_NUM {
            signal_condvar(self.condvar_id);
        } else {
            wait_condvar(self.condvar_id, self.mutex_id);
            signal_condvar(self.condvar_id);
        }
        unlock_mutex(self.mutex_id);
    }
}

unsafe impl Sync for Barrier {}

lazy_static! {
    static ref BARRIER_AB: Barrier = Barrier::new();
    static ref BARRIER_BC: Barrier = Barrier::new();
}

fn thread_fn() {
    for _ in 0..300 {
        print!("a");
    }
    BARRIER_AB.block();
    for _ in 0..300 {
        print!("b");
    }
    BARRIER_BC.block();
    for _ in 0..300 {
        print!("c");
    }
    exit(0)
}

#[no_mangle]
pub fn main() -> i32 {
    let mut v: Vec<isize> = Vec::new();
    for _ in 0..THREAD_NUM {
        v.push(thread_create(thread_fn as usize, 0));
    }
    let mut exit_code = 0;
    for tid in v.into_iter() {
        wait_tid(tid as usize, &mut exit_code);
    }
    println!("\nOK!");
    0
}
