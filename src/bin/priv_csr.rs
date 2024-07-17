#![no_std]
#![no_main]

#[macro_use]
extern crate frontier_user;

#[macro_use]
extern crate cfg_if;

use riscv::register::sstatus::{self, SPP};

#[no_mangle]
fn main() -> i32 {
    println!("Try to access privileged CSR in U Mode");
    println!("Kernel should kill this application!");
    cfg_if! {
        if #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))] {
            unsafe {
                sstatus::set_spp(SPP::User);
            }
            0
        } else {
            compile_error!("Unknown target_arch");
        }
    }
}
