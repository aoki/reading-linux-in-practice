use nix::sys::mman::ProtFlags;
use nix::{
    libc::EXIT_FAILURE,
    sys::mman::{mmap, MapFlags},
};
use std::env;
use nix::time::{clock_gettime, ClockId};
use nix::libc::CLOCK_MONOTONIC;
use nix::sys::time::TimeSpec;

fn main() {
    let argv: Vec<String> = env::args().collect();

    let progname = &argv[0];

    if argv.len() != 2 {
        eprintln!("usage: {} <size[KB]>", progname);
        std::process::exit(EXIT_FAILURE);
    }

    let size = match argv[1].parse::<usize>() {
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
            size,
            ProtFlags::PROT_READ | ProtFlags::PROT_WRITE,
            MapFlags::MAP_PRIVATE | MapFlags::MAP_ANONYMOUS,
            -1,
            0,
        )
    };
    let buffer: *mut char = match buffer {
        Ok(buf) => buf as *mut char,
        Err(e) => {
            eprintln!("mmap() failed: {}", e);
            std::process::exit(EXIT_FAILURE);
        },
    }

    let before = get_time();
}

fn get_time() -> TimeSpec {
    match clock_gettime(ClockId::CLOCK_MONOTONIC) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("clock_gettime() failed: {}", e);
            std::process::exit(EXIT_FAILURE);
        },
    }
}
