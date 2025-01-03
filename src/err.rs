use std::{error, fmt};

pub type Result<T> = std::result::Result<T, JellyfinError>;

#[derive(Debug)]
pub enum JellyfinError {
    #[cfg(not(feature = "tracing"))]
    NetworkError(reqwest::Error),
    #[cfg(feature = "tracing")]
    NetworkError(reqwest_middleware::Error),
    UrlParseError(url::ParseError),
    AuthNotFound,
}

impl fmt::Display for JellyfinError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NetworkError(v) => {
                write!(f, "{}", v)
            }
            Self::UrlParseError(v) => {
                write!(f, "{}", v)
            }
            Self::AuthNotFound => {
                write!(f, "Unauthorized.")
            }
        }
    }
}

impl error::Error for JellyfinError {}

#[cfg(not(feature = "tracing"))]
impl From<reqwest::Error> for JellyfinError {
    fn from(value: reqwest::Error) -> Self {
        Self::NetworkError(value)
    }
}

#[cfg(feature = "tracing")]
impl From<reqwest_middleware::Error> for JellyfinError {
    fn from(value: reqwest_middleware::Error) -> Self {
        Self::NetworkError(value)
    }
}

#[cfg(feature = "tracing")]
impl From<reqwest::Error> for JellyfinError {
    fn from(value: reqwest::Error) -> Self {
        Self::NetworkError(reqwest_middleware::Error::Reqwest(value))
    }
}


impl From<url::ParseError> for JellyfinError {
    fn from(value: url::ParseError) -> Self {
        Self::UrlParseError(value)
    }
}
