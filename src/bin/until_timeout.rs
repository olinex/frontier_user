#![no_std]
#![no_main]

#[macro_use]
extern crate frontier_user;

extern crate alloc;
extern crate frontier_lib;

use alloc::vec::Vec;
use frontier_lib::model::signal::Signal;
use frontier_user::constant::shortcut;
use frontier_user::{exec, fork, get_time, kill, wait_pid, wait_pid_no_pause};

#[no_mangle]
pub fn main(_: &str, args: &str) -> i32 {
    let args: Vec<&str> = args
    .split_ascii_whitespace()
    .map(|arg| arg.trim())
    .collect();
    assert_eq!(args.len(), 2, "args must be 2!");
    let sub_command_path = args[0];
    let timeout_ms = args[1]
        .parse::<isize>()
        .expect("Error when parsing timeout!");
    let pid = fork() as usize;
    if pid == 0 {
        if exec(sub_command_path, shortcut::EMPTY) != 0 {
            println!("Error when executing '{}'", sub_command_path);
            return -4;
        }
    } else {
        let start_time = get_time();
        let mut child_exited = false;
        let mut exit_code: i32 = 0;
        loop {
            if get_time() - start_time > timeout_ms {
                break;
            }
            if wait_pid_no_pause(pid, &mut exit_code) as usize == pid {
                child_exited = true;
                println!(
                    "child exited in {}ms, exit_code = {}",
                    get_time() - start_time,
                    exit_code,
                );
            }
        }
        if !child_exited {
            println!("child has run for {}ms, kill it!", timeout_ms);
            kill(pid, Signal::INT);
            assert_eq!(wait_pid(pid, &mut exit_code) as usize, pid);
            println!("exit code of the child is {}", exit_code);
        }
    }
    0
}
