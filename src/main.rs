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
        filesink::RotationPolicy::WEEKLY,
    )
    .unwrap();

    new_file_sink.set_rotation_policy(filesink::RotationPolicy::HOURLY);

    // logger.set_sinks(vec![new_stdout_sink, new_file_sink])?;
    logger.add_sink(Box::new(new_stdout_sink));
    logger.add_sink(Box::new(new_file_sink));

    logger.debug("prior art");
    logger.debug("huh");
    logger.info("Wow this is so cool OOP in Rust");
    logger.warn("WARNUNG WARNUNG");
    logger.error("! ERROR !");
    logger.fatal("FATALITY");
    Ok(())
}
