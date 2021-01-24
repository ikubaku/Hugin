use std::collections::HashMap;
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
    pub(crate) munin_database_root: PathBuf,
    clone_detector_kind: CloneDetectorKind,
    clone_detector_config: HashMap<String, String>,
}

impl Config {
    pub fn default() -> Self {
        Config {
            munin_database_root: PathBuf::from("~/munin"),
            clone_detector_kind: CloneDetectorKind::CCFinderSW,
            clone_detector_config: CCFinderSWConfig::default().to_hashmap(),
        }
    }
}
