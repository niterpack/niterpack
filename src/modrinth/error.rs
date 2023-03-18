use reqwest::StatusCode;

#[derive(thiserror::Error, Debug)]
pub enum ModrinthError {
    #[error("error while sending request")]
    Request(#[from] reqwest::Error),

    #[error("error while serializing response")]
    Serialization(#[from] serde_json::Error),

    #[error("error while parsing url")]
    Url(#[from] url::ParseError),

    #[error("unexpected status code `{0}` in response")]
    UnexpectedStatusCode(StatusCode),
}