use reqwest::Url;
use crate::project::source::Source::Download;
use crate::error::{Result, UnknownFileName};

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

    pub fn file_name(&self) -> Result<&str> {
        match self {
            Download { url } => Url::parse(url)?
                .path_segments()
                .and_then(|segments| segments.last())
                .and_then(|name| if name.is_empty() { None } else { Some(name) })
                .ok_or(UnknownFileName)?
        }
    }
}