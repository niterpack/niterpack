use crate::project::Project;
use eyre::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

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

    pub fn in_path<P: AsRef<Path>>(path: P) -> PathBuf {
        path.as_ref().join("niter.toml")
    }

    pub fn from_str(str: &str) -> Result<MainFile, toml::de::Error> {
        toml::from_str(str)
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<MainFile> {
        Ok(Self::from_str(&fs::read_to_string(path)?)?)
    }

    pub fn to_string(&self) -> Result<String, toml::ser::Error> {
        toml::to_string(self)
    }

    pub fn to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
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
