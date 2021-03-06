use anyhow::anyhow;
use anyhow::bail;
use nix::fcntl::open;
use nix::fcntl::OFlag;
use nix::libc::memcpy;
use nix::libc::EXIT_FAILURE;
use nix::libc::EXIT_SUCCESS;
use nix::sys::mman::munmap;
use nix::sys::mman::ProtFlags;
use nix::sys::mman::{mmap, MapFlags};
use nix::sys::stat::Mode;
use nix::unistd::close;
use nix::unistd::{getpid, Pid};
use std::ffi::c_void;
use std::ffi::CStr;
use std::process::Command;

// 100 MB
const ALLOC_SIZE: usize = 100 * 1024 * 1024;
const OVERWRITE_DATA: &str = "HELLO";

fn close_file(fd: i32) {
    if let Err(e) = close(fd) {
        eprintln!("close() failed: {}", e);
        std::process::exit(EXIT_SUCCESS);
    }
}

fn display_memorymap(pid: Pid) {
    let res = Command::new("cat")
        .arg(format!("/proc/{}/maps", pid))
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

fn open_file(file_name: &str) -> i32 {
    match open(file_name, OFlag::O_RDWR, Mode::empty()) {
        Ok(fd) => fd,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(EXIT_FAILURE)
        }
    }
}

/// # ファイルマップの動作確認プログラム
/// - ファイルが仮想アドレス空間にマップされていること
/// - マップされた領域の読み出しによって、ファイルを読み出せること
/// - マップされた領域への書き込みによって、ファイルに書き込めること
///
/// ## 仕様
/// 1. プロセスのメモリマップ情報を出力
/// 2. `testfile` ファイルを開く
/// 3. ファイルを `mmap()` によってメモリ空間にマップする
/// 4. プロセスのメモリマップ情報を再度表示する
/// 5. マップされた領域のデータを読み出して出力する
/// 6. マップされた領域のデータを書き換える
///
/// ## Usage
/// ```shellsession
/// $ printf "hello world" > testfile; cargo run --example filemap
/// ```
fn main() {
    let pid = getpid();

    // 1. プロセスのメモリマップ情報を出力
    println!("*** memory map before mapping file ***");
    display_memorymap(pid);

    // 2. `testfile` ファイルを開く
    let fd = open_file("testfile");

    // 3. ファイルを `mmap()` によってメモリ空間にマップする
    // `MapFlags::MAP_PRIVATE` を指定すると、メモリからファイルに書き戻さない,
    let file_contents = unsafe {
        mmap(
            std::ptr::null_mut(),
            ALLOC_SIZE,
            ProtFlags::PROT_READ | ProtFlags::PROT_WRITE,
            MapFlags::MAP_SHARED,
            fd,
            0,
        )
    };

    let file_contents = match file_contents {
        Ok(fc) => unsafe { CStr::from_ptr(fc as *const _) }.to_string_lossy(),
        Err(e) => {
            println!("mmap() failed: {}", e);
            close_file(fd);
            std::process::exit(EXIT_FAILURE);
        }
    };

    println!(
        "*** succeeded to map file: address = {:p}; size = {} ***",
        file_contents.as_ptr(),
        ALLOC_SIZE
    );

    // 4. プロセスのメモリマップ情報を再度表示する
    println!("*** memory map after mapping file ***");
    display_memorymap(pid);

    // 5. マップされた領域のデータを読み出して出力する
    println!(
        "*** file contents before overwrite mapped region: {} ***",
        file_contents
    );

    // 6. マップされた領域のデータを書き換える
    unsafe {
        memcpy(
            file_contents.as_ptr() as *mut c_void,
            OVERWRITE_DATA.as_ptr() as *const c_void,
            OVERWRITE_DATA.len(),
        );
    }

    println!("*** overwitten mapped region with: {} ***", file_contents);

    unsafe {
        if let Err(e) = munmap(file_contents.as_ptr() as *mut c_void, ALLOC_SIZE) {
            eprintln!("{}", e);
        }
    }

    close_file(fd);

    std::process::exit(EXIT_SUCCESS);
}
