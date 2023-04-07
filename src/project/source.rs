use crate::project::source::Source::{Download, Modrinth};
use eyre::{eyre, Result, WrapErr};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Source {
    Download { url: String },
    Modrinth { version_id: String },
}

impl Source {
    pub fn url(&self) -> Result<String> {
        match self {
            Download { url } => Ok(url.clone()),
            Modrinth { version_id } => Ok(crate::modrinth::get_version(version_id)
                .wrap_err("failed to get modrinth version")?
                .ok_or_else(|| eyre!("invalid modrinth version id"))?
                .primary_file()
                .ok_or_else(|| eyre!("primary file not found"))?
                .url
                .clone()),
        }
    }

    pub fn file_name(&self) -> Result<String> {
        match self {
            Download { url } => Url::parse(url)?
                .path_segments()
                .and_then(|segments| segments.last())
                .and_then(|name| if name.is_empty() { None } else { Some(name) })
                .map(|s| s.into())
                .ok_or_else(|| eyre!("invalid url")),
            Modrinth { version_id } => Ok(crate::modrinth::get_version(version_id)
                .wrap_err("failed to get modrinth version")?
                .ok_or_else(|| eyre!("invalid modrinth version id"))?
                .primary_file()
                .ok_or_else(|| eyre!("primary file not found"))?
                .filename
                .clone()),
        }
    }
}
