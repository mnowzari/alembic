use core::fmt;

use chrono::Local;

pub enum LogLevels {
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
}
impl fmt::Display for LogLevels {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogLevels::Debug => write!(f, "DEBUG"),
            LogLevels::Info => write!(f, "INFO"),
            LogLevels::Warn => write!(f, "WARN"),
            LogLevels::Error => write!(f, "ERROR"),
            LogLevels::Fatal => write!(f, "FATAL"),
        }
    }
}

/// All sinks must implement the LogMessage trait.
pub trait LogMessage {
    fn log_message(
        &mut self,
        message: &str,
        timestamp: chrono::DateTime<Local>,
        log_levels: &LogLevels,
    );
}
