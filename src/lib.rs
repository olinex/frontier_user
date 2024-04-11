// @author:    olinex
// @time:      2023/10/11
#![no_std]
#![feature(linkage)]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]
#![feature(ascii_char_variants)]

extern crate alloc;

// self mods
#[macro_use]
pub mod console;

pub mod configs;
pub mod constant;
pub mod heap;
pub mod lang_items;
pub mod syscall;
pub mod usercall;

// use other mods

// use self mods

// reexports
pub use usercall::*;

#[inline(always)]
pub fn convert_to_str<'a>(count: usize, mut base_addr: usize) -> (&'a str, &'a str) {
    let path = if count >= 1 {
        let length = unsafe { *(base_addr as *const usize) };
        base_addr += core::mem::size_of::<usize>();
        let slice = unsafe { core::slice::from_raw_parts(base_addr as *const u8, length) };
        base_addr += length;
        core::str::from_utf8(slice).unwrap()
    } else {
        ""
    };
    let args = if count == 2 {
        let length = unsafe { *(base_addr as *const usize) };
        base_addr += core::mem::size_of::<usize>();
        let slice = unsafe { core::slice::from_raw_parts(base_addr as *const u8, length) };
        core::str::from_utf8(slice).unwrap()
    } else {
        ""
    };
    (path, args)
}

#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start(count: usize, base_addr: usize) -> ! {
    clear_bss();
    heap::init_heap();
    let (path, args) = convert_to_str(count, base_addr);
    exit(main(path, args));
    panic!("unreachable after sys_exit!");
}

#[linkage = "weak"]
#[no_mangle]
fn main(_: &str, _: &str) -> i32 {
    panic!("Cannot find main!");
}

fn clear_bss() {
    extern "C" {
        fn start_bss();
        fn end_bss();
    }
    (start_bss as usize..end_bss as usize).for_each(|addr| unsafe {
        (addr as *mut u8).write_volatile(0);
    });
}
