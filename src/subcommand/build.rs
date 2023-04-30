use crate::build;
use crate::Project;
use log::info;
use std::env;

#[derive(clap::Args)]
pub struct BuildArgs;

impl BuildArgs {
    pub fn run(&self) -> eyre::Result<()> {
        let current_dir = env::current_dir().unwrap();

        build::build(
            &Project::format(current_dir.clone())?,
            current_dir.join("build"),
        )?;

        info!("Finished building modpack");
        Ok(())
    }
}
