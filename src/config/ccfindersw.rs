use std::path::Path;

use serde_derive::Deserialize;

#[derive(Deserialize)]
pub enum Languages {
    CPlusPlus,
}

#[derive(Deserialize)]
pub struct CCFinderSWConfig {
    executable_path: Box<Path>,
    token_length: u32,
    language: Languages,
    extensions: Vec<String>,
}
