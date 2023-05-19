pub mod error;

use error::ModrinthError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub slug: String,
    pub id: String,
    pub project_type: ProjectType,
    pub versions: Vec<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProjectType {
    Mod,
    Modpack,
    ResourcePack,
    Shader,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Version {
    pub id: String,
    pub name: String,
    pub version_number: String,
    pub files: Vec<VersionFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionFile {
    pub url: String,
    pub filename: String,
    pub primary: bool,
    pub hashes: ModrinthVersionFileHashes,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModrinthVersionFileHashes {
    pub sha512: String,
    pub sha1: String,
}

impl Version {
    pub fn primary_file(&self) -> Option<&VersionFile> {
        self.files
            .iter()
            .find(|file| file.primary)
            .or_else(|| self.files.first())
    }
}

macro_rules! get {
    (
        path: [$( $path:expr ),+],
        $(query: { $($query:tt)* },)?
    ) => {
        let request = reqwest::blocking::Client::builder()
            .build()?
            .get(format!("https://api.modrinth.com/v2/{}", vec![$($path),*].join("/")));

        $(
        let mut query = Vec::new();
        for (key, value) in serde_json::json!({$($query)*}).as_object().unwrap() {
            if value == &serde_json::Value::Null {
                continue;
            } else if let serde_json::Value::String(value) = value {
                query.push((key.to_string(), value.to_string()))
            } else {
                query.push((key.to_string(), serde_json::to_string(&value)?))
            }
        }
        let request = request.query(&query);
        )?

        let response = request.send()?;

        Ok(serde_json::from_str(response.error_for_status()?
            .text()?
            .as_str())?)
    };
}

pub fn check_slug(slug: &str) -> bool {
    lazy_regex::regex_is_match!(r#"^[\w!@$()`.+,"\-']{3,64}$"#, slug)
}

fn check_slug_err(slug: &str) -> Result<(), ModrinthError> {
    if check_slug(slug) {
        Ok(())
    } else {
        Err(ModrinthError::InvalidSlugOrId(slug.to_string()))
    }
}

pub fn check_id(id: &str) -> bool {
    lazy_regex::regex_is_match!(r#"^[a-zA-Z0-9]{8}$"#, id)
}

fn check_id_err(id: &str) -> Result<(), ModrinthError> {
    if check_id(id) {
        Ok(())
    } else {
        Err(ModrinthError::InvalidSlugOrId(id.to_string()))
    }
}

pub fn version(id: &str) -> Result<Version, ModrinthError> {
    check_id_err(id)?;
    get! {
        path: ["version", id],
    }
}

pub fn project(id: &str) -> Result<Project, ModrinthError> {
    check_slug_err(id)?;
    get! {
        path: ["project", id],
    }
}

pub fn project_versions(
    id: &str,
    loader: Option<&str>,
    game_version: Option<&str>,
) -> Result<Vec<Version>, ModrinthError> {
    check_slug_err(id)?;
    get! {
        path: ["project", id, "version"],
        query: {
            "loaders": loader.map(|loader| vec![loader]),
            "game_versions": game_version.map(|game_version| vec![game_version]),
        },
    }
}
