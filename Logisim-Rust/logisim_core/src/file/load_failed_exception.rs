//! Load failed exception
//!
//! Exception thrown when circuit file loading fails

/// Exception thrown when circuit file loading fails
/// Equivalent to Java's LoadFailedException
#[derive(Debug, thiserror::Error)]
pub enum LoadFailedException {
    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("Invalid file format: {0}")]
    InvalidFormat(String),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("IO error: {0}")]
    IoError(String),

    #[error("Library not found: {0}")]
    LibraryNotFound(String),

    #[error("Unsupported version: {0}")]
    UnsupportedVersion(String),
}

impl From<std::io::Error> for LoadFailedException {
    fn from(error: std::io::Error) -> Self {
        LoadFailedException::IoError(error.to_string())
    }
}

impl From<crate::circ_parser::CircParseError> for LoadFailedException {
    fn from(error: crate::circ_parser::CircParseError) -> Self {
        LoadFailedException::ParseError(error.to_string())
    }
}
