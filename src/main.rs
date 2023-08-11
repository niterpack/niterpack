extern crate core;

mod commands;
mod logger;
mod manifest;
mod ops;
mod project;
mod source;
mod toml;
mod util;

pub use manifest::*;
pub use project::*;
pub use source::Source;

use crate::commands::Commands;
use clap::{command, Parser};
use log::error;

#[derive(clap::Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() {
    logger::install();

    let cli = Cli::parse();
    let result = cli.command.run();

    if let Err(err) = result {
        error!("{}", err);
        std::process::exit(1);
    }
}
