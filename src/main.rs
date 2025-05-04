mod handler;
mod message;
mod sinks;

use std::error::Error;
use crate::sinks::*;

fn main() -> Result<(), Box<dyn Error>>{
    println!("Logpile development driver");
    let mut logger: handler::Handler = handler::Handler::new().unwrap();

    let new_stdout_sink = Box::new(stdoutsink::StdoutSink::new().unwrap());
    let new_file_sink = Box::new(filesink::FileSink::new().unwrap());

    logger.set_sinks(vec![new_stdout_sink, new_file_sink])?;

    logger.debug(String::from("prior art"));
    logger.info(String::from("Wow this is so cool OOP in Rust"));
    logger.warn(String::from("WARNUNG WARNUNG"));
    logger.error(String::from("! ERROR !"));
    logger.fatal(String::from("FATALITY"));
    Ok(())
}
