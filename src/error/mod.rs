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
