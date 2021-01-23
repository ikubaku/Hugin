use std::path::PathBuf;

use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct Session {
    pub(crate) project_path: PathBuf,
    pub(crate) jobs_path: PathBuf,
}
