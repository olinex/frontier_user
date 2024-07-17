#![no_std]
#![no_main]

#[macro_use]
extern crate frontier_user;

use frontier_user::read;

#[no_mangle]
pub fn main() -> i32 {
    let mut buf = [0u8; 256];
    let mut lines = 0usize;
    let mut total_size = 0usize;
    loop {
        let len = read(0, &mut buf) as usize;
        if len == 0 {
            break;
        }
        total_size += len;
        let string = core::str::from_utf8(&buf[..len]).unwrap();
        lines += string
            .chars()
            .fold(0, |acc, c| acc + if c == '\n' { 1 } else { 0 });
    }
    if total_size > 0 {
        lines += 1;
    }
    println!("{}", lines);
    0
}
