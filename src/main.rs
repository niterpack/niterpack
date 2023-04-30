extern crate core;

mod format;
mod logger;
mod modrinth;
mod ops;
mod project;
mod subcommand;

pub use project::{Project, Mod};

use crate::subcommand::Subcommand;
use clap::{command, Parser};

#[derive(clap::Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Subcommand,
}

fn main() -> eyre::Result<()> {
    logger::init();

    let cli = Cli::parse();
    cli.command.run()?;

    Ok(())
}
