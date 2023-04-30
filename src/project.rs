use crate::Source;
use crate::Source::{Download, Modrinth};
use eyre::{eyre, Result, WrapErr};
use std::path::PathBuf;
use url::Url;

#[derive(Debug, Clone)]
pub struct Manifest {
    pub name: String,
    pub version: String,
}

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
}

impl Manifest {
    pub fn new(name: String, version: String) -> Self {
        Self { name, version }
    }
}

impl From<Project> for Manifest {
    fn from(value: Project) -> Self {
        value.manifest
    }
}

impl Mod {
    pub fn new(name: String, file: Option<String>, source: Source) -> Self {
        Mod { name, file, source }
    }

    pub fn download_url(&self) -> Result<String> {
        match &self.source {
            Download { url } => Ok(url.clone()),
            Modrinth { version } => Ok(match crate::modrinth::get_version(version)
                .wrap_err("failed to fetch modrinth version")?
            {
                Some(version) => version,
                None => crate::modrinth::get_versions(&self.name)
                    .wrap_err("failed to fetch modrinth project versions")?
                    .into_iter()
                    .find(|modrinth_version| &modrinth_version.version_number == version)
                    .ok_or_else(|| eyre!("could not find version `{}`", version))?,
            }
            .primary_file()
            .ok_or_else(|| eyre!("primary file not found"))?
            .url
            .clone()),
        }
    }

    pub fn file_name(&self) -> Result<String> {
        match self.file.clone() {
            Some(file) => Ok(file),
            None => match &self.source {
                Download { url } => Url::parse(url)?
                    .path_segments()
                    .and_then(|segments| segments.last())
                    .and_then(|name| if name.is_empty() { None } else { Some(name) })
                    .map(|s| s.into())
                    .ok_or_else(|| eyre!("invalid url")),
                Modrinth { version } => Ok(match crate::modrinth::get_version(version)
                    .wrap_err("failed to fetch modrinth version")?
                {
                    Some(version) => version,
                    None => crate::modrinth::get_versions(&self.name)
                        .wrap_err("failed to fetch modrinth project versions")?
                        .into_iter()
                        .find(|modrinth_version| &modrinth_version.version_number == version)
                        .ok_or_else(|| eyre!("could not find version `{}`", version))?,
                }
                .primary_file()
                .ok_or_else(|| eyre!("primary file not found"))?
                .filename
                .clone()),
            },
        }
    }
}

impl Project {
    pub fn new(manifest: Manifest) -> Self {
        Self::with_mods(manifest, vec![])
    }

    pub fn with_mods(manifest: Manifest, mods: Vec<Mod>) -> Self {
        Project { manifest, mods }
    }

    pub fn format(path: PathBuf) -> Result<Self> {
        crate::format::format_all(path)
    }

    pub fn create(&self, path: PathBuf) -> Result<()> {
        crate::format::create_all(self, path)
    }
}

impl From<Manifest> for Project {
    fn from(value: Manifest) -> Self {
        Project {
            manifest: value,
            mods: vec![],
        }
    }
}
