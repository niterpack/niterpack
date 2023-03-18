mod error;

use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use crate::modrinth::error::ModrinthError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModrinthProject {
    pub slug: String,
    pub id: String,
    pub project_type: ModrinthProjectType,
    pub versions: Vec<String>
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ModrinthProjectType {
    Mod,
    Modpack,
    ResourcePack,
    Shader
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModrinthVersion {
    pub id: String,
    pub name: String,
    pub version_number: String,
    pub files: Vec<ModrinthVersionFile>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModrinthVersionFile {
    pub url: String,
    pub filename: String,
    pub primary: bool
}

impl ModrinthVersion {
    pub fn primary_file(&self) -> Option<&ModrinthVersionFile> {
        self.files.iter().find(|file| file.primary == true)
    }
}

pub fn get_version(id: &str) -> Result<Option<ModrinthVersion>, ModrinthError> {
    let response = reqwest::blocking::get(format!("https://api.modrinth.com/v2/version/{}", id))?;
    match response.status() {
        StatusCode::NOT_FOUND => Ok(None),
        StatusCode::OK => Ok(Some(serde_json::from_str(response.text()?.as_str())?)),
        status => Err(ModrinthError::UnexpectedStatusCode(status))
    }
}

pub fn get_project(id: &str) -> Result<Option<ModrinthProject>, ModrinthError> {
    let response = reqwest::blocking::get(format!("https://api.modrinth.com/v2/project/{}", id))?;
    match response.status() {
        StatusCode::NOT_FOUND => Ok(None),
        StatusCode::OK => Ok(Some(serde_json::from_str(response.text()?.as_str())?)),
        status => Err(ModrinthError::UnexpectedStatusCode(status))
    }
}

pub fn get_versions(id: &str) -> Result<Vec<ModrinthVersion>, ModrinthError> {
    let response = reqwest::blocking::get(format!("https://api.modrinth.com/v2/project/{}/version", id))?;
    match response.status() {
        StatusCode::NOT_FOUND => Ok(vec![]),
        StatusCode::OK => Ok(serde_json::from_str(response.text()?.as_str())?),
        status => Err(ModrinthError::UnexpectedStatusCode(status))
    }
}
