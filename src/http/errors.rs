use thiserror::Error;

#[derive(Debug, Error, uniffi::Error)]
#[uniffi(flat_error)]
pub enum HttpError {
    #[error("failed to parse URL: {0}")]
    UrlParsing(url::ParseError),
    #[error("HTTP request failed: {0}")]
    Request(reqwest::Error),
    #[error("failed to serialize payload: {0}")]
    Serialize(serde_json::Error),
    #[error("this endpoint requires authentication")]
    Unauthenticated,
    #[error("session lock is poisoned")]
    PoisonedSession,
    #[error("esup-multi api returned {0}: {1}")]
    InvalidStatus(reqwest::StatusCode, String),
}
