mod parse;
mod build;
mod log;
mod project;

use std::env;
use clap::{command, Command};
use crate::build::build;
use crate::log::elog;
use crate::project::Project;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

fn cli() -> Command {
    command!()
        .subcommand_required(true)
        .subcommand(
            Command::new("build")
                .about("Builds the project")
        )
        .subcommand(
            Command::new("init")
                .about("Creates a new Niter project in the current directory")
        )
}

fn main() {
    let current_dir = env::current_dir().unwrap();
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("build", _)) => {
            let project = Project::parse(current_dir.clone()).unwrap_or_else(|err| {
                elog(err.to_string());
                std::process::exit(1);
            });

            log!("Building modpack {}, version {}", project.name, project.version);

            build(project, current_dir.join("build")).unwrap_or_else(|err| {
                elog(err.to_string());
                std::process::exit(1);
            });
        },
        Some(("init", _)) => {
            let project = Project::new(
                current_dir.file_name().unwrap().to_string_lossy().to_string(),
                "0.1.0".into()
            );

            project.create_files(current_dir)
                .unwrap_or_else(|err| {
                    elog(err.to_string());
                    std::process::exit(1);
                });

            log!("Created modpack '{}'", project.name)
        }
        _ => unreachable!()
    }
}
