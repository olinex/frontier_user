#![no_std]
#![no_main]

#[macro_use]
extern crate frontier_user;
use frontier_user::{get_time, sleep};

#[no_mangle]
pub fn main(_: &str, _: &str) -> i32 {
    println!("into sleep test!");
    let start = get_time();
    println!("current time_microseconds = {}", start);
    sleep(10000);
    let end = get_time();
    println!(
        "microseconds = {} after sleeping 100 ticks, delta = {}us!",
        end,
        end - start
    );
    println!("r_sleep passed!");
    0
}
