use clap::{Parser, Subcommand};
use nix::unistd::{execve, fork, getpid, ForkResult};
use std::ffi::CString;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Get ppid and loop. No print
    GetPPID {},

    /// Fork process
    Fork {},

    /// Frok and Exec
    ForkAndExec {},
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::GetPPID {} => {
            println!("Get PPID. Ctrl + C to stop. ");
            loop {
                nix::unistd::getppid();
            }
        }
        Commands::Fork {} => match unsafe { fork() } {
            Ok(ForkResult::Parent { child, .. }) => {
                println!(
                    "I'm parent! my pid is {} and the pid of my child is {}.",
                    getpid(),
                    child
                );
            }
            Ok(ForkResult::Child) => println!("I'm child! my pid is {}.", getpid()),
            Err(_) => eprintln!("fork() failed."),
        },
        Commands::ForkAndExec {} => match unsafe { fork() } {
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
        },
    }
}
