#[allow(dead_code, unused)]
mod handler;
mod sinks;

use crate::sinks::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Logpile development driver");
    let mut logger: handler::Handler = handler::Handler::new().unwrap();

    let new_stdout_sink = Box::new(stdoutsink::StdoutSink::new().unwrap());
    let new_file_sink = Box::new(filesink::FileSink::new().unwrap());

    logger.set_sinks(vec![new_stdout_sink, new_file_sink])?;

    logger.debug("prior art");
    logger.debug("huh");
    logger.info("Wow this is so cool OOP in Rust");
    logger.warn("WARNUNG WARNUNG");
    logger.error("! ERROR !");
    logger.fatal("FATALITY");
    Ok(())
}
