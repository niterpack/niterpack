mod add;
mod build;

#[derive(clap::Subcommand)]
pub enum Subcommand {
    /// Add a new mod to the current modpack
    Add(add::AddArgs),

    /// Build the current modpack
    Build(build::BuildArgs)
}

impl Subcommand {
    pub fn run(&self) -> eyre::Result<()> {
        match &self {
            Subcommand::Add(args) => args.run(),
            Subcommand::Build(args) => args.run()
        }
    }
}
