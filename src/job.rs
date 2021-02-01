use std::error::Error;
use std::path::{Path, PathBuf};

use semver::Version;

use serde_derive::{Deserialize, Serialize};

use crate::clone_pair::ClonePair;
use crate::error::InvalidPathError;

#[derive(Clone, Debug, Serialize, Deserialize)]
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

#[derive(Clone, Debug, Serialize, Deserialize)]
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Job {
    pub(crate) project: SourceInfo,
    pub(crate) example_sketch: SourceInfo,
    pub(crate) library_info: LibraryInfo,
}

impl Job {
    pub fn create_result(&self, pairs: Vec<ClonePair>) -> JobResult {
        if pairs.is_empty() {
            JobResult {
                job: self.clone(),
                clone_pairs: None,
            }
        } else {
            JobResult {
                job: self.clone(),
                clone_pairs: Some(pairs),
            }
        }
    }
}

#[derive(Serialize)]
pub struct JobResult {
    job: Job,
    clone_pairs: Option<Vec<ClonePair>>,
}

#[derive(Serialize)]
pub struct JobResults {
    pub(crate) results: Vec<JobResult>,
}
