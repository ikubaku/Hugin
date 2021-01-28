use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

use log::{debug, error, info, warn};

use zip::ZipArchive;

mod parser;

use crate::clone_pair::ClonePair;
use crate::config::ccfindersw::CCFinderSWConfig;
use crate::error::RunnerProcessFailedError;
use crate::job::Job;
use crate::runner::Runner;
use crate::runner::ccfindersw::parser::ResultParser;

pub struct CCFinderSWRunner {
    project_path: PathBuf,
    database_path: PathBuf,
    config: CCFinderSWConfig,
}

impl CCFinderSWRunner {
    pub fn create(config: CCFinderSWConfig, project_path: &Path, database_path: &Path) -> Self {
        CCFinderSWRunner {
            project_path: PathBuf::from(project_path),
            database_path: PathBuf::from(database_path),
            config,
        }
    }
}

impl Runner for CCFinderSWRunner {
    fn run_job(&self, job: Job) -> Result<Vec<ClonePair>, Box<dyn Error>> {
        warn!("Some functionalities are not yet implemented!");
        let project_source_name = job.project.get_file_name()?;
        let example_source_name = job.example_sketch.get_file_name()?;
        let library_info = job.library_info;
        let library_archive_path = library_info.get_absolute_location(&self.database_path)?;

        let working_dir = tempfile::tempdir()?;
        let sources_path = working_dir.path().join("src");
        fs::create_dir(&sources_path)?;

        debug!(
            "Copying the project source file...: {}",
            job.project
                .get_location_from(&self.project_path)?
                .to_str()
                .unwrap()
        );
        fs::copy(
            job.project.get_location_from(&self.project_path)?,
            sources_path.join(project_source_name.clone()),
        )?;

        {
            let mut example_source = File::create(sources_path.join(example_source_name.clone()))?;
            debug!(
                "Opening the library archive...: {}",
                library_archive_path.to_str().unwrap()
            );
            let library_zip = File::open(library_archive_path)?;
            let mut library_archive = ZipArchive::new(library_zip)?;
            debug!(
                "Searching the source file: {}",
                job.example_sketch
                    .get_non_canonical_path_from(
                        &Path::new(library_info.archive_root.as_str()).join("examples")
                    )
                    .to_str()
                    .unwrap()
            );
            let mut file = match library_archive.by_name(
                job.example_sketch
                    .get_non_canonical_path_from(
                        &Path::new(library_info.archive_root.as_str()).join("examples"),
                    )
                    .to_str()
                    .unwrap(),
            ) {
                Ok(f) => f,
                Err(e) => {
                    error!(
                        "Could not open an example sketch source: {}",
                        job.example_sketch
                            .get_non_canonical_path_from(&Path::new(""))
                            .to_str()
                            .unwrap()
                    );
                    return Err(e.into());
                }
            };
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            write!(example_source, "{}", contents)?;

            let status = Command::new(self.config.get_executable_path_as_string())
                .current_dir(&working_dir)
                .args(&[
                    "D",
                    "-d",
                    "src",
                    "-l",
                    &self.config.language_to_option_value(),
                    "-o",
                    "result",
                    "-t",
                    &self.config.token_length_to_option_value(),
                    "-w",
                    "2",
                    "-antlr",
                    &self.config.extensions_to_option_value(),
                    "-charset",
                    "auto",
                ])
                .spawn()?
                .wait()?;
            if status.success() {
                let mut file = File::open(&working_dir.path().join("result.txt"))?;
                let mut contents = String::new();
                file.read_to_string(&mut contents)?;
                let parser = ResultParser::new();
                let (_, parse_result) = parser.parse_result::<()>(&contents)?;
                let clone_pairs = parse_result.get_clone_pairs(project_source_name.as_str(), example_source_name.as_str())?;
                debug!("pairs: {:?}", clone_pairs);
                return Ok(clone_pairs);
            } else {
                return Err(RunnerProcessFailedError::new(status.code().unwrap()).into());
            }
        }
    }
}
