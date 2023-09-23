use anyhow::Result;
use clap::{CommandFactory, Parser, Subcommand};
use colored::Colorize;

mod output_args;

mod subcommands;
use crate::subcommands::*;

const VERSION_STRING: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Parser)]
#[clap(author, about)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Subcommands>,
    #[clap(long = "version", short = 'V', help = "Print version info and exit")]
    version: bool,
}

#[derive(Debug, Subcommand)]
enum Subcommands {
    #[clap(about = "Fetch ABI from the chain")]
    Fetch(Fetch),
    #[clap(about = "Extract ABI from Sierra file")]
    FromSierra(FromSierra),
}

#[tokio::main]
async fn main() {
    if let Err(err) = run_command(Cli::parse()).await {
        eprintln!("{}", format!("Error: {err}").red());
        std::process::exit(1);
    }
}

async fn run_command(cli: Cli) -> Result<()> {
    match (cli.version, cli.command) {
        (false, None) => Ok(Cli::command().print_help()?),
        (true, _) => {
            println!("{}", VERSION_STRING);
            Ok(())
        }
        (false, Some(command)) => match command {
            Subcommands::Fetch(cmd) => cmd.run().await,
            Subcommands::FromSierra(cmd) => cmd.run().await,
        },
    }
}
