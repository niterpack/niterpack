mod error;

use reqwest::StatusCode;
use url::Url;
use serde::{Deserialize, Serialize};
use crate::modrinth::error::ModrinthError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModrinthVersion {
    pub id: String,
    pub name: String,
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
    let response = reqwest::blocking::get(Url::parse("https://api.modrinth.com/v2/version/")?.join(id)?)?;
    match response.status() {
        StatusCode::NOT_FOUND => Ok(None),
        StatusCode::OK => Ok(Some(serde_json::from_str(response.text()?.as_str())?)),
        status => Err(ModrinthError::UnexpectedStatusCode(status))
    }
}
