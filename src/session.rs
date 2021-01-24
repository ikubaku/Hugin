use std::error::Error;
use std::path::{Path, PathBuf};

use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct Session {
    project_path: String,
    jobs_path: String,
}

impl Session {
    pub fn get_absolute_project_path(
        &self,
        session_path: &Path,
    ) -> Result<PathBuf, Box<dyn Error>> {
        Ok(PathBuf::from(session_path)
            .join(&self.project_path)
            .canonicalize()?)
    }

    pub fn get_absolute_jobs_path(&self, session_path: &Path) -> Result<PathBuf, Box<dyn Error>> {
        Ok(PathBuf::from(session_path)
            .join(&self.jobs_path)
            .canonicalize()?)
    }
}
