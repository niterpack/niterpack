mod format;
mod build;
mod log;
mod project;
mod error;

use std::env;
use clap::{arg, command, Command};
use reqwest::Url;
use crate::build::build;
use crate::log::UnwrapOrLogExt;
use crate::project::{Mod, Project};

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
            let download_url = &Url::parse(sub_matches.get_one::<String>("LINK").unwrap()).unwrap_or_log();
            let file_name = download_url
                .path_segments()
                .and_then(|segments| segments.last())
                .and_then(|name| if name.is_empty() { None } else { Some(name) })
                .unwrap();

            let mod_data = Mod::new(
                file_name.into(),
                download_url.clone().into()
            );

            format::create_mod_file(
                &mod_data,
                current_dir
                    .join("mods")
                    .join(file_name)
                    .with_extension("json")
            ).unwrap_or_log();

            log!("Added mod '{}'", file_name)
        },
        Some(("init", _)) => {
            let project = Project::new(
                current_dir.file_name().unwrap().to_string_lossy().to_string(),
                "0.1.0".into()
            );

            project.create_files(current_dir).unwrap_or_log();

            log!("Created modpack '{}'", project.name)
        },
        _ => unreachable!()
    }
}
