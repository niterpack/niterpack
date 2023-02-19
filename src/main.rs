mod parse;
mod build;
mod log;
mod project;

use std::env;
use clap::{command, Command};
use crate::build::build;
use crate::log::elog;
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
    let project = parse(current_dir.clone()).unwrap_or_else(|err| {
        elog(err.to_string());
        std::process::exit(1);
    });

    match matches.subcommand() {
        Some(("build", _)) => {
            log!("Building modpack {}, version {}", project.name, project.version);

            build(project, current_dir.join("build")).unwrap_or_else(|err| {
                elog(err.to_string());
                std::process::exit(1);
            });
        }
        _ => unreachable!()
    }
}
