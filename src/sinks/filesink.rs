use super::base;
use std::error::Error;

pub struct FileSink {
    type_id: String,
}

impl base::LogMessage for FileSink {
    fn log_message(&mut self, message: &String, log_levels: &base::LogLevels) {
        println!(
            "[TIMESTAMP-HERE] [{}] [{}] {}",
            log_levels, self.type_id, message
        );
    }
}

impl FileSink {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(FileSink {
            type_id: String::from("file"),
        })
    }
}
