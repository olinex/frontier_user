// @author:    olinex
// @time:      2024/03/21
#![no_std]
#![no_main]

// extern other crate
#[macro_use]
extern crate frontier_user;

// self mods

// use other mods
use frontier_user::{close, fork, pipe, read, wait_pid, write};

// use self mods

static STR: &str = "Hello, world!";

#[no_mangle]
pub fn main(_: &str, _: &str) -> i32 {
    // create pipe
    let mut read_tap_fd = 0;
    let mut write_tap_fd = 0;
    pipe(&mut read_tap_fd, &mut write_tap_fd);
    // read end
    assert_eq!(read_tap_fd, 3);
    // write end
    assert_eq!(write_tap_fd, 4);
    match fork() {
        0 => {
            // child process, read from parent
            // close write_end
            close(write_tap_fd);
            let mut buffer = [0u8; 32];
            let len_read = read(read_tap_fd, &mut buffer) as usize;
            // close read_end
            close(read_tap_fd);
            assert_eq!(core::str::from_utf8(&buffer[..len_read]).unwrap(), STR);
            println!("Read OK, child process exited!");
            0
        }
        child_pid => {
            // parent process, write to child
            // close read end
            close(read_tap_fd);
            assert_eq!(write(write_tap_fd, STR.as_bytes()), STR.len() as isize);
            // close write end
            close(write_tap_fd);
            let mut child_exit_code: i32 = 0;
            wait_pid(child_pid as usize, &mut child_exit_code);
            assert_eq!(child_exit_code, 0);
            println!("pipetest passed!");
            0
        }
    }
}
