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

#[derive(Debug)]
pub struct InvalidConfigurationError {
    description: String,
}

impl InvalidConfigurationError {
    pub fn new(desc: &str) -> Self {
        InvalidConfigurationError {
            description: String::from(desc),
        }
    }
}

impl fmt::Display for InvalidConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}

impl Error for InvalidConfigurationError {}

#[derive(Debug)]
pub struct RunnerProcessFailedError {
    status_code: i32,
}

impl RunnerProcessFailedError {
    pub fn new(status_code: i32) -> Self {
        RunnerProcessFailedError { status_code }
    }
}

impl fmt::Display for RunnerProcessFailedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Process exited abnormally with code: {}",
            self.status_code
        )
    }
}

impl Error for RunnerProcessFailedError {}

#[derive(Debug)]
pub struct InvalidCCFinderSWResult;

impl fmt::Display for InvalidCCFinderSWResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "The result is in invalid format.")
    }
}

impl Error for InvalidCCFinderSWResult {}

#[derive(Debug)]
pub struct FileNotFoundFromResultError;

impl fmt::Display for FileNotFoundFromResultError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "The file is not found from the result.")
    }
}

impl Error for FileNotFoundFromResultError {}
