#![no_std]
#![no_main]

#[macro_use]
extern crate frontier_user;
extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use frontier_user::{create_mutex, lock_mutex, unlock_mutex};
use frontier_user::{exit, get_time, thread_create, wait_tid};

static mut A: usize = 0;
const PER_THREAD_DEFAULT: usize = 10000;
const THREAD_COUNT_DEFAULT: usize = 16;
static mut PER_THREAD: usize = 0;

unsafe fn critical_section(t: &mut usize) {
    let a = &mut A as *mut usize;
    let cur = a.read_volatile();
    for _ in 0..500 {
        *t = (*t) * (*t) % 10007;
    }
    a.write_volatile(cur + 1);
}
unsafe fn f() -> ! {
    let mut t = 2usize;
    for _ in 0..PER_THREAD {
        lock_mutex(0);
        critical_section(&mut t);
        unlock_mutex(0);
    }
    exit(t as i32)
}

#[no_mangle]
pub fn main(_: &str, args: &str) -> i32 {
    let mut thread_count = THREAD_COUNT_DEFAULT;
    let mut per_thread = PER_THREAD_DEFAULT;
    let args: Vec<String> = args
        .split_ascii_whitespace()
        .map(|arg| arg.trim().to_string())
        .collect();
    if args.len() >= 1 {
        thread_count = args[0].parse().unwrap();
        if args.len() >= 2 {
            per_thread = args[1].parse().unwrap();
        }
    }
    unsafe {
        PER_THREAD = per_thread;
    }

    let start = get_time();
    assert_eq!(create_mutex(true), 0);
    let mut v = Vec::new();
    for _ in 0..thread_count {
        v.push(thread_create(f as usize, 0) as usize);
    }
    let mut exit_code = 0;
    for tid in v.into_iter() {
        wait_tid(tid, &mut exit_code);
    }
    println!("time cost is {}ms", (get_time() - start) / 1000);
    assert_eq!(unsafe { A }, unsafe { PER_THREAD } * thread_count);
    0
}
