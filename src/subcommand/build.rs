use std::env;
use log::info;
use crate::build;
use crate::error::Result;
use crate::project::Project;

#[derive(clap::Args)]
pub struct BuildArgs;

impl BuildArgs {
    pub fn run(&self) -> Result<()> {
        let current_dir = env::current_dir().unwrap();

        build::build(
            &Project::format(current_dir.clone())?,
            current_dir.join("build")
        )?;

        info!("Finished building modpack");
        Ok(())
    }
}
