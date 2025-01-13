use std::error::Error;
use std::time::SystemTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum EnvLogLevel {
    INFO,
    DEBUG,
    WARN,
    ERROR,
}

pub struct LogMessage {
    pub level: EnvLogLevel,
    pub log_message: String,
    pub timestamp: SystemTime,
}

// LogHandler should establish several things:
// a. The log level
// b. What is the output sink?
#[derive(Serialize, Deserialize, Debug)]
pub struct LogHandler {
    log_level: EnvLogLevel,
    output_sink: String, // where to dump output
}

impl LogHandler {
    pub fn new(env_level: EnvLogLevel) -> Result<LogHandler, Box<dyn Error>> {
        Ok(LogHandler {
            log_level: env_level,
            output_sink: String::new(),
        })
    }

    pub fn log(&self, log_level: EnvLogLevel, message: String) -> Result<LogMessage, Box<dyn Error>> {
        // create and return log message object
        let new_log_message = LogMessage {
            level: log_level,
            log_message: message,
            timestamp: SystemTime::now(),
        };
        Ok(new_log_message)
    }
}


#[cfg(test)]
mod logging_tests {
    use std::{env, ffi::OsString, fs};

    use super::*;

    #[test]
    fn test_log_message_creation_all_log_levels() {
        let log_handler: LogHandler = LogHandler::new(EnvLogLevel::INFO).unwrap();

        let log_line_one = log_handler.log(EnvLogLevel::INFO, "This is an INFO log message".to_string())
            .unwrap();
        let log_line_two = log_handler.log(EnvLogLevel::DEBUG, "This is a DEBUG log message".to_string())
            .unwrap();
        let log_line_three = log_handler.log(EnvLogLevel::WARN, "This is a WARN log message".to_string())
            .unwrap();
        let log_line_four = log_handler.log(EnvLogLevel::ERROR, "This is an ERROR log message".to_string())
            .unwrap();

        assert_eq!(log_line_one.level, EnvLogLevel::INFO);
        assert_eq!(log_line_two.level, EnvLogLevel::DEBUG);
        assert_eq!(log_line_three.level, EnvLogLevel::WARN);
        assert_eq!(log_line_four.level, EnvLogLevel::ERROR);

        assert_eq!("This is an INFO log message".to_string(), log_line_one.log_message);
        assert_eq!("This is a DEBUG log message".to_string(), log_line_two.log_message);
        assert_eq!("This is a WARN log message".to_string(), log_line_three.log_message);
        assert_eq!("This is an ERROR log message".to_string(), log_line_four.log_message);
    }
}

