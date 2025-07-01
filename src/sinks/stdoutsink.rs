use chrono::Local;

use super::base;
use std::error::Error;

pub struct StdoutSink {
    type_id: String,
}

impl base::LogMessage for StdoutSink {
    fn log_message(
        &mut self,
        message: &str,
        timestamp: chrono::DateTime<Local>,
        log_levels: &base::LogLevels,
    ) {
        println!(
            "[{:?}] [{}] [{}] {}",
            timestamp, log_levels, self.type_id, message
        );
    }
}

impl StdoutSink {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(StdoutSink {
            type_id: String::from("stdout"),
        })
    }
}
