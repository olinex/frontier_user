use core::arch::asm;

mod ids {
    pub const OPEN: usize = 56;
    pub const CLOSE: usize = 57;
    pub const READ: usize = 63;
    pub const WRITE: usize = 64;
    pub const EXIT: usize = 93;
    pub const YIELD: usize = 124;
    pub const GET_TIME: usize = 169;
    pub const GET_PID: usize = 172;
    pub const FORK: usize = 220;
    pub const EXEC: usize = 221;
    pub const WAIT_PID: usize = 260;
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
    syscall(ids::OPEN, [path.as_ptr() as usize, flags as usize, 0])
}

#[inline(always)]
pub fn sys_close(fd: usize) -> isize {
    syscall(ids::CLOSE, [fd, 0, 0])
}

#[inline(always)]
pub fn sys_read(fd: usize, buffer: &mut [u8]) -> isize {
    syscall(ids::READ, [fd, buffer.as_mut_ptr() as usize, buffer.len()])
}

#[inline(always)]
pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    syscall(ids::WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])
}

#[inline(always)]
pub fn sys_exit(exit_code: i32) -> isize {
    syscall(ids::EXIT, [exit_code as usize, 0, 0])
}

#[inline(always)]
pub fn sys_yield() -> isize {
    syscall(ids::YIELD, [0, 0, 0])
}

#[inline(always)]
pub fn sys_get_time() -> isize {
    syscall(ids::GET_TIME, [0, 0, 0])
}

#[inline(always)]
pub fn sys_get_pid() -> isize {
    syscall(ids::GET_PID, [0, 0, 0])
}

#[inline(always)]
pub fn sys_fork() -> isize {
    syscall(ids::FORK, [0, 0, 0])
}

#[inline(always)]
pub fn sys_exec(path: &str) -> isize {
    syscall(ids::EXEC, [path.as_ptr() as usize, 0, 0])
}

#[inline(always)]
pub fn sys_wait_pid(pid: isize, exit_code: &mut i32) -> isize {
    syscall(
        ids::WAIT_PID,
        [pid as usize, exit_code as *mut i32 as usize, 0],
    )
}
