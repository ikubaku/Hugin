use std::collections::HashMap;
use std::path::PathBuf;

use serde_derive::Deserialize;

pub mod ccfindersw;

#[derive(Deserialize, PartialEq)]
pub enum CloneDetectorKind {
    CCFinderSW,
}

#[derive(Deserialize)]
pub struct Config {
    pub(crate) munin_database_root: PathBuf,
    clone_detector_kind: CloneDetectorKind,
    clone_detector_config: HashMap<String, String>,
}
