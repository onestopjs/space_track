use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Request error: {source}")]
    AuthError {
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
