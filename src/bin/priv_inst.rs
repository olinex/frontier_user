#![no_std]
#![no_main]

#[macro_use]
extern crate frontier_user;

#[macro_use]
extern crate cfg_if;

use core::arch::asm;

#[no_mangle]
fn main() -> i32 {
    println!("Try to execute privileged instruction in U Mode");
    println!("Kernel should kill this application!");
    cfg_if! {
        if #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))] {
            unsafe {
                asm!("sret");
            }
            0
        } else {
            compile_error!("Unknown target_arch");
        }
    }
}
