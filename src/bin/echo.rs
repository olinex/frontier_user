// @author:    olinex
// @time:      2024/03/23
#![no_std]
#![no_main]

// extern other crate
#[macro_use]
extern crate frontier_user;

// self mods

// use other mods

// use self mods

#[no_mangle]
fn main(_: &str, args: &str) -> i32 {
    println!("{}", args);
    0
}
