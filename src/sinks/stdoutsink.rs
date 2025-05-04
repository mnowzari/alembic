use std::error::Error;
use super::base;

pub struct StdoutSink {
    type_id: String,
}

impl base::LogMessage for StdoutSink {
    fn log_message(&mut self, message: &String, log_levels: &base::LogLevels) {
        println!("[TIMESTAMP-HERE] [{}] [STDOUT] {}", log_levels, message);
    }
}

impl StdoutSink {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(StdoutSink {
            type_id: String::from("stdout"),
        })
    }
}