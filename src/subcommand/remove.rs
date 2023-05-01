use crate::toml::JoinToml;
use eyre::ensure;
use log::info;
use std::{env, fs};

#[derive(clap::Args)]
pub struct RemoveArgs {
    /// Name of the mod to remove
    #[arg(id = "MOD")]
    mod_name: String,
}

impl RemoveArgs {
    pub fn run(&self) -> eyre::Result<()> {
        let path = env::current_dir()
            .unwrap()
            .join_mods_dir()
            .join_mod_file(&self.mod_name);

        ensure!(
            path.exists(),
            "mod `{}` doesn't exist in this modpack",
            &self.mod_name
        );

        fs::remove_file(path)?;

        info!("Removed mod `{}` from modpack", &self.mod_name);
        Ok(())
    }
}
