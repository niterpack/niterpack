use crate::project::source::Source;
use crate::project::Mod;
use eyre::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct ModFile {
    pub name: Option<String>,
    pub file: Option<String>,
    #[serde(flatten)]
    pub source: Source,
}

impl ModFile {
    pub fn to_mod<F>(&self, f: F) -> Mod
    where
        F: FnOnce() -> String,
    {
        Mod {
            name: self.name.clone().unwrap_or_else(f),
            file: self.file.clone(),
            source: self.source.clone(),
        }
    }

    pub fn in_path<P: AsRef<Path>>(path: P, mod_name: &str) -> PathBuf {
        path.as_ref().join(mod_name).with_extension("toml")
    }

    pub fn from_str(str: &str) -> Result<ModFile, toml::de::Error> {
        toml::from_str(str)
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<ModFile> {
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

impl From<Mod> for ModFile {
    fn from(value: Mod) -> Self {
        ModFile {
            name: Some(value.name),
            source: value.source,
            file: value.file,
        }
    }
}
