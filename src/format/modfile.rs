use crate::project::source::Source;
use crate::project::Mod;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ModFile {
    pub name: Option<String>,
    pub file: Option<String>,
    #[serde(flatten)]
    pub source: Source,
}

impl ModFile {
    pub fn to_mod<F>(&self, f: F) -> Mod
    where
        F: FnOnce() -> String,
    {
        Mod {
            name: self.name.clone().unwrap_or_else(f),
            file: self.file.clone(),
            source: self.source.clone(),
        }
    }
}

impl From<Mod> for ModFile {
    fn from(value: Mod) -> Self {
        ModFile {
            name: Some(value.name),
            source: value.source,
            file: value.file,
        }
    }
}
