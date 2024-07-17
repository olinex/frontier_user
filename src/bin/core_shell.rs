// @author:    olinex
// @time:      2023/10/11
#![no_std]
#![no_main]

// extern other crate
#[macro_use]
extern crate frontier_user;

extern crate alloc;

// self mods

// use other mods
use alloc::format;
use alloc::string::{String, ToString};
use frontier_fs::OpenFlags;
use frontier_user::console::get_console_char;
use frontier_user::constant::{charater, descriptor, keycode, shortcut};
use frontier_user::{close, dup, exec, exit, fork, open, wait_pid};

// use self mods

fn dup_file(path: &str, flags: OpenFlags, close_fd: usize) -> Option<isize> {
    if path.is_empty() {
        println!("Empty ouput file path");
        return Some(-4);
    }
    let other_fd = open(path, flags);
    if other_fd == -1 {
        println!("Error when opening file: {}", path);
        return Some(-4);
    }
    // be carefully, if you close the stdout,
    // any print call by this process will failed.
    let other_fd = other_fd as usize;
    if close(close_fd) == -1 {
        println!("Error when close fd: {}", close_fd);
        return Some(-4);
    }
    let dup_fd = dup(other_fd) as usize;
    if dup_fd != close_fd {
        println!("Error when dup file");
        return Some(-4);
    }
    if close(other_fd) == -1 {
        println!("Error when close dup file");
        return Some(-4);
    }
    return None;
}

fn ensure_command(main_line: &mut String) -> Option<isize> {
    let result = match main_line.as_str() {
        "exit" => exit(0),
        shortcut::EMPTY => None,
        other => {
            let pid = fork();
            if pid == 0 {
                let (main_line, output_file, input_file) =
                    if let Some((main_line, output)) = other.split_once(charater::GT) {
                        (main_line.trim().to_string(), Some(output.trim()), None)
                    } else if let Some((main_line, input)) = other.split_once(charater::LT) {
                        (main_line.trim().to_string(), None, Some(input.trim()))
                    } else {
                        (main_line.clone(), None, None)
                    };
                if let Some(path) = output_file {
                    if let Some(code) =
                        dup_file(path, OpenFlags::RW | OpenFlags::CREATE, descriptor::STDOUT)
                    {
                        return Some(code);
                    }
                }
                if let Some(path) = input_file {
                    if let Some(code) = dup_file(path, OpenFlags::READ, descriptor::STDIN) {
                        return Some(code);
                    };
                }
                let (path, args) = if let Some((path, args)) = main_line.split_once(charater::SPACE)
                {
                    (path.trim(), args.trim())
                } else {
                    (main_line.trim(), "")
                };
                let path = format!("{}\0", path);
                let args = format!("{}\0", args);
                // child process
                if exec(&path, &args) == -1 {
                    println!("Error when executing!");
                    return Some(-4);
                }
                unreachable!();
            } else {
                let mut exit_code: i32 = 0;
                let exit_pid = wait_pid(pid as usize, &mut exit_code);
                assert_eq!(pid, exit_pid);
                println!("Process {} exited with code {}", pid, exit_code);
                None
            }
        }
    };
    main_line.clear();
    result
}

fn run_as_escaped_mode(c: char, _: &mut String) -> Option<i32> {
    match c as u8 {
        keycode::UP_ARROW | keycode::RIGHT_ARROW | keycode::DOWN_ARROW => None,
        _ => None,
    }
}

fn run_as_normal_mode(c: char, line: &mut String) -> Option<i32> {
    match c as u8 {
        keycode::LINE_FEED | keycode::CARRIAGE_RETURN => {
            println!("");
            if let Some(error_code) = ensure_command(line) {
                return Some(error_code as i32);
            }
            print!(">> ");
        }
        keycode::BACK_SPACE | keycode::DELETE => {
            if !line.is_empty() {
                print!(
                    "{} {}",
                    keycode::BACK_SPACE as char,
                    keycode::BACK_SPACE as char
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
fn main(_: &str, _: &str) -> i32 {
    println!("Rust Core Shell...");
    let mut line: String = String::new();
    let mut control_mode = false;
    print!(">> ");
    loop {
        let c = get_console_char();
        let result = match (c as u8, control_mode) {
            (keycode::CHANGE_MODE, false) => {
                control_mode = true;
                None
            }
            (c, true) => {
                control_mode = false;
                run_as_escaped_mode(c as char, &mut line)
            }
            (c, false) => run_as_normal_mode(c as char, &mut line),
        };
        if let Some(exit_code) = result {
            return exit_code;
        }
    }
}
