// @author:    olinex
// @time:      2024/06/15

// self mods

// use other mods
use frontier_lib::model::signal::Signal;

// use self mods
use crate::{get_pid, kill};

#[panic_handler]
fn panic_handler(panic_info: &core::panic::PanicInfo) -> ! {
    let err = panic_info.message().unwrap();
    if let Some(location) = panic_info.location() {
        println!(
            "Panicked at {}:{}, {}",
            location.file(),
            location.line(),
            err
        );
    } else {
        println!("Panicked: {}", err);
    }
    kill(get_pid() as usize, Signal::ABRT);
    unreachable!()
}

#[macro_export]
macro_rules! vstore {
    ($var_ref: expr, $value: expr) => {
        unsafe { core::intrinsics::volatile_store($var_ref as *const _ as _, $value) }
    };
}

#[macro_export]
macro_rules! vload {
    ($var_ref: expr) => {
        unsafe { core::intrinsics::volatile_load($var_ref as *const _ as _) }
    };
}

#[macro_export]
macro_rules! memory_fence {
    () => {
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst)
    };
}
