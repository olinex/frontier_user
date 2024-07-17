#![no_std]
#![no_main]

#[macro_use]
extern crate frontier_user;

extern crate alloc;

use alloc::format;
use frontier_user::{close, fork, get_time, pipe, read, wait_any_pid, write};

const LENGTH: usize = 3000;
#[no_mangle]
pub fn main() -> i32 {
    // create pipes
    // parent write to child
    let mut down_read_fd = 0;
    let mut down_write_fd = 0;
    // child write to parent
    let mut up_read_fd = 0;
    let mut up_write_fd = 0;
    pipe(&mut down_read_fd, &mut down_write_fd);
    pipe(&mut up_read_fd, &mut up_write_fd);
    let mut random_str = [0u8; LENGTH];
    if fork() == 0 {
        // close write end of down pipe
        close(down_write_fd);
        // close read end of up pipe
        close(up_read_fd);
        assert_eq!(read(down_read_fd, &mut random_str) as usize, LENGTH);
        close(down_read_fd);
        let sum: usize = random_str.iter().map(|v| *v as usize).sum::<usize>();
        println!("sum = {}(child)", sum);
        let sum_str = format!("{}", sum);
        write(up_write_fd, sum_str.as_bytes());
        close(up_write_fd);
        println!("Child process exited!");
        0
    } else {
        // close read end of down pipe
        close(down_read_fd);
        // close write end of up pipe
        close(up_write_fd);
        // generate a long random string
        for ch in random_str.iter_mut() {
            *ch = get_time() as u8;
        }
        // send it
        assert_eq!(
            write(down_write_fd, &random_str) as usize,
            random_str.len()
        );
        // close write end of down pipe
        close(down_write_fd);
        // calculate sum(parent)
        let sum: usize = random_str.iter().map(|v| *v as usize).sum::<usize>();
        println!("sum = {}(parent)", sum);
        // recv sum(child)
        let mut child_result = [0u8; 32];
        let result_len = read(up_read_fd, &mut child_result) as usize;
        close(up_read_fd);
        // check
        assert_eq!(
            sum,
            str::parse::<usize>(core::str::from_utf8(&child_result[..result_len]).unwrap())
                .unwrap()
        );
        let mut exit_code: i32 = 0;
        let pid = wait_any_pid(&mut exit_code);
        println!("pipe_large_test passed! process {} exit with code {}", pid, exit_code);
        0
    }
}
