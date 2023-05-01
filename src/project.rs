use crate::Source;
use crate::Source::{Download, Modrinth};
use eyre::{eyre, Result, WrapErr};
use std::path::Path;
use url::Url;

#[derive(Debug, Clone)]
pub struct Manifest {
    pub name: String,
    pub version: String,
    pub minecraft_version: Option<String>,
    pub loader: Option<String>,
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
    pub fn new(
        name: String,
        version: String,
        minecraft_version: Option<String>,
        loader: Option<String>,
    ) -> Self {
        Self {
            name,
            version,
            minecraft_version,
            loader,
        }
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
                None => crate::modrinth::get_versions(&self.name, None, None)
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
                    None => crate::modrinth::get_versions(&self.name, None, None)
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

    pub fn read<P: AsRef<Path>>(path: P) -> Result<Self> {
        crate::toml::read_project(path)
    }

    pub fn write<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        crate::toml::write_project(path, self.clone())
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
