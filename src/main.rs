extern crate core;

mod format;
mod build;
mod logger;
mod project;
mod modrinth;
mod subcommand;

use clap::{command, Parser};
use crate::subcommand::Subcommand;

#[derive(clap::Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Subcommand
}

fn main() -> eyre::Result<()> {
    logger::init();

    let cli = Cli::parse();
    cli.command.run()?;

    Ok(())
}
