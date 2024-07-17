// @author:    olinex
// @time:      2024/05/16
#![no_std]
#![no_main]

// extern other crate
#[macro_use]
extern crate frontier_user;

extern crate alloc;

// self mods

// use other mods
use alloc::vec::Vec;

// use self mods
use frontier_user::{exit, wait_tid, thread_create};

pub fn thread_a() -> ! {
    for _ in 0..1000 { print!("a"); }
    println!("");
    exit(1)
}

pub fn thread_b() -> ! {
    for _ in 0..1000 { print!("b"); }
    println!("");
    exit(2)
}

pub fn thread_c() -> ! {
    for _ in 0..1000 { print!("c"); }
    println!("");
    exit(3)
}

#[no_mangle]
pub fn main() -> i32 {
    let mut v = Vec::new();
    v.push(thread_create(thread_a as usize, 0) as usize);
    v.push(thread_create(thread_b as usize, 0) as usize);
    v.push(thread_create(thread_c as usize, 0) as usize);
    
    for tid in v.iter() {
        println!("wait wtih tid {}", tid);
        let mut exit_code = 0;
        let exit_code = wait_tid(*tid, &mut exit_code);
        println!("thread#{} exited with code {}", tid, exit_code);
    }
    println!("main thread exited.");
    0
}