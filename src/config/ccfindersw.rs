use std::collections::HashMap;
use std::fmt;
use std::iter::FromIterator;
use std::path::PathBuf;
use std::str::FromStr;

use log::error;

use crate::config::{CloneDetectorKind, Config};
use crate::error::InvalidConfigurationError;

#[derive(Debug, PartialEq)]
pub enum Languages {
    CPlusPlus,
}

fn serialize_language(l: &Languages) -> String {
    match l {
        Languages::CPlusPlus => String::from("CPlusPlus"),
    }
}

fn deserialize_language(s: &str) -> Result<Languages, ()> {
    if s == "CPlusPlus" {
        Ok(Languages::CPlusPlus)
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
    pub fn try_from_config(config: &Config) -> Option<Self> {
        if config.clone_detector_kind != CloneDetectorKind::CCFinderSW {
            None
        } else {
            match CCFinderSWConfig::from_hashmap(&config.clone_detector_config) {
                Ok(c) => Some(c),
                Err(e) => {
                    error!("Invalid configuration: {:?}", e);
                    None
                }
            }
        }
    }

    pub fn default() -> Self {
        CCFinderSWConfig {
            executable_path: PathBuf::from("CCFinderSW"),
            token_length: 50,
            language: Languages::CPlusPlus,
            extensions: Vec::from([String::from("pde"), String::from("ino")]),
        }
    }

    pub fn to_hashmap(&self) -> HashMap<String, String> {
        [
            (
                String::from("executable_path"),
                String::from(self.executable_path.to_str().unwrap()),
            ),
            (String::from("token_length"), self.token_length.to_string()),
            (String::from("language"), serialize_language(&self.language)),
            (String::from("extensions"), self.extensions.join(",")),
        ]
        .iter()
        .cloned()
        .collect()
    }

    fn from_hashmap(hashmap: &HashMap<String, String>) -> Result<Self, InvalidConfigurationError> {
        let executable_path = PathBuf::from(
            shellexpand::tilde(hashmap.get("executable_path").ok_or(
                InvalidConfigurationError::new("Missing key: `executable_path`"),
            )?)
            .as_ref(),
        )
        .canonicalize()
        .or(Err(InvalidConfigurationError::new(
            "Could not canonicalize the specified path.",
        )))?;
        let token_length = u32::from_str(hashmap.get("token_length").ok_or(
            InvalidConfigurationError::new("Missing key: `token_length`"),
        )?)
        .map_err(|_| InvalidConfigurationError::new("Invalid value for `token_length`"))?;
        let language = deserialize_language(
            hashmap
                .get("language")
                .ok_or(InvalidConfigurationError::new("Missing key: `language`"))?,
        )
        .or_else(|_| {
            Err(InvalidConfigurationError::new(
                "Invalid value for `language`",
            ))
        })?;
        let extensions: Vec<String> = hashmap
            .get("extensions")
            .ok_or(InvalidConfigurationError::new("Missing key `extensions`"))?
            .split(",")
            .map(|s| String::from(s))
            .collect();

        Ok(CCFinderSWConfig {
            executable_path,
            token_length,
            language,
            extensions,
        })
    }

    pub fn get_executable_path_as_string(&self) -> String {
        String::from(self.executable_path.to_str().unwrap())
    }

    pub fn token_length_to_option_value(&self) -> String {
        self.token_length.to_string()
    }

    pub fn language_to_option_value(&self) -> String {
        match self.language {
            Languages::CPlusPlus => String::from("cpp"),
        }
    }

    pub fn extensions_to_option_value(&self) -> String {
        self.extensions.join("|")
    }
}
