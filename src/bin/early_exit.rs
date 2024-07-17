#![no_std]
#![no_main]

#[macro_use]
extern crate frontier_user;
extern crate alloc;

use frontier_user::{exit, thread_create};

pub fn thread_a() -> ! {
    for i in 0..1000 {
        print!("{}", i);
    }
    exit(1)
}

#[no_mangle]
pub fn main() -> i32 {
    thread_create(thread_a as usize, 0);
    println!("main thread exited.");
    exit(0)
}
