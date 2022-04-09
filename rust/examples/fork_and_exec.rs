use nix::unistd::{execve, fork, getpid, ForkResult};
use std::ffi::CString;

fn main() {
    match unsafe { fork() } {
        Ok(ForkResult::Parent { child, .. }) => {
            println!(
                "I'm parent! my pid is {} and the pid of my child is {}.",
                getpid(),
                child
            );
        }
        Ok(ForkResult::Child) => {
            println!("I'm child! my pid is {}.", getpid());
            let path = CString::new("/bin/echo").unwrap();
            let args = [&path, &CString::new("hello").unwrap()];
            execve(&path, &args, &[CString::new("").unwrap()]).expect("execve() failed.");
        }
        Err(_) => eprintln!("fork() failed."),
    }
}
