// @author:    olinex
// @time:      2024/04/08

// self mods

// use other mods
use core::arch::asm;

// use self mods
use frontier_lib::{
    constant::sysid,
    model::signal::{Signal, SignalAction, SignalFlags},
};

#[inline(always)]
pub fn sys_dup(fd: usize) -> isize {
    syscall(sysid::DUP, [fd, 0, 0])
}

fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut ret: isize;
    unsafe {
        asm!(
            "ecall",
            inlateout("x10") args[0] => ret,
            in("x11") args[1],
            in("x12") args[2],
            in("x17") id
        );
    }
    ret
}

#[inline(always)]
pub fn sys_open(path: &str, flags: u32) -> isize {
    syscall(sysid::OPEN, [path.as_ptr() as usize, flags as usize, 0])
}

#[inline(always)]
pub fn sys_close(fd: usize) -> isize {
    syscall(sysid::CLOSE, [fd, 0, 0])
}

#[inline(always)]
pub fn sys_pipe(read_tap_fd: &mut usize, write_tap_fd: &mut usize) -> isize {
    syscall(
        sysid::PIPE,
        [
            read_tap_fd as *mut usize as usize,
            write_tap_fd as *mut usize as usize,
            0,
        ],
    )
}

#[inline(always)]
pub fn sys_read(fd: usize, buffer: &mut [u8]) -> isize {
    syscall(
        sysid::READ,
        [fd, buffer.as_mut_ptr() as usize, buffer.len()],
    )
}

#[inline(always)]
pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    syscall(sysid::WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])
}

#[inline(always)]
pub fn sys_exit(exit_code: i32) -> isize {
    syscall(sysid::EXIT, [exit_code as usize, 0, 0])
}

#[inline(always)]
pub fn sys_yield() -> isize {
    syscall(sysid::YIELD, [0, 0, 0])
}

#[inline(always)]
pub fn sys_get_time() -> isize {
    syscall(sysid::GET_TIME, [0, 0, 0])
}

#[inline(always)]
pub fn sys_get_pid() -> isize {
    syscall(sysid::GET_PID, [0, 0, 0])
}

#[inline(always)]
pub fn sys_fork() -> isize {
    syscall(sysid::FORK, [0, 0, 0])
}

#[inline(always)]
pub fn sys_exec(path: &str, args: &str) -> isize {
    syscall(
        sysid::EXEC,
        [path.as_ptr() as usize, args.as_ptr() as usize, 0],
    )
}

#[inline(always)]
pub fn sys_wait_pid(pid: isize, exit_code: &mut i32) -> isize {
    syscall(
        sysid::WAIT_PID,
        [pid as usize, exit_code as *mut i32 as usize, 0],
    )
}

#[inline(always)]
pub fn sys_kill(pid: usize, signal: Signal) -> isize {
    syscall(sysid::KILL, [pid, signal as usize, 0])
}

#[inline(always)]
pub fn sys_sig_action(
    signal: Signal,
    new_action: *const SignalAction,
    old_action: *mut SignalAction,
) -> isize {
    syscall(
        sysid::SIG_ACTION,
        [signal as usize, new_action as usize, old_action as usize],
    )
}

#[inline(always)]
pub fn sys_sig_proc_mask(mask: SignalFlags) -> isize {
    syscall(sysid::SIG_PROC_MASK, [mask.bits() as usize, 0, 0])
}

#[inline(always)]
pub fn sys_sig_return() -> isize {
    syscall(sysid::SIG_RETURN, [0, 0, 0])
}
