use crate::source::BuildSource;
use crate::{Manifest, Source};
use eyre::Result;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct Mod {
    pub name: String,
    pub file: Option<String>,
    pub source: Source,
}

#[derive(Debug, Clone)]
pub struct Project {
    pub manifest: Manifest,
    pub mods: Vec<Mod>,
    pub config_dir: Option<PathBuf>,
}

impl Mod {
    pub fn new(name: String, file: Option<String>, source: Source) -> Self {
        Mod { name, file, source }
    }

    pub fn build_source(&self, manifest: &Manifest) -> Result<BuildSource> {
        BuildSource::generate(manifest, self)
    }
}

impl Project {
    pub fn new(manifest: Manifest, mods: Vec<Mod>, config_dir: Option<PathBuf>) -> Self {
        Self {
            manifest,
            mods,
            config_dir,
        }
    }

    pub fn read<P: AsRef<Path>>(path: P) -> Result<Self> {
        crate::toml::read_project(path)
    }

    pub fn write<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        crate::toml::write_project(path, self.clone())
    }

    pub fn build_sources(&self) -> Result<Vec<BuildSource>> {
        let mut result = Vec::new();
        for mod_data in &self.mods {
            result.push(mod_data.build_source(&self.manifest)?);
        }
        Ok(result)
    }
}

impl From<Manifest> for Project {
    fn from(value: Manifest) -> Self {
        Project {
            manifest: value,
            mods: vec![],
            config_dir: None,
        }
    }
}
