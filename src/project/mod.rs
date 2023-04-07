pub mod source;

use crate::project::source::Source;
use eyre::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Project {
    pub name: String,
    pub version: String,
    pub mods: Vec<Mod>,
}

impl Project {
    pub fn new(name: String, version: String) -> Self {
        Project {
            name,
            version,
            mods: vec![],
        }
    }

    pub fn format(path: PathBuf) -> Result<Self> {
        Ok(crate::format::format_project(path)?)
    }

    pub fn create(&self, path: PathBuf) -> Result<()> {
        Ok(crate::format::create_project(self, path)?)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mod {
    #[serde(skip)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    pub source: Source,
}

impl Mod {
    pub fn new(name: String, file: Option<String>, source: Source) -> Self {
        Mod { name, file, source }
    }

    pub fn file_or_source(&self) -> Result<String> {
        match self.file.clone() {
            Some(file) => Ok(file),
            None => self.source.file_name(),
        }
    }
}
