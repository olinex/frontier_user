// @author:    olinex
// @time:      2023/11/02

// self mods

// use other mods
use core::ptr::{null, null_mut};
use frontier_fs::OpenFlags;
use frontier_lib::model::signal::{Signal, SignalAction, SignalFlags};

// use self mods
use crate::syscall::*;

#[inline(always)]
pub fn dup(fd: usize) -> isize {
    sys_dup(fd)
}

#[inline(always)]
pub fn open(path: &str, flags: OpenFlags) -> isize {
    sys_open(path, flags.bits())
}

#[inline(always)]
pub fn close(fd: usize) -> isize {
    sys_close(fd)
}

#[inline(always)]
pub fn pipe(read_tap_fd: &mut usize, write_tap_fd: &mut usize) -> isize {
    sys_pipe(read_tap_fd, write_tap_fd)
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
pub fn exec(path: &str, args: &str) -> isize {
    sys_exec(path, args)
}

#[inline(always)]
pub fn exec_without_args(path: &str) -> isize {
    sys_exec(path, "\0")
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

pub fn kill(pid: usize, signal: Signal) -> isize {
    sys_kill(pid, signal)
}

pub fn sig_action(
    signal: Signal,
    new_action: Option<&SignalAction>,
    old_action: Option<&mut SignalAction>,
) -> isize {
    sys_sig_action(
        signal,
        new_action.map_or(null(), |a| a),
        old_action.map_or(null_mut(), |a| a),
    )
}

pub fn sig_proc_mask(mask: SignalFlags) -> isize {
    sys_sig_proc_mask(mask)
}

pub fn sig_return() -> isize {
    sys_sig_return()
}
