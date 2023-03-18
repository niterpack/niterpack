mod ext;
mod error;

use std::fs;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use serde::de::Unexpected;
use serde_json::Value;
use crate::project::{Project, Mod};
use crate::format::ext::FromValueExt;
use crate::format::error::FormatError;

const SUPPORTED_FORMAT: &str = "0beta";

#[derive(Debug, Serialize, Deserialize)]
struct MainFile {
    format: String,
    name: String,
    version: String
}

#[derive(Debug)]
pub struct ProjectFormatter {
    path: PathBuf,
    main_file: MainFile
}


impl ProjectFormatter {

    pub fn format(path: PathBuf) -> Result<ProjectFormatter, FormatError> {
        let main_file = format_main_file(path.join("niter.json"))?;
        Ok(ProjectFormatter {
            path,
            main_file
        })
    }

    pub fn create(path: PathBuf, project: &Project) -> Result<ProjectFormatter, FormatError> {
        let main_file: MainFile = project.into();
        create_main_file(path.join("niter.json"), &main_file)?;

        Ok(ProjectFormatter {
            path,
            main_file
        })
    }

    pub fn mods_path(&self) -> PathBuf {
        self.path.join("mods")
    }

    pub fn mod_path(&self, name: &str) -> PathBuf {
        self.mods_path().join(name).with_extension("json")
    }

    pub fn mods(&self) -> Result<Vec<String>, FormatError> {
        let mut mods: Vec<String> = Vec::new();
        let mods_path = self.mods_path();

        if mods_path.exists() {
            for entry in fs::read_dir(&mods_path)? {
                let entry = entry?;
                let path = entry.path();

                if path.is_dir() || path.extension() != Some("json".as_ref()) {
                    continue;
                }

                mods.push(path.with_extension("").file_name().unwrap().to_os_string().into_string().unwrap());
            }
        }

        Ok(mods)
    }

    fn create_mods_dir(&self) -> Result<(), FormatError> {
        let path = self.mods_path();
        if !path.exists() {
            fs::create_dir(self.mods_path())?;
        }
        Ok(())
    }

    pub fn format_mod(&self, name: &str) -> Result<Mod, FormatError> {
        let path = self.mod_path(name);

        let mut mod_data = serde_json::from_str::<Mod>(
            fs::read_to_string(&path)?
                .as_str()
        )?;

        mod_data.name = name.into();
        Ok(mod_data)
    }


    pub fn create_mod(&self, mod_data: &Mod) -> Result<(), FormatError> {
        self.create_mods_dir()?;

        let path = self.mod_path(&mod_data.name);
        Ok(serde_json::to_writer_pretty(
            fs::File::create(&path)?,
            mod_data
        )?)
    }

    pub fn remove_mod(&self, name: &str) -> Result<(), FormatError> {
        let path = self.mod_path(name);
        fs::remove_file(&path)?;
        Ok(())
    }
}


impl From<&Project> for MainFile {
    fn from(value: &Project) -> Self {
        MainFile {
            format: SUPPORTED_FORMAT.into(),
            name: value.name.clone(),
            version: value.version.clone()
        }
    }
}


fn format_main_file(path: PathBuf) -> Result<MainFile, FormatError> {
    if !path.exists() {
        return Err(FormatError::MainFileNotFound);
    }

    let main_file: Value = serde_json::from_str(
        fs::read_to_string(&path)?
            .as_str()
    )?;

    let format = main_file
        .get("format")
        .ok_or_else(|| FormatError::Serialization(serde::de::Error::missing_field("format")))?;
    let format = format
        .as_str()
        .ok_or_else(|| FormatError::Serialization(serde::de::Error::invalid_type(Unexpected::from_value(format), &"a string")))?;

    if format != SUPPORTED_FORMAT {
        return Err(FormatError::UnsupportedFormat(format.into()));
    }

    Ok(serde_json::from_value(main_file)?)
}

fn create_main_file(path: PathBuf, main_file: &MainFile) -> Result<(), FormatError> {
    if path.exists() {
        return Err(FormatError::AlreadyInitialized);
    }

    Ok(serde_json::to_writer_pretty(
        fs::File::create(&path)?,
        main_file
    )?)
}

pub fn create_project(project: &Project, path: PathBuf) -> Result<(), FormatError> {
    let formatter = ProjectFormatter::create(path, project)?;

    for mod_data in &project.mods {
        formatter.create_mod(mod_data)?;
    }

    Ok(())
}

pub fn format_project(path: PathBuf) -> Result<Project, FormatError> {
    let formatter = ProjectFormatter::format(path)?;

    let mut mods: Vec<Mod> = Vec::new();
    for mod_name in formatter.mods()? {
        mods.push(formatter.format_mod(&mod_name)?)
    }

    Ok(Project {
        name: formatter.main_file.name,
        version: formatter.main_file.version,
        mods
    })
}
