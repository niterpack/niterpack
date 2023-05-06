use crate::Project;

#[derive(Debug, Clone)]
pub struct Manifest {
    pub name: String,
    pub version: String,
    pub minecraft_version: Option<String>,
    pub loader: Option<String>,
}

impl Manifest {
    pub fn new(
        name: String,
        version: String,
        minecraft_version: Option<String>,
        loader: Option<String>,
    ) -> Self {
        Self {
            name,
            version,
            minecraft_version,
            loader,
        }
    }
}

impl From<Project> for Manifest {
    fn from(value: Project) -> Self {
        value.manifest
    }
}
