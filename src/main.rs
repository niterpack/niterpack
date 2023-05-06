extern crate core;

mod commands;
mod logger;
mod modrinth;
mod ops;
mod project;
mod source;
mod toml;

pub use project::*;
pub use source::Source;

use crate::commands::Commands;
use clap::{command, Parser};

#[derive(clap::Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() -> eyre::Result<()> {
    logger::init();

    let cli = Cli::parse();
    cli.command.run()?;

    Ok(())
}
