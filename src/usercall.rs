// @author:    olinex
// @time:      2023/11/02

// self mods

// use other mods
use frontier_fs::OpenFlags;

// use self mods
use crate::syscall::*;

#[inline(always)]
pub fn open(path: &str, flags: OpenFlags) -> isize {
    sys_open(path, flags.bits())
}

#[inline(always)]
pub fn close(fd: usize) -> isize {
    sys_close(fd)
}

#[inline(always)]
pub fn read(fd: usize, buf: &mut [u8]) -> isize {
    sys_read(fd, buf)
}

#[inline(always)]
pub fn write(fd: usize, buf: &[u8]) -> isize {
    sys_write(fd, buf)
}

#[inline(always)]
pub fn exit(exit_code: i32) -> isize {
    sys_exit(exit_code)
}

#[inline(always)]
pub fn yield_out() -> isize {
    sys_yield()
}

#[inline(always)]
pub fn get_time() -> isize {
    sys_get_time()
}

#[inline(always)]
pub fn get_pid() -> isize {
    sys_get_pid()
}

#[inline(always)]
pub fn fork() -> isize {
    sys_fork()
}

#[inline(always)]
pub fn exec(path: &str) -> isize {
    sys_exec(path)
}

pub fn wait(exit_code: &mut i32) -> isize {
    loop {
        match sys_wait_pid(-1, exit_code) {
            -2 => {
                yield_out();
            }
            // -1 or a real pid
            exit_pid => return exit_pid,
        }
    }
}

pub fn wait_pid(pid: usize, exit_code: &mut i32) -> isize {
    loop {
        match sys_wait_pid(pid as isize, exit_code) {
            -2 => {
                yield_out();
            }
            // -1 or a real pid
            exit_pid => return exit_pid,
        }
    }
}

pub fn sleep(period_ms: usize) {
    let start = sys_get_time();
    while sys_get_time() < start + period_ms as isize {
        sys_yield();
    }
}
