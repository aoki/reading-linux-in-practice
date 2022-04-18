use anyhow::{anyhow, bail};
use nix::libc::{malloc, EXIT_FAILURE};
use std::{ffi::c_void, process::Command};

const BUFFER_SIZE: usize = 100 * 1024 * 1024;

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
    println!("*** free memory info before malloc ***:");
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

    for i in 0..BUFFER_SIZE {
        unsafe {
            p.offset(i as isize).write_bytes(0, 1);
        }
    }

    println!("*** free memory info before fork ***:");
    display_memory_state();
}
