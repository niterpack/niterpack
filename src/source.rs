use crate::Mod;
use eyre::{ContextCompat, Result, WrapErr};
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

pub struct BuildSource {
    pub name: String,
    pub url: String,
    pub file: String,
}

impl BuildSource {
    pub fn from_mod(mod_data: &Mod) -> Result<BuildSource> {
        Ok(match &mod_data.source {
            Source::Download { url } => BuildSource {
                name: mod_data.name.to_string(),
                file: Url::parse(url)?
                    .path_segments()
                    .and_then(|segments| segments.last())
                    .and_then(|name| if name.is_empty() { None } else { Some(name) })
                    .map(|s| s.into())
                    .wrap_err("invalid url")?,
                url: url.to_string(),
            },
            Source::Modrinth { version } => {
                let version = match crate::modrinth::version(version) {
                    Ok(version) => version,
                    Err(_) => crate::modrinth::project_versions(&mod_data.name, None, None)
                        .wrap_err("failed to fetch modrinth project versions")?
                        .into_iter()
                        .find(|modrinth_version| &modrinth_version.version_number == version)
                        .wrap_err(format!("could not find version `{}`", version))?,
                };
                let file = version.primary_file().wrap_err("primary file not found")?;
                BuildSource {
                    name: mod_data.name.to_string(),
                    url: file.url.to_string(),
                    file: file.filename.to_string(),
                }
            }
        })
    }
}
