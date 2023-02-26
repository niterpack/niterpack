use std::fs;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use crate::error::{Result, ValueExpected, AlreadyInitiated, MainFileNotFound, ModAlreadyExists, UnsupportedFormat};
use crate::project::{Project, Mod};

const SUPPORTED_FORMAT: &str = "0beta";

#[derive(Serialize, Deserialize)]
pub struct MainFile {
    format: String,
    name: String,
    version: String
}

#[derive(Debug, Default)]
pub struct ProjectFormatting {
    path: PathBuf
}

impl ProjectFormatting {
    pub fn new(path: PathBuf) -> ProjectFormatting {
        ProjectFormatting {
            path
        }
    }

    pub fn main_file_path(&self) -> PathBuf {
        self.path.join("niter.json")
    }

    pub fn mods_path(&self) -> PathBuf {
        self.path.join("mods")
    }

    pub fn mod_path(&self, name: &str) -> PathBuf {
        self.mods_path().join(name).with_extension("json")
    }

    fn create_mods_dir(&self) -> Result<()> {
        let path = self.mods_path();
        if !path.exists() {
            fs::create_dir(self.mods_path())?;
        }
        Ok(())
    }

    pub fn format_mod(&self, name: &str) -> Result<Mod> {
        let path = self.mod_path(name);

        let mut mod_data = serde_json::from_str::<Mod>(fs::read_to_string(&path)?.as_str())?;
        mod_data.name = name.into();
        Ok(mod_data)
    }

    pub fn format_main_file(&self) -> Result<MainFile> {
        let path = self.main_file_path();

        if !path.exists() || !path.is_file() {
            return Err(MainFileNotFound.into());
        }

        let main_file: Value = serde_json::from_str(fs::read_to_string(&path)?.as_str())?;

        let format = main_file["format"]
            .as_str()
            .ok_or(ValueExpected::from_path("format".into(), &path))?;

        if format != SUPPORTED_FORMAT {
            return Err(UnsupportedFormat(format.into()).into())
        }

        Ok(serde_json::from_value(main_file)?)
    }

    pub fn create_mod(&self, name: &str, mod_data: &Mod) -> Result<()> {
        self.create_mods_dir()?;

        let path = self.mod_path(name);

        if path.exists() {
            return Err(ModAlreadyExists(name.into()).into());
        }

        serde_json::to_writer_pretty(
            fs::File::create(path)?,
            mod_data
        )?;

        Ok(())
    }

    fn create_main_file(&self, project: &Project) -> Result<()> {
        let path = self.main_file_path();

        if path.exists() {
            return Err(AlreadyInitiated.into());
        }

        let main_file = MainFile {
            format: SUPPORTED_FORMAT.into(),
            name: project.name.clone(),
            version: project.version.clone()
        };

        serde_json::to_writer_pretty(fs::File::create(path)?, &main_file)
            .map_err(|err| err.into())
    }

    pub fn remove_mod(&self, name: &str) -> Result<()> {
        let path = self.mod_path(name);

        fs::remove_file(path)?;

        Ok(())
    }
}

pub fn format_project(path: PathBuf) -> Result<Project> {
    let formatting = ProjectFormatting::new(path);

    let main_file = formatting.format_main_file()?;
    let mods_path = formatting.mods_path();
    let mut mods: Vec<Mod> = vec![];

    if mods_path.exists() {
        for entry in fs::read_dir(formatting.mods_path())? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() || path.extension() != Some("json".as_ref()) {
                continue;
            }

            mods.push(formatting.format_mod(path.file_name().unwrap().to_str().unwrap())?);
        }
    }

    Ok(Project {
        name: main_file.name,
        version: main_file.version,
        mods
    })
}

pub fn create_project(project: &Project, path: PathBuf) -> Result<()> {
    let formatting = ProjectFormatting::new(path);

    formatting.create_main_file(project)?;

    for mod_data in &project.mods {
        formatting.create_mod(mod_data.name.as_str(), mod_data)?;
    }

    Ok(())
}
