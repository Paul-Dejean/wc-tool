use std::{fmt, io};

#[derive(Debug)]
pub enum ProcessFileError {
    IoError(io::Error),
    FileNotFound(String),
}

impl fmt::Display for ProcessFileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProcessFileError::IoError(err) => write!(f, "I/O error: {}", err),
            ProcessFileError::FileNotFound(path) => write!(f, "File not found: {}", path),
        }
    }
}

impl std::error::Error for ProcessFileError {}

impl From<io::Error> for ProcessFileError {
    fn from(err: io::Error) -> Self {
        ProcessFileError::IoError(err)
    }
}
