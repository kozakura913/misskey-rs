use std::convert::Infallible;

use thiserror::Error;

/// Possible errors from HTTP client.
#[derive(Debug, Error)]
pub enum Error {
    /// Errors from underlying [reqwest](https://docs.rs/reqwest) library.
	#[cfg(feature="reqwest-client")]
    #[error("network error: {0}")]
    Network(#[from] reqwest::Error),
    /// Errors from underlying [isahc](https://docs.rs/isahc) library.
	#[cfg(feature="isahc-client")]
    #[error("network error: {0}")]
    Network(#[from] isahc::Error),
    /// IO error.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    /// JSON encode/decode error.
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    /// Invalid URL.
    #[error("Invalid URL: {0}")]
    InvalidUrl(#[from] url::ParseError),
}

impl From<Infallible> for Error {
    fn from(x: Infallible) -> Error {
        match x {}
    }
}

/// Specialized Result type for operations on [`HttpClient`][`crate::HttpClient`].
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::Error;

    #[test]
    fn test_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Error>();
    }

    #[test]
    fn test_sync() {
        fn assert_send<T: Sync>() {}
        assert_send::<Error>();
    }
}
