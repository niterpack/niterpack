pub mod source;

use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::error::Result;
use crate::project::source::Source;

#[derive(Debug, Clone)]
pub struct Project {
    pub name: String,
    pub version: String,
    pub mods: Vec<Mod>
}

impl Project {
    pub fn new(
        name: String,
        version: String
    ) -> Self {
        Project {
            name,
            version,
            mods: vec![]
        }
    }

    pub fn format(path: PathBuf) -> Result<Self> {
        crate::format::format_project(path)
    }

    pub fn create(&self, path: PathBuf) -> Result<()> {
        crate::format::create_project(self, path)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mod {
    #[serde(skip)]
    pub name: String,
    pub file: Option<String>,
    pub source: Source
}

impl Mod {
    pub fn new(
        name: String,
        file: Option<String>,
        source: Source
    ) -> Self {
        Mod {
            name,
            file,
            source
        }
    }

    pub fn file_or_source(&self) -> Result<String> {
       Ok(self.file.clone().unwrap_or(self.source.file_name()?))
    }
}