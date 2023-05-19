use std::{fs, path::Path};

use eyre::Result;
use serde::{Deserialize, Serialize};

use crate::{source::BuildSource, Manifest};

pub fn write_index<P: AsRef<Path>>(
    path: P,
    manifest: &Manifest,
    sources: Vec<BuildSource>,
) -> Result<()> {
    let index = Index::from_project(manifest, sources);
    let string = serde_json::to_string(&index)?;
    fs::write(path, string)?;
    Ok(())
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Index {
    #[serde(rename = "formatVersion")]
    format: i32,
    game: Game,

    name: String,

    #[serde(rename = "versionId")]
    version: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    summary: Option<String>,

    files: Vec<File>,
    dependencies: Dependencies,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Game {
    Minecraft,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct File {
    path: String,
    hashes: Hashes,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "env")]
    environments: Option<Environments>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Hashes {
    sha1: String,
    sha512: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Environments {
    client: EnvironmentSupport,
    server: EnvironmentSupport,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum EnvironmentSupport {
    Required,
    Optional,
    Unsupported,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Dependencies {
    minecraft: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    forge: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "fabric-loader")]
    fabric: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "quilt-loader")]
    quilt: Option<String>,
}

impl Index {
    pub fn new(
        name: String,
        version: String,
        summary: Option<String>,
        files: Vec<File>,
        dependencies: Dependencies,
    ) -> Self {
        Self {
            format: 1,
            game: Game::Minecraft,

            name,
            version,
            summary,
            files,
            dependencies,
        }
    }

    pub fn from_project(manifest: &Manifest, sources: Vec<BuildSource>) -> Self {
        Self::new(
            manifest.name.clone(),
            manifest.version.clone(),
            None,
            sources
                .iter()
                .map(|source| source.to_owned().into())
                .collect(),
            todo!(),
        )
    }
}

impl File {
    pub fn new(path: String, hashes: Hashes, environments: Option<Environments>) -> Self {
        Self {
            path,
            hashes,
            environments,
        }
    }
}

impl From<BuildSource> for File {
    fn from(value: BuildSource) -> Self {
        Self::new(
            format!("mods/{}", value.file),
            Hashes::new(value.sha1, value.sha512),
            None,
        )
    }
}

impl Hashes {
    pub fn new(sha1: String, sha512: String) -> Self {
        Self { sha1, sha512 }
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::Minecraft
    }
}

impl Dependencies {
    pub fn forge(minecraft: String, forge: String) -> Self {
        Self {
            minecraft,
            forge: Some(forge),
            fabric: None,
            quilt: None,
        }
    }

    pub fn fabric(minecraft: String, fabric: String) -> Self {
        Self {
            minecraft,
            forge: None,
            fabric: Some(fabric),
            quilt: None,
        }
    }

    pub fn quilt(minecraft: String, quilt: String) -> Self {
        Self {
            minecraft,
            forge: None,
            fabric: None,
            quilt: Some(quilt),
        }
    }
}
