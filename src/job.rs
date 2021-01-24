use std::error::Error;
use std::path::{Path, PathBuf};

use crate::error::InvalidPathError;
use semver::Version;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SourceInfo {
    location: String,
}

impl SourceInfo {
    pub fn get_location_from(&self, path: &Path) -> Result<PathBuf, Box<dyn Error>> {
        Ok(PathBuf::from(path)
            .join(Path::new(&self.location))
            .canonicalize()?)
    }

    pub fn get_non_canonical_path_from(&self, path: &Path) -> PathBuf {
        PathBuf::from(path).join(Path::new(&self.location))
    }

    pub fn get_file_name(&self) -> Result<String, Box<dyn Error>> {
        Ok(String::from(
            PathBuf::from(&self.location)
                .file_name()
                .ok_or(InvalidPathError::new(&self.location))?
                .to_str()
                .unwrap(),
        ))
    }
}

#[derive(Debug, Deserialize)]
pub struct LibraryInfo {
    name: String,
    version: Version,
    location: String,
    pub(crate) archive_root: String,
}

impl LibraryInfo {
    pub fn get_absolute_location(&self, database_path: &Path) -> Result<PathBuf, Box<dyn Error>> {
        Ok(PathBuf::from(database_path)
            .join(Path::new("libraries"))
            .join(&self.location)
            .canonicalize()?)
    }
}

#[derive(Debug, Deserialize)]
pub struct Job {
    pub(crate) project: SourceInfo,
    pub(crate) example_sketch: SourceInfo,
    pub(crate) library_info: LibraryInfo,
}
