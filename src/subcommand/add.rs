use crate::format::ProjectFormatter;
use crate::modrinth;
use crate::modrinth::ModrinthProjectType;
use crate::project::source::Source;
use crate::project::Mod;
use eyre::{ensure, eyre, ContextCompat, WrapErr};
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
        let project = modrinth::get_project(&self.mod_name)
            .wrap_err("failed to fetch modrinth project")?
            .ok_or_else(|| eyre!("project `{}` not found", &self.mod_name))?;

        ensure!(
            project.project_type == ModrinthProjectType::Mod,
            "only `mod` project types are allowed"
        );

        let version_id = match &self.version_name {
            Some(version_name) => match modrinth::get_version(version_name)
                .wrap_err("failed to fetch modrinth version")?
            {
                Some(version) => version.id,
                None => {
                    modrinth::get_versions(&self.mod_name)
                        .wrap_err("failed to fetch modrinth project versions")?
                        .into_iter()
                        .find(|version| &version.version_number == version_name)
                        .ok_or_else(|| eyre!("could not find version `{}`", version_name))?
                        .clone()
                        .id
                }
            },
            None => project
                .versions
                .last()
                .wrap_err("project doesn't have any versions")?
                .clone(),
        };

        Ok(Mod::new(
            project.slug,
            None,
            Source::Modrinth { version_id },
        ))
    }

    pub fn run(&self) -> eyre::Result<()> {
        let mod_data = self.mod_data()?;

        let formatter = ProjectFormatter::format(env::current_dir().unwrap())?;

        formatter.create_mod(&mod_data)?;

        info!("Added mod `{}` to modpack", mod_data.name);
        Ok(())
    }
}
