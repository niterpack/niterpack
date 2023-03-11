use url::Url;
use serde::{Deserialize, Serialize};
use crate::project::source::Source::{Download, Modrinth};
use crate::error::{Error, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all="snake_case")]
pub enum Source {
    Download {
        url: String
    },
    Modrinth {
        version_id: String
    }
}

impl Source {
    pub fn parse(string: &str) -> Source {
        Download {
            url: string.into()
        }
    }

    pub fn url(&self) -> Result<String> {
        match self {
            Download { url } => Ok(url.clone()),
            Modrinth { version_id } => Ok(crate::modrinth::get_version(version_id)?
                .primary_file()
                .ok_or_else(|| Error::InvalidSource)?
                .url
                .clone())
        }
    }

    pub fn file_name(&self) -> Result<String> {
        match self {
            Download { url } => Url::parse(url)?
                .path_segments()
                .and_then(|segments| segments.last())
                .and_then(|name| if name.is_empty() { None } else { Some(name) })
                .map(|s| s.into())
                .ok_or_else(|| Error::InvalidSource),
            Modrinth { version_id } => Ok(crate::modrinth::get_version(version_id)?
                .primary_file()
                .ok_or_else(|| Error::InvalidSource)?
                .filename
                .clone())
        }
    }
}