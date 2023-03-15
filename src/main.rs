extern crate core;

mod format;
mod build;
mod logger;
mod project;
mod error;
mod modrinth;
mod subcommand;

use clap::{command, Parser};
use log::error;
use crate::subcommand::Subcommand;

#[derive(clap::Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Subcommand
}

fn main() {
    logger::init();

    let cli = Cli::parse();

    if let Err(err) = cli.command.run() {
        error!("{}", err);
    }
}
