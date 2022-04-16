use anyhow::anyhow;
use anyhow::bail;
use nix::fcntl::open;
use nix::fcntl::OFlag;
use nix::sys::stat::Mode;
use nix::unistd::{getpid, Pid};
use std::process::Command;

fn cat_proc(pid: Pid) -> anyhow::Result<String> {
    Command::new("cat")
        .arg(format!("/proc/{}/maps", pid))
        .output()
        .map_err(|e| anyhow!(e))
        .and_then(|ret| match ret.status.success() {
            true => String::from_utf8(ret.stdout).map_err(|e| anyhow!(e)),
            false => String::from_utf8(ret.stderr)
                .map_err(|e| anyhow!(e))
                .and_then(|std_err| bail!(std_err)),
        })
}

fn main() {
    let pid = getpid();

    match cat_proc(pid) {
        Ok(stdout) => println!("{}", stdout),
        Err(stderr) => {
            eprintln!("{}", stderr);
            std::process::exit(nix::libc::EXIT_FAILURE);
        }
    }

    println!("*** memory map before mapping file ***");

    let r = open("testfile", OFlag::O_RDWR, Mode::empty());
    println!("{:?}", r);
}
