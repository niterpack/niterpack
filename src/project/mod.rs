use std::path::PathBuf;
use crate::error::Result;

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
        crate::format::format(path)
    }

    pub fn create_files(&self, path: PathBuf) -> Result<()> {
        crate::format::create_main_file(self, path.join("niter.json"))
    }
}

#[derive(Debug, Clone)]
pub struct Mod {
    pub file: String,
    pub download: String
}

impl Mod {
    pub fn new(
        file: String,
        download: String
    ) -> Self {
        Mod {
            file,
            download
        }
    }
}