use reqwest::StatusCode;

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

pub trait NotFound<T> {
    fn not_found(self) -> Result<Option<T>, ModrinthError>;
}

impl<T> NotFound<T> for Result<T, ModrinthError> {
    fn not_found(self) -> Result<Option<T>, ModrinthError> {
        match self {
            Ok(t) => Ok(Some(t)),
            Err(err) => {
                if let ModrinthError::Reqwest(ref err) = err {
                    if let Some(status) = err.status() {
                        if status == StatusCode::NOT_FOUND {
                            return Ok(None);
                        }
                    }
                }
                Err(err)
            }
        }
    }
}
