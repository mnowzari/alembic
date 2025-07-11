#[allow(dead_code, unused)]
mod handler;
mod sinks;
mod utils;

use crate::sinks::*;
use std::{error::Error, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Logpile development driver");
    let mut logger: handler::Handler = handler::Handler::new().unwrap();

    let new_stdout_sink = stdoutsink::StdoutSink::new().unwrap();
    let mut new_file_sink = filesink::FileSink::new(
        PathBuf::from("/home/mattnowzari/Documents/rust_projects/alembic/alembic.log"),
        filesink::RotationPolicy::Weekly,
    )
    .unwrap();

    new_file_sink.set_rotation_policy(filesink::RotationPolicy::Hourly);

    // logger.set_sinks(vec![new_stdout_sink, new_file_sink])?;
    logger.add_sink(Box::new(new_stdout_sink));
    logger.add_sink(Box::new(new_file_sink));

    logger.debug("println debugging is best debugging");
    logger.info("Very informational");
    logger.warn("WARNUNG");
    logger.error("! ERROR !");
    logger.fatal("FATALITY.");
    Ok(())
}
