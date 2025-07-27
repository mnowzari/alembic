use chrono::Local;

use crate::{
    sinks::base::{LogLevels, LogMessage},
    utils::{self},
};
use std::error::Error;

pub struct Handler {
    sinks: Vec<Box<dyn LogMessage>>, // Box dyn that implements the LogMessage trait
}

impl Handler {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Handler { sinks: vec![] })
    }

    pub fn add_sink(&mut self, sink: Box<dyn LogMessage>) {
        // Adds individual sinks to the handler
        self.sinks.push(sink)
    }

    pub fn set_sinks(&mut self, sinks: Vec<Box<dyn LogMessage>>) {
        // Replaces the handler's entire vector with the incoming vector
        self.sinks = sinks
    }

    fn log_to_sinks(&mut self, message: &str, log_level: LogLevels) {
        // Generate timestamp here so all sinks have the same timestamp
        let timestamp: chrono::DateTime<Local> = utils::generate_human_timestamp();

        for s in self.sinks.iter_mut() {
            s.log_message(&String::from(message), timestamp, &log_level);
        }
    }

    pub fn debug(&mut self, message: &str) {
        self.log_to_sinks(message, LogLevels::Debug);
    }

    pub fn info(&mut self, message: &str) {
        self.log_to_sinks(message, LogLevels::Info);
    }

    pub fn warn(&mut self, message: &str) {
        self.log_to_sinks(message, LogLevels::Warn);
    }

    pub fn error(&mut self, message: &str) {
        self.log_to_sinks(message, LogLevels::Error);
    }

    pub fn fatal(&mut self, message: &str) {
        self.log_to_sinks(message, LogLevels::Fatal);
    }
}
