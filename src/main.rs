use std::error::Error;

use clap::clap_app;

use log::{debug, info, trace, warn, error};
use flexi_logger::{Logger, LogSpecification, LevelFilter};

fn main() -> Result<(), Box<dyn Error>> {
    // Parse options
    let matches = clap_app!(Hugin =>
        (version: "0.1.0")
        (author: "ikubaku <hide4d51@gmail.com")
        (about: "An Arduino Project code cloning detector: Job dispatcher module")
    ).get_matches();

    // Initialize logger
    Logger::with(
        LogSpecification::default(LevelFilter::Info)
            .build()
    );

    println!("Hello, world!");

    Ok(())
}
