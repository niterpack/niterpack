mod error;

use crate::modrinth::error::ModrinthError;
use reqwest::StatusCode;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModrinthProject {
    pub slug: String,
    pub id: String,
    pub project_type: ModrinthProjectType,
    pub versions: Vec<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ModrinthProjectType {
    Mod,
    Modpack,
    ResourcePack,
    Shader,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModrinthVersion {
    pub id: String,
    pub name: String,
    pub version_number: String,
    pub files: Vec<ModrinthVersionFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModrinthVersionFile {
    pub url: String,
    pub filename: String,
    pub primary: bool,
}

impl ModrinthVersion {
    pub fn primary_file(&self) -> Option<&ModrinthVersionFile> {
        self.files.iter().find(|file| file.primary)
    }
}

fn fetch<T: Default + DeserializeOwned>(paths: Vec<&str>) -> Result<T, ModrinthError> {
    let response = reqwest::blocking::Client::builder()
        .build()?
        .get(format!("https://api.modrinth.com/v2/{}", paths.join("/")))
        .send()?;
    match response.status() {
        StatusCode::NOT_FOUND => Ok(T::default()),
        StatusCode::OK => Ok(serde_json::from_str(response.text()?.as_str())?),
        status => Err(ModrinthError::UnexpectedStatusCode(status)),
    }
}

fn fetch_with_query<T: Default + DeserializeOwned, Q: AsRef<str> + Serialize>(
    paths: Vec<&str>,
    query: Vec<(&str, Q)>,
) -> Result<T, ModrinthError> {
    let response = reqwest::blocking::Client::builder()
        .build()?
        .get(format!("https://api.modrinth.com/v2/{}", paths.join("/")))
        .query(&query)
        .send()?;
    match response.status() {
        StatusCode::NOT_FOUND => Ok(T::default()),
        StatusCode::OK => Ok(serde_json::from_str(response.text()?.as_str())?),
        status => Err(ModrinthError::UnexpectedStatusCode(status)),
    }
}

pub fn check_slug(slug: &str) -> bool {
    lazy_regex::regex_is_match!(r#"^[\w!@$()`.+,"\-']{3,64}$"#, slug)
}

pub fn check_id(id: &str) -> bool {
    lazy_regex::regex_is_match!(r#"^[a-zA-Z0-9]{8}$"#, id)
}

pub fn get_version(id: &str) -> Result<Option<ModrinthVersion>, ModrinthError> {
    if !check_id(id) {
        return Ok(None);
    }

    fetch(vec!["version", id])
}

pub fn get_project(id: &str) -> Result<Option<ModrinthProject>, ModrinthError> {
    if !check_slug(id) {
        return Ok(None);
    }

    fetch(vec!["project", id])
}

pub fn get_versions(
    id: &str,
    loader: Option<&str>,
    game_version: Option<&str>,
) -> Result<Vec<ModrinthVersion>, ModrinthError> {
    if !check_slug(id) {
        return Ok(vec![]);
    }

    let mut query = Vec::new();
    if let Some(loader) = loader {
        query.push(("loaders", serde_json::to_string(&vec![loader])?));
    }
    if let Some(game_version) = game_version {
        query.push(("game_versions", serde_json::to_string(&vec![game_version])?));
    }

    fetch_with_query(vec!["project", id, "version"], query)
}
