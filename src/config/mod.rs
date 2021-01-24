use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::path::PathBuf;

use serde_derive::Deserialize;

use crate::config::ccfindersw::CCFinderSWConfig;

pub mod ccfindersw;

#[derive(Clone, Deserialize, PartialEq)]
pub enum CloneDetectorKind {
    CCFinderSW,
}

#[derive(Clone, Deserialize)]
pub struct Config {
    munin_database_root: String,
    clone_detector_kind: CloneDetectorKind,
    clone_detector_config: HashMap<String, String>,
}

impl Config {
    pub fn default() -> Self {
        Config {
            munin_database_root: String::from("~/munin"),
            clone_detector_kind: CloneDetectorKind::CCFinderSW,
            clone_detector_config: CCFinderSWConfig::default().to_hashmap(),
        }
    }

    pub fn get_absolute_database_root_path(&self) -> Result<PathBuf, Box<dyn Error>> {
        Ok(
            PathBuf::from(shellexpand::tilde(self.munin_database_root.as_str()).as_ref())
                .canonicalize()?,
        )
    }
}
