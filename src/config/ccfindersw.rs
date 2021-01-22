use std::path::PathBuf;
use std::collections::HashMap;
use std::str::FromStr;
use std::fmt;

use log::error;

use crate::config::{Config, CloneDetectorKind};

#[derive(Debug)]
pub struct InvalidConfigurationError {
    description: String,
}

impl InvalidConfigurationError {
    pub fn new(desc: &str) -> Self {
        InvalidConfigurationError {
            description: String::from(desc)
        }
    }
}

impl fmt::Display for InvalidConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}

#[derive(Debug, PartialEq)]
pub enum Languages {
    CPlusPlus,
}

fn deserialize_language(s: &str) -> Result<Languages, ()> {
    if s == "CPlusPlus" {
        return Ok(Languages::CPlusPlus)
    } else {
        Err(())
    }
}

#[derive(Debug)]
pub struct CCFinderSWConfig {
    executable_path: PathBuf,
    token_length: u32,
    language: Languages,
    extensions: Vec<String>,
}

impl CCFinderSWConfig {
    pub fn try_from_config(config: Config) -> Option<Self> {
        if config.clone_detector_kind != CloneDetectorKind::CCFinderSW {
            None
        } else {
            match CCFinderSWConfig::from_hashmap(config.clone_detector_config) {
                Ok(c) => Some(c),
                Err(e) => {
                    error!("Invalid configuration: {:?}", e);
                    None
                }
            }
        }
    }

    fn from_hashmap(hashmap: HashMap<String, String>) -> Result<Self, InvalidConfigurationError> {
        let executable_path = PathBuf::from(
            hashmap.get("executable_path")
                .ok_or(InvalidConfigurationError::new("Missing key: `executable_path`"))?
        );
        let token_length = u32::from_str(
            hashmap.get("token_length")
                .ok_or(InvalidConfigurationError::new("Missing key: `token_length`"))?)
            .map_err(|_| InvalidConfigurationError::new("Invalid value for `token_length`"))?;
        let language = deserialize_language(
            hashmap.get("language")
                .ok_or(InvalidConfigurationError::new("Missing key: `language`"))?
        ).or_else(|_| Err(InvalidConfigurationError::new("Invalid value for `language`")))?;
        let extensions: Vec<String> = hashmap.get("extensions")
            .ok_or(InvalidConfigurationError::new("Missing key `extensions`"))?
            .split(",").map(|s| String::from(s)).collect();

        Ok(CCFinderSWConfig {
            executable_path,
            token_length,
            language,
            extensions,
        })
    }
}
