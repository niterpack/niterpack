mod add;
mod build;

use crate::error::Result;

#[derive(clap::Subcommand)]
pub enum Subcommand {
    /// Add a new mod to the current modpack
    Add(add::AddArgs),

    /// Build the current modpack
    Build(build::BuildArgs)
}

impl Subcommand {
    pub fn run(&self) -> Result<()> {
        match &self {
            Subcommand::Add(args) => args.run(),
            Subcommand::Build(args) => args.run()
        }
    }
}
