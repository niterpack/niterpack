use crate::project::Project;
use eyre::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

pub const MAIN_FILE_NAME: &str = "niter.toml";

#[derive(Debug, Serialize, Deserialize)]
pub struct MainFile {
    pub modpack: Modpack,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Modpack {
    pub name: String,
    pub version: String,
}

impl MainFile {
    pub fn new(modpack: Modpack) -> MainFile {
        MainFile { modpack }
    }

    pub fn get_path(path: &Path) -> PathBuf {
        path.join(MAIN_FILE_NAME)
    }

    pub fn from_str(str: &str) -> Result<MainFile, toml::de::Error> {
        toml::from_str(str)
    }

    pub fn format(path: &Path) -> Result<MainFile> {
        Ok(Self::from_str(&fs::read_to_string(path)?)?)
    }

    pub fn to_string(&self) -> Result<String, toml::ser::Error> {
        toml::to_string(self)
    }

    pub fn create(&self, path: &Path) -> Result<()> {
        fs::write(path, self.to_string()?)?;
        Ok(())
    }
}

impl From<Project> for MainFile {
    fn from(value: Project) -> Self {
        MainFile::new(Modpack::new(value.name, value.version))
    }
}

impl Modpack {
    pub fn new(name: String, version: String) -> Modpack {
        Modpack { name, version }
    }
}
