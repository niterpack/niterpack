mod error;

use crate::modrinth::error::ModrinthError;
use reqwest::StatusCode;
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

pub fn check_slug(slug: &str) -> bool {
    return lazy_regex::regex_is_match!(r#"^[\w!@$()`.+,"\-']{3,64}$"#, slug);
}

pub fn check_id(id: &str) -> bool {
    return lazy_regex::regex_is_match!(r#"^[a-zA-Z0-9]{8}$"#, id);
}

pub fn get_version(id: &str) -> Result<Option<ModrinthVersion>, ModrinthError> {
    let response = reqwest::blocking::get(format!("https://api.modrinth.com/v2/version/{}", id))?;
    match response.status() {
        StatusCode::NOT_FOUND => Ok(None),
        StatusCode::BAD_REQUEST => Ok(None),
        StatusCode::OK => Ok(Some(serde_json::from_str(response.text()?.as_str())?)),
        status => Err(ModrinthError::UnexpectedStatusCode(status)),
    }
}

pub fn get_project(id: &str) -> Result<Option<ModrinthProject>, ModrinthError> {
    let response = reqwest::blocking::get(format!("https://api.modrinth.com/v2/project/{}", id))?;
    match response.status() {
        StatusCode::NOT_FOUND => Ok(None),
        StatusCode::BAD_REQUEST => Ok(None),
        StatusCode::OK => Ok(Some(serde_json::from_str(response.text()?.as_str())?)),
        status => Err(ModrinthError::UnexpectedStatusCode(status)),
    }
}

pub fn get_versions(id: &str) -> Result<Vec<ModrinthVersion>, ModrinthError> {
    let response = reqwest::blocking::get(format!(
        "https://api.modrinth.com/v2/project/{}/version",
        id
    ))?;
    match response.status() {
        StatusCode::NOT_FOUND => Ok(vec![]),
        StatusCode::BAD_REQUEST => Ok(vec![]),
        StatusCode::OK => Ok(serde_json::from_str(response.text()?.as_str())?),
        status => Err(ModrinthError::UnexpectedStatusCode(status)),
    }
}
