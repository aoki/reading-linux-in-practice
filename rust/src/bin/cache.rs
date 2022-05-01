use core::ffi::c_void;
use nix::sys::mman::ProtFlags;
use nix::sys::time::TimeSpec;
use nix::time::{clock_gettime, ClockId};
use nix::{
    libc::EXIT_FAILURE,
    sys::mman::{mmap, munmap, MapFlags},
};
use std::env;

const CACHE_LINE_SIZE_BYTE: usize = 64;
const NLOOP: usize = 4 * 1024 * 1024 * 1024;
const NSECS_PER_SEC: usize = 1_000_000_000;

fn main() {
    let argv: Vec<String> = env::args().collect();

    let progname = &argv[0];

    if argv.len() != 2 {
        eprintln!("usage: {} <size[KB]>", progname);
        std::process::exit(EXIT_FAILURE);
    }

    let size_byte = match argv[1].parse::<usize>() {
        Ok(s) => {
            if s <= 0 {
                eprintln!("size should be >= 1: {}", s);
                std::process::exit(EXIT_FAILURE);
            }
            s * 1024
        }
        Err(e) => {
            eprintln!("size should be a number: {}, {}", &argv[1], e);
            std::process::exit(EXIT_FAILURE);
        }
    };

    let buffer = unsafe {
        mmap(
            std::ptr::null_mut(),
            size_byte,
            ProtFlags::PROT_READ | ProtFlags::PROT_WRITE,
            MapFlags::MAP_PRIVATE | MapFlags::MAP_ANONYMOUS,
            -1,
            0,
        )
    };
    let buffer: *mut c_void = match buffer {
        Ok(buf) => buf,
        Err(e) => {
            eprintln!("mmap() failed: {}", e);
            std::process::exit(EXIT_FAILURE);
        }
    };

    let before = get_time();

    // for _ in 0..(NLOOP / (size_byte / CACHE_LINE_SIZE_BYTE)) {
    //     for j in (0..size_byte).step_by(CACHE_LINE_SIZE_BYTE) {
    //         unsafe {
    //             buffer
    //                 .offset(j as isize)
    //                 .write_bytes(0, CACHE_LINE_SIZE_BYTE);
    //         }
    //     }
    // }

    for _ in 0..(NLOOP / (size_byte / CACHE_LINE_SIZE_BYTE)) {
        for j in 0..(size_byte / CACHE_LINE_SIZE_BYTE) {
            unsafe {
                buffer
                    .offset((j * CACHE_LINE_SIZE_BYTE) as isize)
                    .write_bytes(0, CACHE_LINE_SIZE_BYTE);
            }
        }
    }

    let after = get_time();

    println!(
        "{}\t{}",
        &argv[1],
        diff_nsec(&before, &after) as f64 / NLOOP as f64
    );

    if let Err(e) = unsafe { munmap(buffer as *mut c_void, size_byte) } {
        eprintln!("mumap() failed: {}", e);
        std::process::exit(EXIT_FAILURE);
    };
}

fn diff_nsec(before: &TimeSpec, after: &TimeSpec) -> usize {
    (after.tv_sec() as usize * NSECS_PER_SEC + after.tv_nsec() as usize)
        - (before.tv_sec() as usize * NSECS_PER_SEC + before.tv_nsec() as usize)
}

fn get_time() -> TimeSpec {
    match clock_gettime(ClockId::CLOCK_MONOTONIC) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("clock_gettime() failed: {}", e);
            std::process::exit(EXIT_FAILURE);
        }
    }
}
