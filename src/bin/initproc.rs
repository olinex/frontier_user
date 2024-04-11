#![no_std]
#![no_main]

// @author:    olinex
// @time:      2023/10/08

#[macro_use]
extern crate frontier_user;

// self mods

// use other mods
use frontier_user::{exec_without_args, fork, wait, yield_out};

// use self mods

#[no_mangle]
fn main(_: &str, _: &str) -> i32 {
    match fork() {
        0 => {
            println!("[initproc] Creating a simple shell process...");
            exec_without_args("core_shell\0");
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
