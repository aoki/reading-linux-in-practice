use nix::unistd::{getpid, ForkResult};

fn main() {
    match unsafe { nix::unistd::fork() } {
        Ok(ForkResult::Parent { child, .. }) => {
            println!(
                "I'm parent! my pid is {} and the pid of my child is {}.",
                getpid(),
                child
            )
        }
        Ok(ForkResult::Child) => println!("I'm child! my pid is {}.", getpid()),
        Err(_) => eprintln!("fork() failed."),
    };
}
