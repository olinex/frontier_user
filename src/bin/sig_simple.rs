#![no_std]
#![no_main]

#[macro_use]
extern crate frontier_user;

extern crate frontier_lib;

use frontier_lib::model::signal::{Signal, SignalAction, SignalFlags};
use frontier_user::usercall::{get_pid, kill, exit, sig_action, sig_return};

fn func() {
    println!("user_sig_test passed");
    sig_return();
}

#[no_mangle]
pub fn main() -> i32 {
    let new = SignalAction::new(func as usize, SignalFlags::empty());
    let mut old = SignalAction::default();

    println!("signal_simple: sigaction");
    if sig_action(Signal::USR1, Some(&new), Some(&mut old)) < 0 {
        panic!("Sigaction failed!");
    }
    println!("signal_simple: kill");
    if kill(get_pid() as usize, Signal::USR1) < 0 {
        println!("Kill failed!");
        exit(1);
    }
    println!("signal_simple: Done");
    0
}
