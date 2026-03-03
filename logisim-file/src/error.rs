//! File-format error types.

use thiserror::Error;

pub type Result<T> = std::result::Result<T, FileError>;

#[derive(Debug, Error)]
pub enum FileError {
    #[error("XML parse error: {0}")]
    Xml(#[from] quick_xml::Error),

    #[error("Attribute error: {0}")]
    Attr(#[from] quick_xml::events::attributes::AttrError),

    #[error("UTF-8 error: {0}")]
    Utf8(#[from] std::str::Utf8Error),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Missing required attribute: {0}")]
    MissingAttribute(String),

    #[error("Invalid coordinate: {0}")]
    InvalidCoord(String),

    #[error("Unknown component library: {0}")]
    UnknownLibrary(String),

    #[error("Unknown component: lib={lib}, name={name}")]
    UnknownComponent { lib: String, name: String },

    #[error("Format error: {0}")]
    Format(String),
}
