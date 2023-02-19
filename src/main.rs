mod parse;
mod modpack;
mod build;

use std::env;
use clap::{command, Command};
use crate::build::build;
use crate::parse::parse;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

fn cli() -> Command {
    command!()
        .subcommand_required(true)
        .subcommand(
            Command::new("build")
                .about("Builds the project")
        )
}

fn main() {
    let current_dir = env::current_dir().unwrap();
    let matches = cli().get_matches();
    let modpack = parse(current_dir.clone()).unwrap_or_else(|err| {
        eprintln!("error: {}", err);
        std::process::exit(1);
    });

    match matches.subcommand() {
        Some(("build", _)) => {
            println!("Building modpack {}, version {}", modpack.name, modpack.version);

            build(modpack, current_dir.join("build")).unwrap_or_else(|err| {
                eprintln!("error: {}", err);
                std::process::exit(1);
            });
        }
        _ => unreachable!()
    }
}
