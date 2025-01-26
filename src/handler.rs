use serde::{Deserialize, Serialize};
use std::error::Error;

use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use crate::message::{self, LogMessage};

// LogHandler should establish several things:
// a. The log level
#[derive(Serialize, Deserialize, Debug)]
pub struct LogHandler {
    output_sink: String, // where to dump output
}

impl LogHandler {
    pub fn new() -> Result<LogHandler, Box<dyn Error>> {
        Ok(LogHandler {
            output_sink: String::new(),
        })
    }

    pub fn log(
        &self,
        level: message::EnvLogLevel,
        message: &str,
    ) -> Result<LogMessage, Box<dyn Error>> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
            .to_string();

        // create and return log message object
        Ok(LogMessage {
            level: message::EnvLogLevel::Debug,
            log_message: message.to_string(),
            timestamp,
        })
    }
}

// #[cfg(test)]
// mod logging_tests {
//     // use std::{env, ffi::OsString, fs};

//     use super::*;

//     #[test]
//     fn test_log_message_creation_all_log_levels() {
//         let log_handler: LogHandler = LogHandler::new(EnvLogLevel::Info).unwrap();

//         let log_line_one = log_handler
//             .log(EnvLogLevel::Info, "This is an Info log message".to_string())
//             .unwrap();

//         let log_line_two = log_handler
//             .log(
//                 EnvLogLevel::Debug,
//                 "This is a Debug log message".to_string(),
//             )
//             .unwrap();

//         let log_line_three = log_handler
//             .log(EnvLogLevel::Warn, "This is a Warn log message".to_string())
//             .unwrap();

//         let log_line_four = log_handler
//             .log(
//                 EnvLogLevel::Error,
//                 "This is an Error log message".to_string(),
//             )
//             .unwrap();

//         assert_eq!(log_line_one.level, EnvLogLevel::Info);
//         assert_eq!(log_line_two.level, EnvLogLevel::Debug);
//         assert_eq!(log_line_three.level, EnvLogLevel::Warn);
//         assert_eq!(log_line_four.level, EnvLogLevel::Error);

//         assert_eq!(
//             "This is an Info log message".to_string(),
//             log_line_one.log_message
//         );
//         assert_eq!(
//             "This is a Debug log message".to_string(),
//             log_line_two.log_message
//         );
//         assert_eq!(
//             "This is a Warn log message".to_string(),
//             log_line_three.log_message
//         );
//         assert_eq!(
//             "This is an Error log message".to_string(),
//             log_line_four.log_message
//         );
//     }
// }
