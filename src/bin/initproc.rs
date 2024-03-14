#![no_std]
#![no_main]

#[macro_use]
extern crate frontier_user;

// @author:    olinex
// @time:      2023/10/08

// self mods

// use other mods
use frontier_user::{exec, fork, wait, yield_out};

// use self mods

#[no_mangle]
fn main() -> i32 {
    match fork() {
        0 => {
            println!("[initproc] Creating a simple shell process...");
            exec("core_shell\0");
        }
        _ => loop {
            let mut exit_code: i32 = 0;
            let pid = wait(&mut exit_code);
            if pid == -1 {
                yield_out();
                continue;
            }
        }
    };
    0
}
