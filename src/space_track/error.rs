use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Auth error: {source}")]
    AuthError { source: reqwest::Error },
    #[error("Request error: {source}")]
    RequestError {
        #[from]
        source: reqwest::Error,
    },
    #[error("Invalid cookie")]
    CookieError,
    #[error("Cookie parse error: {source}")]
    CookieParseError {
        #[from]
        source: reqwest::header::ToStrError,
    },
}
