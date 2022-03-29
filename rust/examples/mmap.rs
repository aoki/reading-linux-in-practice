use nix::unistd::getpid;
use std::process::Command;

fn main() {
    let pid = getpid();
    println!("*** memory map before memory allocation ***");
    let mut cmd = Command::new("cat");
    cmd.arg(format!("/proc/{}/maps", pid));
    let output = cmd.output().expect("failed to execute command").stdout;
    let o = std::str::from_utf8(&output).unwrap();
    println!("{}", o);

    std::process::exit(nix::libc::EXIT_SUCCESS);
}
