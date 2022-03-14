use clap::{Parser, Subcommand};

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
        Commands::GetPPID {} => playground::ppid::getppid(),
        Commands::Fork {} => playground::fork::fork(),
        Commands::ForkAndExec {} => playground::fork_and_exec::fork_and_exec(),
    }
}
