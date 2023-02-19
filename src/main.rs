mod parse;
mod modpack;
mod build;

use std::env;
use std::path::PathBuf;
use clap::{arg, command, Command};
use crate::parse::parse;

fn cli() -> Command {
    command!()
        .arg(arg!(<PATH> "Path to the folder of parsing").value_parser(clap::value_parser!(PathBuf)))
}

fn main() {
    let matches = cli().get_matches();
    println!(
        "{:?}",
        parse(
            matches.get_one::<PathBuf>("PATH")
                .unwrap_or(&env::current_dir().unwrap())
                .clone()
        )
    );
}
