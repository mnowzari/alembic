use core::fmt;

pub enum LogLevels {
    DEBUG,
    INFO,
    WARN,
    ERROR,
    FATAL,
}
impl fmt::Display for LogLevels {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogLevels::DEBUG => write!(f, "DEBUG"),
            LogLevels::INFO => write!(f, "INFO"),
            LogLevels::WARN => write!(f, "WARN"),
            LogLevels::ERROR => write!(f, "ERROR"),
            LogLevels::FATAL => write!(f, "FATAL"),
        }
    }
}

/// All sinks must implement the LogMessage trait.
pub trait LogMessage {
    fn log_message(&mut self, message: &String, log_levels: &LogLevels);
}
