use std::io;

#[derive(thiserror::Error, Debug)]
pub enum FormatError {
    #[error("could not find `niter.json` in this directory")]
    MainFileNotFound,

    #[error("a modpack in this directory is already initialized")]
    AlreadyInitialized,

    #[error("error while deserializing")]
    Deserialization(#[from] toml::de::Error),

    #[error("error while serializing")]
    Serialization(#[from] toml::ser::Error),

    #[error("error while serializing")]
    Serde(#[from] serde_json::Error),

    #[error("error while performing I/O")]
    IO(#[from] io::Error),
}
