#![no_std]
#![no_main]

#[macro_use]
extern crate frontier_user;

static TESTS: &[&str] = &[
    "exit_test\0",
    "fantastic_text\0",
    "forktest\0",
    "forktest2\0",
    "forktest_simple\0",
    "hello_world\0",
    "matrix\0",
    "sleep\0",
    "sleep_simple\0",
    "stack_overflow\0",
    "yield_out\0",
];

use frontier_user::{exec, fork, wait_pid};

#[no_mangle]
pub fn main() -> i32 {
    for test in TESTS {
        println!("Usertests: Running {}", test);
        let pid = fork();
        if pid == 0 {
            exec(*test);
            panic!("unreachable!");
        } else {
            let mut exit_code: i32 = Default::default();
            let wait_pid = wait_pid(pid as usize, &mut exit_code);
            assert_eq!(pid, wait_pid);
            println!(
                "\x1b[32mUsertests: Test {} in Process {} exited with code {}\x1b[0m",
                test, pid, exit_code
            );
        }
    }
    println!("Usertests passed!");
    0
}
