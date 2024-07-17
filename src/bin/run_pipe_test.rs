#![no_std]
#![no_main]

#[macro_use]
extern crate frontier_user;

use frontier_user::constant::shortcut;
use frontier_user::{exec, fork, wait_any_pid};

#[no_mangle]
pub fn main() -> i32 {
    for i in 0..5 {
        if fork() == 0 {
            exec("pipe_large_test\0", shortcut::NULL);
        } else {
            let mut exit_code: i32 = 0;
            let pid = wait_any_pid(&mut exit_code);
            println!("Iter {} OK, process {} exit with code {}", i, pid, exit_code);
        }
    }
    0
}
