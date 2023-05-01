#[derive(thiserror::Error, Debug)]
pub enum ModrinthError {
    #[error("reqwest error")]
    Reqwest(#[from] reqwest::Error),

    #[error("error while serializing response")]
    Serialization(#[from] serde_json::Error),

    #[error("error while parsing url")]
    Url(#[from] url::ParseError),

    #[error("invalid slug or id `{0}`")]
    InvalidSlugOrId(String),
}
