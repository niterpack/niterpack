pub mod mainfile;
pub mod modfile;

use crate::format::mainfile::MainFile;
use crate::format::modfile::ModFile;
use crate::{Mod, Project};
use eyre::{Result, WrapErr};
use std::fs;
use std::path::PathBuf;

#[derive(Debug)]
pub struct ProjectFormatter {
    main_file: MainFile,
    path: PathBuf,
}

impl ProjectFormatter {
    pub fn format(path: PathBuf) -> Result<ProjectFormatter> {
        Ok(ProjectFormatter {
            main_file: MainFile::from_file(MainFile::in_path(&path))
                .wrap_err("failed to format main file")?,
            path,
        })
    }

    pub fn create(path: PathBuf, project: &Project) -> Result<ProjectFormatter> {
        let main_file = MainFile::from(project.clone());
        main_file
            .to_file(MainFile::in_path(&path))
            .wrap_err("failed to create main file")?;
        Ok(ProjectFormatter { main_file, path })
    }

    pub fn mods_path(&self) -> PathBuf {
        self.path.join("mods")
    }

    pub fn mods(&self) -> Result<Vec<String>> {
        let mut mods: Vec<String> = Vec::new();
        let mods_path = self.mods_path();

        if mods_path.exists() {
            for entry in fs::read_dir(&mods_path)? {
                let entry = entry?;
                let path = entry.path();

                if path.is_dir() || path.extension() != Some("toml".as_ref()) {
                    continue;
                }

                mods.push(
                    path.with_extension("")
                        .file_name()
                        .unwrap()
                        .to_os_string()
                        .into_string()
                        .unwrap(),
                );
            }
        }

        Ok(mods)
    }

    fn create_mods_dir(&self) -> Result<()> {
        let path = self.mods_path();
        if !path.exists() {
            fs::create_dir(self.mods_path())?;
        }
        Ok(())
    }

    pub fn format_mod(&self, name: &str) -> Result<Mod> {
        ModFile::from_file(ModFile::in_path(self.mods_path(), name))
            .map(|file| file.to_mod(|| name.to_string()))
    }

    pub fn create_mod(&self, mod_data: &Mod) -> Result<()> {
        self.create_mods_dir()
            .wrap_err("failed to create mods directory")?;

        ModFile::from(mod_data.clone()).to_file(ModFile::in_path(self.mods_path(), &mod_data.name))
    }
}

pub fn create_all(project: &Project, path: PathBuf) -> Result<()> {
    let formatter = ProjectFormatter::create(path, project)?;

    for mod_data in &project.mods {
        formatter
            .create_mod(mod_data)
            .wrap_err(format!("failed to create mod `{}`", &mod_data.name))?;
    }

    Ok(())
}

pub fn format_all(path: PathBuf) -> Result<Project> {
    let formatter = ProjectFormatter::format(path)?;

    let mut mods: Vec<Mod> = Vec::new();
    for mod_name in formatter.mods().wrap_err("failed to list mods")? {
        mods.push(
            formatter
                .format_mod(&mod_name)
                .wrap_err(format!("failed to format mod `{}`", &mod_name))?,
        )
    }

    Ok(Project {
        name: formatter.main_file.modpack.name,
        version: formatter.main_file.modpack.version,
        mods,
    })
}
