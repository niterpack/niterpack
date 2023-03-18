use std::env;
use eyre::{eyre, WrapErr};
use log::info;
use url::Url;
use crate::format::ProjectFormatter;
use crate::modrinth;
use crate::project::Mod;
use crate::project::source::Source;

#[derive(clap::Args)]
pub struct AddArgs {
    /// Source of the new mod, either url or modrinth version id
    source: String,

    /// Name of the new mod
    #[arg(short, long)]
    name: String
}

impl AddArgs {
    pub fn parse_source(&self) -> eyre::Result<Source> {
        if let Some(_) = modrinth::get_version(&self.source).wrap_err("failed to get modrinth version")? {
            Ok(Source::Modrinth { version_id: self.source.clone() })
        } else {
            Url::parse(&self.source).map_err(|_| eyre!("invalid source url or modrinth version id"))?;
            Ok(Source::Download { url: self.source.clone() })
        }
    }

    pub fn mod_data(&self) -> eyre::Result<Mod> {
        Ok(Mod::new(self.name.clone(), None, self.parse_source()?))
    }

    pub fn run(&self) -> eyre::Result<()> {
        let mod_data = self.mod_data()?;

        let formatter = ProjectFormatter::format(
            env::current_dir().unwrap()
        )?;

        formatter.create_mod(&mod_data)?;

        info!("Added mod `{}` to modpack", self.name);
        Ok(())
    }
}