mod format;
mod build;
mod log;
mod project;
mod error;

use std::env;
use clap::{arg, command, Command};
use crate::build::build;
use crate::format::ProjectFormatting;
use crate::log::UnwrapOrLogExt;
use crate::project::{Mod, Project};
use crate::project::source::Source;

fn cli() -> Command {
    command!()
        .subcommand_required(true)
        .subcommand(
            Command::new("build")
                .about("Builds the project")
        )
        .subcommand(
            Command::new("add")
                .about("Adds a new mod")
                .arg(arg!(<NAME>  "Name of the new mod"))
                .arg(arg!(<LINK>  "Download link to the new mod"))
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
            let project = Project::format(current_dir.clone()).unwrap_or_log();

            log!("Building modpack {}, version {}", project.name, project.version);

            build(project, current_dir.join("build")).unwrap_or_log();
        },
        Some(("add", sub_matches)) => {
            let formatting = ProjectFormatting::new(current_dir.clone());

            let name = sub_matches.get_one::<String>("NAME").unwrap();
            let download = sub_matches.get_one::<String>("LINK").unwrap();

            let mod_data = Mod::new(
                name.clone(),
                None,
                Source::Download {
                    url: download.clone()
                }
            );

            formatting.create_mod(
                name,
                &mod_data
            ).unwrap_or_log();

            log!("Added mod '{}'", name)
        },
        Some(("init", _)) => {
            let project = Project::new(
                current_dir.file_name().unwrap().to_os_string().into_string().unwrap(),
                "0.1.0".into()
            );

            project.create(current_dir).unwrap_or_log();

            log!("Created modpack '{}'", project.name)
        },
        _ => unreachable!()
    }
}
