use serde_derive::Deserialize;

pub mod ccfindersw;

#[derive(Deserialize)]
pub enum CloneDetectorKind {
    CCFinderSW,
}

#[derive(Deserialize)]
pub struct Config<CDC> {
    clone_detector_kind: CloneDetectorKind,
    clone_detector_config: CDC,
}
