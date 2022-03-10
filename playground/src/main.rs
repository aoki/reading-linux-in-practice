use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    GetPPID {},
    Fork {},
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
        Commands::Fork {} => {}
    }
}
