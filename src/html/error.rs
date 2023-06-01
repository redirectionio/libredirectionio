use std::result;

/// This error describes all of the potential failures that can occur during the html parse process.
#[derive(Debug)]
#[non_exhaustive]
pub enum HtmlParseError {
    FromUtf8Error(std::string::FromUtf8Error),
}

impl std::fmt::Display for HtmlParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FromUtf8Error(source) => write!(f, "{source}"),
        }
    }
}

impl std::error::Error for HtmlParseError {}

impl From<std::string::FromUtf8Error> for HtmlParseError {
    fn from(error: std::string::FromUtf8Error) -> Self {
        Self::FromUtf8Error(error)
    }
}

pub type Result<T> = result::Result<T, HtmlParseError>;
