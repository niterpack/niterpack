use crate::{Manifest, Mod, Project, Source};
use eyre::{ContextCompat, Result, WrapErr};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

pub fn read_project<P: AsRef<Path>>(path: P) -> Result<Project> {
    let manifest =
        read_manifest(path.as_ref().join("niter.toml")).wrap_err("failed to read manifest file")?;
    let mods = read_mods(path.as_ref().join("mods")).wrap_err("failed to read mods directory")?;

    Ok(Project::with_mods(manifest, mods))
}

pub fn read_manifest<P: AsRef<Path>>(path: P) -> Result<Manifest> {
    let string = fs::read_to_string(path)?;
    read_manifest_from_str(&string)
}

pub fn read_manifest_from_str(string: &str) -> Result<Manifest> {
    let manifest = toml::from_str::<TomlManifest>(string)?;
    Ok(manifest.into())
}

pub fn read_mods<P: AsRef<Path>>(path: P) -> Result<Vec<Mod>> {
    let mut mods = Vec::new();
    for entry in fs::read_dir(path)? {
        let mod_data = read_mod(entry?.path()).wrap_err("failed to read mod file")?;
        mods.push(mod_data);
    }
    Ok(mods)
}

pub fn read_mod<P: AsRef<Path>>(path: P) -> Result<Mod> {
    let string = fs::read_to_string(path.as_ref())?;
    read_mod_from_str(
        path.as_ref()
            .file_name()
            .and_then(|name| name.to_os_string().into_string().ok())
            .wrap_err("failed to get file name")?,
        &string,
    )
}

pub fn read_mod_from_str(name: String, string: &str) -> Result<Mod> {
    let mod_data = toml::from_str::<TomlMod>(string)?;
    Ok(mod_data.into_mod(name))
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TomlManifest {
    pub modpack: TomlManifestModpack,
    pub minecraft: TomlManifestMinecraft,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TomlManifestModpack {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TomlManifestMinecraft {
    pub loader: String,
    pub version: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TomlMod {
    pub name: Option<String>,
    pub file: Option<String>,
    #[serde(flatten)]
    pub source: Source,
}

impl From<TomlManifest> for Manifest {
    fn from(value: TomlManifest) -> Self {
        Manifest::new(value.modpack.name, value.modpack.version)
    }
}

impl From<TomlManifest> for Project {
    fn from(value: TomlManifest) -> Self {
        Project::new(value.into())
    }
}

impl TomlMod {
    pub fn into_mod(self, name: String) -> Mod {
        Mod::new(self.name.unwrap_or(name), self.file, self.source)
    }
}
