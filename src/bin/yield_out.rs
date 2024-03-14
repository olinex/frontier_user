#![no_std]
#![no_main]

#[macro_use]
extern crate frontier_user;
use frontier_user::{get_pid, yield_out};

#[no_mangle]
pub fn main() -> i32 {
    println!("Hello, I am process {}.", get_pid());
    yield_out();
    println!("yield pass.");
    0
}
