#![no_std]
#![no_main]

#[macro_use]
extern crate frontier_user;
extern crate alloc;

use alloc::vec::Vec;
use frontier_user::{exit, thread_create, wait_tid};

const THREAD_NUM: usize = 3;

fn thread_fn() {
    for ch in 'a'..='c' {
        for _ in 0..300 {
            print!("{}", ch);
        }
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
