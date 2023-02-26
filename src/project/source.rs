use reqwest::Url;
use serde::{Deserialize, Serialize};
use crate::project::source::Source::Download;
use crate::error::{Result, UnknownFileName};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all="snake_case")]
pub enum Source {
    Download {
        url: String
    }
}

impl Source {
    pub fn parse(string: &str) -> Source {
        Download {
            url: string.into()
        }
    }

    pub fn url(self) -> String {
        match self {
            Download { url } => url
        }
    }

    pub fn file_name(&self) -> Result<String> {
        match self {
            Download { url } => Ok(Url::parse(url)?
                .path_segments()
                .and_then(|segments| segments.last())
                .and_then(|name| if name.is_empty() { None } else { Some(name) })
                .ok_or(UnknownFileName)?
                .into())
        }
    }
}