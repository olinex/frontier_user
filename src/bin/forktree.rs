#![no_std]
#![no_main]

#[macro_use]
extern crate frontier_user;
use frontier_user::{exit, fork, get_pid, sleep, yield_out};

const DEPTH: usize = 4;

fn fork_child(cur: &str, branch: char) {
    let mut next = [0u8; DEPTH + 1];
    let l = cur.len();
    if l >= DEPTH {
        return;
    }
    next[..l].copy_from_slice(cur.as_bytes());
    next[l] = branch as u8;
    if fork() == 0 {
        fork_tree(core::str::from_utf8(&next[..l + 1]).unwrap());
        yield_out();
        exit(0);
    }
}

fn fork_tree(cur: &str) {
    println!("pid{}: {}", get_pid(), cur);
    fork_child(cur, '0');
    fork_child(cur, '1');
}

#[no_mangle]
pub fn main(_: &str, _: &str) -> i32 {
    fork_tree("");
    sleep(3000);
    0
}
