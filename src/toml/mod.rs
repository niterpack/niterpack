use crate::{Manifest, Mod, Project, Source};
use eyre::{Result, WrapErr};
use serde::{Deserialize, Serialize};
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

pub fn read_project<P: AsRef<Path>>(path: P) -> Result<Project> {
    let manifest = read_manifest(path.as_ref().join_manifest_file())
        .wrap_err("failed to read manifest file")?;
    let mods =
        read_mods(path.as_ref().join_mods_dir()).wrap_err("failed to read mods directory")?;

    Ok(Project::new(manifest, mods, None))
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
        let mod_path = entry?.path();
        if !mod_path.is_file() || mod_path.extension().and_then(OsStr::to_str) != Some("toml") {
            continue;
        }

        let mod_data = read_mod(mod_path).wrap_err("failed to read mod file")?;
        mods.push(mod_data);
    }
    Ok(mods)
}

pub fn read_mod<P: AsRef<Path>>(path: P) -> Result<Mod> {
    let string = fs::read_to_string(path.as_ref())?;
    read_mod_from_str(&string)
}

pub fn read_mod_from_str(string: &str) -> Result<Mod> {
    let mod_data = toml::from_str::<TomlMod>(string)?;
    Ok(mod_data.into())
}

pub fn write_project<P: AsRef<Path>>(path: P, project: Project) -> Result<()> {
    write_manifest(path.as_ref().join_manifest_file(), project.manifest)
        .wrap_err("failed to write manifest file")?;
    write_mods(path.as_ref().join_mods_dir(), project.mods)
        .wrap_err("failed to write to mods directory")?;

    Ok(())
}

pub fn write_manifest<P: AsRef<Path>>(path: P, manifest: Manifest) -> Result<()> {
    let string = toml::to_string(&TomlManifest::from(manifest))?;
    fs::write(path, string)?;
    Ok(())
}

pub fn write_mods<P: AsRef<Path>>(path: P, mods: Vec<Mod>) -> Result<()> {
    if !path.as_ref().exists() {
        fs::create_dir(&path)?;
    }
    for mod_data in mods {
        write_mod(path.as_ref().join_mod_file(&mod_data.name), mod_data)
            .wrap_err("failed to write mod file")?;
    }
    Ok(())
}

pub fn write_mod<P: AsRef<Path>>(path: P, mod_data: Mod) -> Result<()> {
    let string = toml::to_string(&TomlMod::from(mod_data))?;
    fs::write(path, string)?;
    Ok(())
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TomlManifest {
    pub modpack: TomlManifestModpack,
    pub minecraft: Option<TomlManifestMinecraft>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TomlManifestModpack {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TomlManifestMinecraft {
    pub loader: Option<String>,
    pub version: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TomlMod {
    pub name: String,
    pub file: Option<String>,
    #[serde(flatten)]
    pub source: Source,
}

impl From<TomlManifest> for Manifest {
    fn from(value: TomlManifest) -> Self {
        Manifest::new(
            value.modpack.name,
            value.modpack.version,
            value
                .minecraft
                .clone()
                .and_then(|minecraft| minecraft.version),
            value.minecraft.and_then(|minecraft| minecraft.loader),
        )
    }
}

impl From<Manifest> for TomlManifest {
    fn from(value: Manifest) -> Self {
        TomlManifest {
            modpack: TomlManifestModpack {
                name: value.name,
                version: value.version,
            },
            minecraft: None,
        }
    }
}

impl From<TomlManifest> for Project {
    fn from(value: TomlManifest) -> Self {
        Project::from(Manifest::from(value))
    }
}

impl From<TomlMod> for Mod {
    fn from(value: TomlMod) -> Self {
        Mod::new(value.name, value.file, value.source)
    }
}

impl From<Mod> for TomlMod {
    fn from(value: Mod) -> Self {
        TomlMod {
            name: value.name,
            file: value.file,
            source: value.source,
        }
    }
}

pub trait JoinToml {
    fn join_manifest_file(&self) -> PathBuf;
    fn join_mods_dir(&self) -> PathBuf;
    fn join_mod_file(&self, name: &str) -> PathBuf;
}

impl JoinToml for Path {
    fn join_manifest_file(&self) -> PathBuf {
        self.join("niterpack").with_extension("toml")
    }

    fn join_mods_dir(&self) -> PathBuf {
        self.join("mods")
    }

    fn join_mod_file(&self, name: &str) -> PathBuf {
        self.join(name).with_extension("toml")
    }
}
