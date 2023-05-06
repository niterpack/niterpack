use crate::ops;
use crate::Project;
use log::info;
use std::env;

#[derive(clap::Args)]
pub struct BuildArgs;

impl BuildArgs {
    pub fn run(&self) -> eyre::Result<()> {
        let current_dir = env::current_dir().unwrap();

        ops::build(&Project::read(&current_dir)?, current_dir.join("build"))?;

        info!("Finished building modpack");
        Ok(())
    }
}
