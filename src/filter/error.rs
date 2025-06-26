use std::result;

/// This error describes all of the potential failures that can occur during the filter process.
#[derive(Debug)]
#[non_exhaustive]
pub enum FilterBodyError {
    /// Error while reading or writing to the buffer
    IoError(std::io::Error),
}

impl std::fmt::Display for FilterBodyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IoError(source) => write!(f, "{source}"),
        }
    }
}

impl std::error::Error for FilterBodyError {}

impl From<std::io::Error> for FilterBodyError {
    fn from(error: std::io::Error) -> Self {
        Self::IoError(error)
    }
}

pub type Result<T> = result::Result<T, FilterBodyError>;
