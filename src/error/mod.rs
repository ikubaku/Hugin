use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct NoValidConfigurationError;

impl fmt::Display for NoValidConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "No valid configuration found.")
    }
}

impl Error for NoValidConfigurationError {}

#[derive(Debug)]
pub struct InvalidPathError {
    path: String,
}

impl InvalidPathError {
    pub fn new(path: &str) -> Self {
        InvalidPathError {
            path: String::from(path),
        }
    }
}

impl fmt::Display for InvalidPathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid path was specified: {}.", self.path)
    }
}

impl Error for InvalidPathError {}
