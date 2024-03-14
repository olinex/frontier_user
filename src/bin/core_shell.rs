#![no_std]
#![no_main]

#[macro_use]
extern crate frontier_user;

extern crate alloc;

// @author:    olinex
// @time:      2023/10/11

// self mods

// use other mods
use alloc::string::String;
use frontier_user::console::get_console_char;
use frontier_user::constant::ascii;
use frontier_user::{exec, exit, fork, wait_pid};

// use self mods

fn ensure_command(line: &mut String) -> Option<isize> {
    line.push(ascii::NULL as char);
    let result = match line.as_str() {
        "\0" => None,
        "exit\0" => Some(exit(0)),
        other => {
            let pid = fork();
            if pid == 0 {
                // child process
                if exec(other) == -1 {
                    println!("Error when executing!");
                    Some(-4)
                } else {
                    unreachable!();
                }
            } else {
                let mut exit_code: i32 = 0;
                let exit_pid = wait_pid(pid as usize, &mut exit_code);
                assert_eq!(pid, exit_pid);
                println!("Shell: Process {} exited with code {}", pid, exit_code);
                None
            }
        }
    };
    line.clear();
    result
}

fn run_as_escaped_mode(c: char, _: &mut String) -> Option<i32> {
   match c as u8 {
    ascii::LEFT_ARROW | ascii::UP_ARROW | ascii::RIGHT_ARROW | ascii::DOWN_ARROW => {
        None
    }
    _ => None
   } 
}

fn run_as_normal_mode(c: char, line: &mut String) -> Option<i32> {
    match c as u8 {
        ascii::LINE_FEED | ascii::CARRIAGE_RETURN => {
            println!("");
            if let Some(error_code) = ensure_command(line) {
                return Some(error_code as i32);
            }
            print!(">> ");
        }
        ascii::BACK_SPACE | ascii::DELETE => {
            if !line.is_empty() {
                print!(
                    "{} {}",
                    ascii::BACK_SPACE as char,
                    ascii::BACK_SPACE as char
                );
                line.pop();
            }
        }
        _ => {
            print!("{}", c);
            line.push(c);
        }
    }
    None
}

#[no_mangle]
fn main() -> i32 {
    println!("Rust Core Shell...");
    let mut line: String = String::new();
    let mut control_mode = false;
    print!(">> ");
    loop {
        let c = get_console_char();
        let result = match (c as u8, control_mode) {
            (ascii::CHANGE_MODE, false) => {
                println!("as control mode");
                control_mode = true;
                None
            }
            (c, true) => {
                control_mode = false;
                run_as_escaped_mode(c as char, &mut line)
            }
            (c, false) => {
                run_as_normal_mode(c as char, &mut line)
            },
        };
        if let Some(exit_code) = result {
            return exit_code;
        }
    }
}
