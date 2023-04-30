mod add;
mod build;
mod init;
mod remove;

#[derive(clap::Subcommand)]
pub enum Subcommand {
    /// Create a new modpack in the current directory
    Init(init::InitArgs),

    /// Add a new mod to the current modpack
    Add(add::AddArgs),

    /// Remove a mod from the current modpack
    Remove(remove::RemoveArgs),

    /// Build the current modpack
    Build(build::BuildArgs),
}

impl Subcommand {
    pub fn run(&self) -> eyre::Result<()> {
        match &self {
            Subcommand::Init(args) => args.run(),
            Subcommand::Add(args) => args.run(),
            Subcommand::Remove(args) => args.run(),
            Subcommand::Build(args) => args.run(),
        }
    }
}
