#![no_std]
#![no_main]

#[macro_use]
extern crate frontier_user;
extern crate alloc;

use alloc::{string::{String, ToString}, vec::Vec};

// not in SUCC_TESTS & FAIL_TESTS
// count_lines, infloop, user_shell, usertests

// item of TESTS : app_name(argv_0), argv_1, argv_2, argv_3, exit_code
static SUCC_TESTS: &[(&str, &str, &str, &str, i32)] = &[
    ("adder_atomic\0", "\0", "\0", "\0", 0),
    ("adder_mutex_blocking\0", "\0", "\0", "\0", 0),
    ("adder_mutex_spin\0", "\0", "\0", "\0", 0),
    ("adder_peterson_spin\0", "\0", "\0", "\0", 0),
    ("adder_peterson_yield\0", "\0", "\0", "\0", 0),
    ("barrier_condvar\0", "\0", "\0", "\0", 0),
    ("barrier_fail\0", "\0", "\0", "\0", 0),
    ("cmdline_args\0", "\0", "\0", "\0", 0),
    ("condsync_condvar\0", "\0", "\0", "\0", 0),
    ("condsync_sem\0", "\0", "\0", "\0", 0),
    ("early_exit\0", "\0", "\0", "\0", 0),
    ("early_exit2\0", "\0", "\0", "\0", 0),
    ("eisenberg\0", "\0", "\0", "\0", 0),
    ("exit_test\0", "\0", "\0", "\0", 0),
    ("fantastic_text\0", "\0", "\0", "\0", 0),
    ("filetest_simple\0", "\0", "\0", "\0", 0),
    ("forktest_simple\0", "\0", "\0", "\0", 0),
    ("forktest\0", "\0", "\0", "\0", 0),
    ("forktest2\0", "\0", "\0", "\0", 0),
    ("forktree\0", "\0", "\0", "\0", 0),
    ("hello_world\0", "\0", "\0", "\0", 0),
    ("huge_write\0", "\0", "\0", "\0", 0),
    ("matrix\0", "\0", "\0", "\0", 0),
    ("mpsc_sem_blocking\0", "\0", "\0", "\0", 0),
    ("mpsc_sem_spin\0", "\0", "\0", "\0", 0),
    ("phil_din_mutex\0", "\0", "\0", "\0", 0),
    ("pipe_large_test\0", "\0", "\0", "\0", 0),
    ("pipe_test\0", "\0", "\0", "\0", 0),
    ("run_pipe_test\0", "\0", "\0", "\0", 0),
    ("sig_simple\0", "\0", "\0", "\0", 0),
    ("sleep_simple\0", "\0", "\0", "\0", 0),
    ("sleep\0", "\0", "\0", "\0", 0),
    ("stackful_coroutine\0", "\0", "\0", "\0", 0),
    ("stackless_coroutine\0", "\0", "\0", "\0", 0),
    ("sync_sem_blocking\0", "\0", "\0", "\0", 0),
    ("sync_sem_spin\0", "\0", "\0", "\0", 0),
    ("thread_test\0", "\0", "\0", "\0", 0),
    ("yield_out\0", "\0", "\0", "\0", 0),
];

static FAIL_TESTS: &[(&str, &str, &str, &str, i32)] = &[
    ("adder\0", "\0", "\0", "\0", -6),
    ("adder_simple_spin\0", "\0", "\0", "\0", -6),
    ("adder_simple_yield\0", "\0", "\0", "\0", -6),
    ("priv_csr\0", "\0", "\0", "\0", -4),
    ("priv_inst\0", "\0", "\0", "\0", -4),
    ("stack_overflow\0", "\0", "\0", "\0", -11),
    ("store_fault\0", "\0", "\0", "\0", -11),
];

use frontier_user::{exec, fork, wait_pid};

fn run_tests(tests: &[(&str, &str, &str, &str, i32)]) -> (i32, Vec<String>) {
    let mut pass_num = 0;
    let mut arr: [*const u8; 4] = [
        core::ptr::null::<u8>(),
        core::ptr::null::<u8>(),
        core::ptr::null::<u8>(),
        core::ptr::null::<u8>(),
    ];
    let mut failed_tests = Vec::new();
    for test in tests {
        println!("Usertests: Running {}", test.0);
        arr[0] = test.0.as_ptr();
        if test.1 != "\0" {
            arr[1] = test.1.as_ptr();
            arr[2] = core::ptr::null::<u8>();
            arr[3] = core::ptr::null::<u8>();
            if test.2 != "\0" {
                arr[2] = test.2.as_ptr();
                arr[3] = core::ptr::null::<u8>();
                if test.3 != "\0" {
                    arr[3] = test.3.as_ptr();
                } else {
                    arr[3] = core::ptr::null::<u8>();
                }
            } else {
                arr[2] = core::ptr::null::<u8>();
                arr[3] = core::ptr::null::<u8>();
            }
        } else {
            arr[1] = core::ptr::null::<u8>();
            arr[2] = core::ptr::null::<u8>();
            arr[3] = core::ptr::null::<u8>();
        }

        let pid = fork();
        if pid == 0 {
            exec(test.0, "\0");
            panic!("unreachable!");
        } else {
            let mut exit_code: i32 = Default::default();
            let wait_pid = wait_pid(pid as usize, &mut exit_code);
            assert_eq!(pid, wait_pid);
            if exit_code == test.4 {
                // summary apps with  exit_code
                pass_num = pass_num + 1;
            } else {
                failed_tests.push(test.0.to_string())
            }
            println!(
                "\x1b[32mUsertests: Test {} in Process {} exited with code {}\x1b[0m",
                test.0, pid, exit_code
            );
        }
    }
    (pass_num, failed_tests)
}

#[no_mangle]
pub fn main(_: &str, _: &str) -> i32 {
    let (succ_num, failed_tests) = run_tests(SUCC_TESTS);
    let (err_num, succed_tests) = run_tests(FAIL_TESTS);
    if succ_num == SUCC_TESTS.len() as i32 && err_num == FAIL_TESTS.len() as i32 {
        println!(
            "{} of sueecssed apps, {} of failed apps run correctly. \nUsertests passed!",
            SUCC_TESTS.len(),
            FAIL_TESTS.len()
        );
        return 0;
    }
    if succ_num != SUCC_TESTS.len() as i32 {
        println!(
            "all successed app_num is  {} , but only  passed {}",
            SUCC_TESTS.len(),
            succ_num
        );
        println!("except success but failed tests: {}", failed_tests.join(","))
    }
    if err_num != FAIL_TESTS.len() as i32 {
        println!(
            "all failed app_num is  {} , but only  passed {}",
            FAIL_TESTS.len(),
            err_num
        );
        println!("except fail but successed tests: {}", succed_tests.join(","))
    }
    println!(" Usertests failed!");
    return -1;
}
