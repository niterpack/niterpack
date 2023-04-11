use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged, rename_all = "kebab-case")]
pub enum Source {
    #[serde(rename_all = "kebab-case")]
    Download { url: String },
    #[serde(rename_all = "kebab-case")]
    Modrinth { version: String },
}
