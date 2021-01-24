use std::error::Error;

use crate::clone_pair::ClonePair;
use crate::job::Job;

pub mod ccfindersw;

pub trait Runner {
    fn run_job(&self, job: Job) -> Result<Vec<ClonePair>, Box<dyn Error>>;
}
