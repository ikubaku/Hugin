use std::path::PathBuf;

use semver::Version;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SourceInfo {
    location: PathBuf,
}

#[derive(Debug, Deserialize)]
pub struct LibraryInfo {
    name: String,
    version: Version,
    location: PathBuf,
}

#[derive(Debug, Deserialize)]
pub struct Job {
    project: SourceInfo,
    example_sketch: SourceInfo,
    library_info: LibraryInfo,
}
