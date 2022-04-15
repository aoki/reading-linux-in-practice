use nix::{
    libc::{system, EXIT_FAILURE},
    unistd::getpid,
};
use std::{os::unix::prelude::CommandExt, process::Command};

fn main() {
    let pid = getpid();
    println!("{}", pid);
    let mut command = Command::new("cat");
    command.arg(format!("/proc/{}/maps", pid));

    println!("*** memory map before mapping file ***");
    let output = command.output().expect("failed to execute command");
    let o = std::str::from_utf8(&output.stdout).unwrap();
    println!("{}", o);
}
