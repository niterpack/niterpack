use crate::toml::JoinToml;
use console::style;
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
        let current_dir = env::current_dir().unwrap();

        ensure!(
            current_dir.join_manifest_file().exists(),
            "could not find `niterpack.toml` in the current directory"
        );

        let mod_path = current_dir.join_mods_dir().join_mod_file(&self.mod_name);

        ensure!(
            mod_path.exists(),
            "mod `{}` doesn't exist in this modpack",
            &self.mod_name
        );

        fs::remove_file(mod_path)?;

        info!(
            "{} {} from modpack",
            style("Removed").green().bold(),
            &self.mod_name
        );
        Ok(())
    }
}
