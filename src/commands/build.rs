use crate::ops;
use crate::Project;
use clap::ValueEnum;
use log::info;
use std::env;
use std::path::Path;

#[derive(clap::Args)]
pub struct BuildArgs {
    output: Output,
}

#[derive(Debug, Clone, ValueEnum)]
enum Output {
    Instance,
    Modrinth,
}

impl BuildArgs {
    pub fn run(&self) -> eyre::Result<()> {
        let current_dir = env::current_dir().unwrap();
        let project = Project::read(&current_dir)?;

        self.output.build(current_dir.join("build"), &project)?;

        info!("Finished building modpack");
        Ok(())
    }
}

impl Output {
    pub fn build<P: AsRef<Path>>(&self, path: P, project: &Project) -> eyre::Result<()> {
        let sources = project.build_sources()?;

        match self {
            Output::Instance => ops::build_instance(sources, path.as_ref().join("instance")),
            Output::Modrinth => {
                ops::build_modrinth(&project.manifest, sources, path.as_ref().join("modrinth"))
            }
        }
    }
}
