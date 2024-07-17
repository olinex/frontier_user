#![no_std]
#![no_main]

// @author:    olinex
// @time:      2023/10/08

#[macro_use]
extern crate frontier_user;

// self mods

// use other mods
use frontier_user::{exec_without_args, fork, wait_any_pid, yield_out};

// use self mods

#[no_mangle]
fn main(_: &str, _: &str) -> i32 {
    println!("[initproc] Starting...");
    match fork() {
        0 => {
            println!("[initproc] Creating a simple shell process...");
            exec_without_args("core_shell\0");
        }
        cpid => loop {
            let mut exit_code: i32 = 0;
            let pid = wait_any_pid(&mut exit_code);
            if pid == cpid {
                println!("[initproc] Child process {} is closed", cpid);
                break;
            }
            match pid {
                -2 => {
                    println!("[initproc] No any process to wait, continue...");
                    yield_out();
                    continue;
                }
                -1 => {
                    println!("[initproc] No any process to run, exit...");
                    break;
                }
                _ => {
                    println!("[initproc] Zombie child process {} is closed", pid);
                    continue;
                },
            }
        },
    };
    0
}
