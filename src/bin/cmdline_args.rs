#![no_std]
#![no_main]

extern crate alloc;

#[macro_use]
extern crate frontier_user;
use alloc::vec::Vec;
use alloc::string::{String, ToString};

#[no_mangle]
pub fn main(path: &str, args: &str) -> i32 {
    assert_eq!(path, "cmdline_args");
    let args: Vec<String> = args
        .split_ascii_whitespace()
        .map(|arg| arg.trim().to_string())
        .collect();
    for (i, arg) in args.iter().enumerate() {
        println!("args[{}] = {}", i, arg);
    }
    0
}
