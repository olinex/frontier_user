// @author:    olinex
// @time:      2024/03/25
#![no_std]
#![no_main]

// extern other crate
#[macro_use]
extern crate frontier_user;
extern crate alloc;

// self mods

// use other mods
use alloc::string::ToString;
use frontier_fs::OpenFlags;
use frontier_user::{close, constant::charater, open, read};

// use self mods

#[no_mangle]
pub fn main(_: &str, args: &str) -> i32 {
    let mut child_path = args
        .split_ascii_whitespace()
        .nth(0)
        .unwrap()
        .trim()
        .to_string();
    child_path.push(charater::NULL);
    let fd = open(&child_path, OpenFlags::READ);
    if fd == -1 {
        panic!("Error occurred when opening file");
    }
    let fd = fd as usize;
    let mut buf = [0u8; 128];
    loop {
        let size = read(fd, &mut buf) as usize;
        if size == 0 {
            break;
        }
        print!("{}", core::str::from_utf8(&buf[..size]).unwrap());
    }
    if close(fd) != 0 {
        panic!("Error occurred when closing file");
    };
    0
}
