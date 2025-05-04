use std::error::Error;
use crate::sinks::base::{LogLevels, LogMessage};

pub struct Handler {
    sinks: Vec<Box<dyn LogMessage>>, // Box dyn that implements the LogMessage trait
}

impl Handler {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Handler { sinks: vec![] })
    }

    pub fn set_sinks(&mut self, sinks: Vec<Box<dyn LogMessage>>) -> Result<(), Box<dyn Error>> {
        Ok(self.sinks = sinks)
    }

    fn log_to_sinks(&mut self, message: String, log_level: LogLevels) {
        for s in self.sinks.iter_mut() {
            s.log_message(&message, &log_level);
        }
    }

    pub fn debug(&mut self, message: String) {
        self.log_to_sinks(message, LogLevels::DEBUG);
    }

    pub fn info(&mut self, message: String) {
        self.log_to_sinks(message, LogLevels::INFO);
    }

    pub fn warn(&mut self, message: String) {
        self.log_to_sinks(message, LogLevels::WARN);
    }

    pub fn error(&mut self, message: String) {
        self.log_to_sinks(message, LogLevels::ERROR);
    }

    pub fn fatal(&mut self, message: String) {
        self.log_to_sinks(message, LogLevels::FATAL);
    }
}