use chrono::Local;

use crate::{
    sinks::base::{LogLevels, LogMessage},
    utils::{self},
};
use std::{collections::HashMap, error::Error};

pub struct Handler {
    sinks: Vec<Box<dyn LogMessage>>, // Box dyn that implements the LogMessage trait
    log_level: u8, // the level at which we should emit logs. Default level is Error
    level_map: HashMap<LogLevels, u8>, // a mapping of log levels to u8 values for comparison
}

impl Handler {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Handler {
            sinks: vec![],
            log_level: 3, // default log level is Error
            level_map: map_log_levels(),
        })
    }

    pub fn add_sink(&mut self, sink: Box<dyn LogMessage>) {
        // Adds individual sinks to the handler
        self.sinks.push(sink)
    }

    pub fn set_sinks(&mut self, sinks: Vec<Box<dyn LogMessage>>) {
        // Replaces the handler's entire vector with the incoming vector
        self.sinks = sinks
    }

    pub fn set_log_level(&mut self, new_log_level: LogLevels) {
        self.log_level = *self.level_map.get(&new_log_level).unwrap();
    }

    fn log_to_sinks(&mut self, message: &str, log_level: LogLevels) {
        // Generate timestamp here so all sinks have the same timestamp
        let timestamp: chrono::DateTime<Local> = utils::generate_human_timestamp();

        let message_log_level: u8 = *self.level_map.get(&log_level).unwrap();

        // only emit if the message level is greater than or equal to the handler's log level
        if message_log_level >= self.log_level {
            for s in self.sinks.iter_mut() {
                s.log_message(&String::from(message), timestamp, &log_level);
            }
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

fn map_log_levels() -> HashMap<LogLevels, u8> {
    let mut level_map: HashMap<LogLevels, u8> = HashMap::new();
    level_map.insert(LogLevels::Debug, 0);
    level_map.insert(LogLevels::Info, 1);
    level_map.insert(LogLevels::Warn, 2);
    level_map.insert(LogLevels::Error, 3);
    level_map.insert(LogLevels::Fatal, 4);
    level_map
}
