use chrono::Local;

use super::base;
use std::error::Error;

pub struct StderrSink {
    type_id: String,
}

impl base::LogMessage for StderrSink {
    fn log_message(
        &mut self,
        message: &str,
        timestamp: chrono::DateTime<Local>,
        log_levels: &base::LogLevels,
    ) {
        eprintln!(
            "[{:?}] [{}] [{}] {}",
            timestamp, log_levels, self.type_id, message
        );
    }
}

impl StderrSink {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(StderrSink {
            type_id: String::from("stderr"),
        })
    }
}
