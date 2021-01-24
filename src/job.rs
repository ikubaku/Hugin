use std::path::PathBuf;

use semver::Version;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SourceInfo {
    pub(crate) location: PathBuf,
}

#[derive(Debug, Deserialize)]
pub struct LibraryInfo {
    name: String,
    version: Version,
    pub(crate) location: PathBuf,
    pub(crate) archive_root: String,
}

#[derive(Debug, Deserialize)]
pub struct Job {
    pub(crate) project: SourceInfo,
    pub(crate) example_sketch: SourceInfo,
    pub(crate) library_info: LibraryInfo,
}
