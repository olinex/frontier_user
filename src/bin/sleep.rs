#![no_std]
#![no_main]

#[macro_use]
extern crate frontier_user;
use frontier_user::{exit, fork, get_time, sleep, wait_pid};

fn sleepy() {
    let time: usize = 1000 * 1000;
    for i in 0..5 {
        sleep(time);
        println!("sleep {} x {} microseconds.", i + 1, time);
    }
    exit(0);
}

#[no_mangle]
pub fn main(_: &str, _: &str) -> i32 {
    let start_time = get_time();
    let pid = fork();
    let mut exit_code: i32 = 0;
    if pid == 0 {
        sleepy();
    }
    let stop_time = get_time();
    assert!(wait_pid(pid as usize, &mut exit_code) == pid && exit_code == 0);
    let wait_time = get_time();
    println!("use {} microseconds to fork.", stop_time - start_time);
    println!("use {} microseconds to wait pid.", wait_time - stop_time);
    println!("use {} microseconds to run other.", get_time() - wait_time);
    println!("sleep pass.");
    0
}
