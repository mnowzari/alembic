use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum EnvLogLevel {
    Info,
    Debug,
    Warn,
    Error,
}
pub struct LogMessage {
    pub level: EnvLogLevel,
    pub log_message: String,
    pub timestamp: String,
}