use std::error::Error;
use std::fs::File;
use std::path::PathBuf;

use clap::clap_app;

use log::{debug, info, trace, warn, error};
use flexi_logger::{Logger, LogSpecification, LevelFilter, LogSpecBuilder, Duplicate};

mod config;
mod error;
mod session;
mod job;

use config::Config;
use std::io::Read;
use crate::config::ccfindersw::CCFinderSWConfig;
use crate::error::NoValidConfigurationError;
use crate::session::Session;
use crate::job::Job;

fn main() -> Result<(), Box<dyn Error>> {
    // Parse options
    let matches = clap_app!(Hugin =>
        (version: "0.1.0")
        (author: "ikubaku <hide4d51@gmail.com")
        (about: "An Arduino Project code cloning detector: Job dispatcher module")
        (@arg CONFIG: -c --config +takes_value "configuration filename")
        (@arg LOG: -l --log "enable logging to file")
        (@arg verbose: -v --verbose ... "verbosity of the logging (max stack: 2)")
        (@arg no_warning: -q --no_warn "suppress warning message (note that verbosity option overrides this)")
        (@arg SESSION: +required "the Hugin session generated by Munin")
    ).get_matches();

    // Initialize logger
    let mut log_spec_builder = LogSpecBuilder::new();
    log_spec_builder.default(LevelFilter::Warn);
    log_spec_builder.insert_modules_from(
        LogSpecification::env()
            .unwrap_or_else(|e| panic!("Something went wrong while parsing RUST_LOG environmental variable: {:?}", e))
    );
    if matches.is_present("no_warning") {
        log_spec_builder.default(LevelFilter::Error);
    }
    match matches.occurrences_of("verbose") {
        0 => {},
        1 => {log_spec_builder.default(LevelFilter::Info); ()},
        2 => {log_spec_builder.default(LevelFilter::Debug); ()},
        _ => panic!("Invalid verbosity was specified(maybe too much switches?)."),
    };
    let log_spec = log_spec_builder.build();
    let logger = Logger::with(log_spec).duplicate_to_stderr(Duplicate::Error);
    let logger = if matches.is_present("LOG") {
        println!("Enabled logging to the log file.");
        logger.log_to_file()
    } else {
        logger
    };
    logger.start()?;

    info!("Started the logger.");

    // Load configuration
    let config: Config;
    if let Some(filename) = matches.value_of("CONFIG") {
        info!("Loading configuration from file: {}...", filename);
        let mut file = File::open(filename)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        config = toml::from_str(contents.as_str())?;
        println!("Munin database directory: {}", config.munin_database_root.to_str().unwrap());
    } else {
        info!("Using the default configuration.")
    }

    // Load session
    let session: Session;
    {
        info!("Loading session...");
        let mut filename = PathBuf::new();
        filename.push(matches.value_of("SESSION").unwrap());
        filename.push("session.toml");
        let mut file = File::open(filename)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        session = toml::from_str(contents.as_str())?;
        info!("The project path is: {}", session.project_path.to_str().unwrap());
        info!("The jobs path is: {}", session.jobs_path.to_str().unwrap());
    }

    // Load jobs
    let mut jobs = Vec::new();
    for job_file in session.jobs_path.read_dir()? {
        debug!("job_file: {:?}", job_file);
        let path = session.jobs_path.join(job_file?.path());
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let job: Job = toml::from_str(contents.as_str())?;
        println!("Found job: {:?}", job);
        jobs.push(job);
    }

    println!("Hello, world!");
    info!("Exiting...");

    Ok(())
}
