use crate::modrinth;
use crate::modrinth::ModrinthProjectType;
use crate::toml::JoinToml;
use crate::Mod;
use crate::Source;
use eyre::{ensure, ContextCompat, WrapErr};
use log::info;
use std::env;

#[derive(clap::Args)]
pub struct AddArgs {
    /// Reference to a project to add as a mod
    ///
    /// You can reference a project using a slug, or an id.
    #[arg(id = "MOD")]
    mod_name: String,

    /// Reference to the project's version
    ///
    /// You can reference a version using a number, or an id.
    #[arg(id = "VERSION", short = 'v', long = "version")]
    version_name: Option<String>,
}

impl AddArgs {
    pub fn mod_data(&self) -> eyre::Result<Mod> {
        let path = env::current_dir().unwrap();

        let manifest = crate::toml::read_manifest(path.join_manifest_file())
            .wrap_err("failed to read manifest file")?;

        let project =
            modrinth::project(&self.mod_name).wrap_err("failed to fetch modrinth project")?;

        ensure!(
            project.project_type == ModrinthProjectType::Mod,
            "only `mod` project types are allowed"
        );

        let version = match &self.version_name {
            Some(version_name) => version_name.into(),
            None => {
                modrinth::project_versions(
                    &project.id,
                    manifest.loader.as_deref(),
                    manifest.minecraft_version.as_deref(),
                )
                .wrap_err("failed to fetch project versions")?
                .first()
                .wrap_err("project doesn't have a valid version for this modpack")?
                .clone()
                .id
            }
        };

        Ok(Mod::new(project.slug, None, Source::Modrinth { version }))
    }

    pub fn run(&self) -> eyre::Result<()> {
        let mod_data = self.mod_data()?;

        crate::toml::write_mod(
            env::current_dir()
                .unwrap()
                .join_mods_dir()
                .join_mod_file(&mod_data.name),
            mod_data.clone(),
        )?;

        info!("Added mod `{}` to modpack", mod_data.name);
        Ok(())
    }
}
