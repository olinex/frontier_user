#![no_std]
#![no_main]

#[macro_use]
extern crate frontier_user;

#[no_mangle]
fn main() -> i32 {
    println!("Hello, world!");
    0
}
