mod add;
mod build;
mod init;
mod remove;

#[derive(clap::Subcommand)]
pub enum Commands {
    /// Create a new modpack in the current directory
    Init(init::InitArgs),

    /// Add a new mod to the current modpack
    Add(add::AddArgs),

    /// Remove a mod from the current modpack
    Remove(remove::RemoveArgs),

    /// Build the current modpack
    Build(build::BuildArgs),
}

impl Commands {
    pub fn run(&self) -> eyre::Result<()> {
        match &self {
            Commands::Init(args) => args.run(),
            Commands::Add(args) => args.run(),
            Commands::Remove(args) => args.run(),
            Commands::Build(args) => args.run(),
        }
    }
}
