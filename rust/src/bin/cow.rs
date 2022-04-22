use anyhow::{anyhow, bail};
use nix::{
    libc::{malloc, EXIT_FAILURE, EXIT_SUCCESS},
    sys::wait::wait,
    unistd::{fork, getpid, ForkResult, Pid},
};
use std::{
    ffi::c_void,
    os::unix::prelude::{AsRawFd, FromRawFd},
    process::{Command, Stdio},
};

const BUFFER_SIZE: usize = 100 * 1024 * 1024;
const PAGE_SIZE: usize = 4096;

fn display_memory_state() {
    let res = Command::new("free")
        .output()
        .map_err(|e| anyhow!(e))
        .and_then(|ret| match ret.status.success() {
            true => String::from_utf8(ret.stdout).map_err(|e| anyhow!(e)),
            false => String::from_utf8(ret.stderr)
                .map_err(|e| anyhow!(e))
                .and_then(|std_err| bail!(std_err)),
        });
    match res {
        Ok(stdout) => println!("{}", stdout),
        Err(stderr) => {
            eprintln!("{}", stderr);
            std::process::exit(EXIT_FAILURE);
        }
    }
}

/// # Copy on Write の実験
/// - `frok()` システムコールの実行後、書き込みが行われるまで、メモリ領域は親プロセストコプロセスとで今日ううされている
/// - メモリ領域への書き込み時にはページフォルトが発生する
///
/// 1. 100M バイトのメモリを獲得して、すべてのページにアクセス
/// 2. システムシステムのメモリ使用量を確認する
/// 3. `fork()` システムコールを発行する
/// 4. 親プロセスと子プロセスはそれぞれ次のような動きをする
///     - 親プロセス
///       1. 子プロセスの終了を待つ
///     - 子プロセス
///       1. システムのメモリ使用量、および自身の仮想メモリ使用量、物理メモリ使用量、メジャーフォルトの回数、マイナーフォールとの回数を表示
///       2. 最初に獲得した領域のすべてのページにアクセス
///       3. システムのメモリ使用量、および自身の仮想メモリ使用量、物理メモリ使用量、メジャーフォルトの回数、マイナーフォールとの回数を表示
fn main() {
    println!("*** free memory info before malloc ***: {}", getpid());
    display_memory_state();

    let p: *mut c_void;
    unsafe {
        p = malloc(BUFFER_SIZE);
    }

    println!("*** free memory info before memory access ***:");
    display_memory_state();

    if p == std::ptr::null_mut() {
        eprintln!("malloc() failed");
        std::process::exit(EXIT_FAILURE);
    }

    for i in 0..(BUFFER_SIZE / PAGE_SIZE) {
        unsafe {
            p.offset(i as isize).write_bytes(0, 1);
        }
    }

    println!("*** free memory info before fork ***:");
    display_memory_state();

    match unsafe { fork() } {
        Ok(ForkResult::Parent { .. }) => parent_fn(),
        Ok(ForkResult::Child) => child_fn(p),
        Err(e) => {
            eprintln!("fork() failed.: {}", e);
            std::process::exit(EXIT_FAILURE)
        }
    }
}

fn grep(pid: Pid) {
    let ps = Command::new("ps")
        .args(["-o", "pid,comm,vsz,rss,min_flt,maj_flt"])
        // .stdout(Stdio::piped())
        .spawn();
    // .expect("failed to execute ps");

    // let grep = Command::new("grep")
    //     .arg(format!("'^ *{}'", pid))
    //     .stdin(unsafe { Stdio::from_raw_fd(ps.stdout.as_ref().unwrap().as_raw_fd()) })
    //     .output()
    //     .map_err(|e| anyhow!(e))
    //     .and_then(|ret| match ret.status.success() {
    //         true => String::from_utf8(ret.stdout).map_err(|e| anyhow!(e)),
    //         false => String::from_utf8(ret.stderr)
    //             .map_err(|e| anyhow!(e))
    //             .and_then(|std_err| bail!(std_err)),
    //     });
    match ps {
        Ok(stdout) => println!("{:?}", stdout),
        Err(stderr) => {
            eprintln!("{}", stderr);
            std::process::exit(EXIT_FAILURE);
        }
    }
}

fn child_fn(p: *mut c_void) {
    println!("*** child({}) ps info before memory access ***:", getpid());
    grep(getpid());

    println!("*** free memory info before memory access ***:");
    display_memory_state();

    for i in 0..(BUFFER_SIZE / PAGE_SIZE) {
        unsafe {
            p.offset(i as isize).write_bytes(0, PAGE_SIZE);
        }
    }

    println!("*** child ps info after memory access ***:");
    grep(getpid());

    println!("*** free memory info after memory access ***:");
    display_memory_state();

    std::process::exit(EXIT_SUCCESS)
}

fn parent_fn() {
    match wait() {
        Err(e) => {
            eprintln!("wait() failed: {:?}", e);
            std::process::exit(EXIT_FAILURE);
        }
        Ok(_) => std::process::exit(EXIT_SUCCESS),
    }
}
