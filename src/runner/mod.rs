use std::path::Path;
use std::error::Error;

use crate::job::Job;
use crate::clone_pair::ClonePair;

pub mod ccfindersw;

pub trait Runner {
    fn run_job(&self, job: Job) -> Result<Vec<ClonePair>, Box<dyn Error>>;
}
