use crate::{Manifest, Project};
use eyre::ContextCompat;
use log::info;
use std::env;

#[derive(clap::Args)]
pub struct InitArgs;

impl InitArgs {
    pub fn run(&self) -> eyre::Result<()> {
        let current_dir = env::current_dir().unwrap();

        let project = Project::new(Manifest::new(
            current_dir
                .file_name()
                .and_then(|name| name.to_os_string().into_string().ok())
                .wrap_err("failed to get file name of the current directory")?,
            String::from("0.1.0"),
        ));

        project.write(current_dir)?;

        info!("Created a new modpack `{}`", &project.manifest.name);
        Ok(())
    }
}
