mod format;
mod build;
mod logger;
mod project;
mod error;
mod modrinth;

use std::env;
use clap::{arg, command, Command};
use log::info;
use crate::build::build;
use crate::format::ProjectFormatter;
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
                .arg(arg!(<SOURCE>      "Source of the new mod"))
                .arg(arg!(--name <NAME> "Name of the new mod")
                    .required(true))
        )
        .subcommand(
            Command::new("remove")
                .about("Removes an existing mod")
                .arg(arg!(<NAME>  "Name of the mod")))
        .subcommand(
            Command::new("init")
                .about("Creates a new Niter project in the current directory")
        )
}

fn main() {
    logger::init();

    let current_dir = env::current_dir().unwrap();
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("build", _)) => {
            let project = Project::format(current_dir.clone()).unwrap();

            info!("Building modpack {}, version {}", project.name, project.version);

            build(&project, current_dir.join("build")).unwrap();
        },
        Some(("add", sub_matches)) => {
            let formatter = ProjectFormatter::format(current_dir.clone()).unwrap();

            let name = sub_matches.get_one::<String>("name").unwrap();
            let source = sub_matches.get_one::<String>("SOURCE").unwrap();

            let mod_data = Mod::new(
                name.clone(),
                None,
                Source::Modrinth {
                    version_id: source.clone()
                }
            );

            formatter.create_mod(
                name,
                &mod_data
            ).unwrap();

            info!("Added mod '{}'", name)
        },
        Some(("remove", sub_matches)) => {
            let formatter = ProjectFormatter::format(current_dir.clone()).unwrap();

            let name = sub_matches.get_one::<String>("NAME").unwrap();

            formatter.remove_mod(name).unwrap();

            info!("Removed mod '{}'", name)
        },
        Some(("init", _)) => {
            let project = Project::new(
                current_dir.file_name().unwrap().to_os_string().into_string().unwrap(),
                "0.1.0".into()
            );

            project.create(current_dir).unwrap();

            info!("Created modpack '{}'", project.name)
        },
        _ => unreachable!()
    }
}
