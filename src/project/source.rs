use crate::project::source::Source::{Download, Modrinth};
use eyre::{eyre, Result, WrapErr};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged, rename_all = "kebab-case")]
pub enum Source {
    #[serde(rename_all = "kebab-case")]
    Download { url: String },
    #[serde(rename_all = "kebab-case")]
    Modrinth { version: String },
}

impl Source {
    pub fn url(&self, name: &str) -> Result<String> {
        match self {
            Download { url } => Ok(url.clone()),
            Modrinth { version } => Ok(
                match crate::modrinth::get_version(version)
                    .wrap_err("failed to fetch modrinth version")?
                {
                    Some(version) => version.id,
                    None => {
                        crate::modrinth::get_versions(name)
                            .wrap_err("failed to fetch modrinth project versions")?
                            .into_iter()
                            .find(|modrinth_version| &modrinth_version.version_number == version)
                            .ok_or_else(|| eyre!("could not find version `{}`", version))?
                            .id
                    }
                },
            ),
        }
    }

    pub fn file_name(&self, name: &str) -> Result<String> {
        match self {
            Download { url } => Url::parse(url)?
                .path_segments()
                .and_then(|segments| segments.last())
                .and_then(|name| if name.is_empty() { None } else { Some(name) })
                .map(|s| s.into())
                .ok_or_else(|| eyre!("invalid url")),
            Modrinth { version } => Ok(match crate::modrinth::get_version(version)
                .wrap_err("failed to fetch modrinth version")?
            {
                Some(version) => version,
                None => crate::modrinth::get_versions(name)
                    .wrap_err("failed to fetch modrinth project versions")?
                    .into_iter()
                    .find(|modrinth_version| &modrinth_version.version_number == version)
                    .ok_or_else(|| eyre!("could not find version `{}`", version))?,
            }
            .primary_file()
            .ok_or_else(|| eyre!("primary file not found"))?
            .filename
            .clone()),
        }
    }
}
