pub mod source;

use std::path::PathBuf;
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

#[derive(Debug, Clone)]
pub struct Mod {
    pub file: Option<String>,
    pub source: Source
}

impl Mod {
    pub fn new(
        file: Option<String>,
        source: Source
    ) -> Self {
        Mod {
            file,
            source
        }
    }
}