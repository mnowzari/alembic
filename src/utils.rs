use std::time::{SystemTime, UNIX_EPOCH};

use chrono::Local;

pub fn generate_unix_timestamp() -> u64 {
    let epoch = SystemTime::now();
    let timestamp: u64 = epoch
        .duration_since(UNIX_EPOCH)
        .expect("We are time traveling")
        .as_secs();
    timestamp
}

pub fn generate_human_timestamp() -> chrono::DateTime<Local> {
    let timestamp: chrono::DateTime<Local> = Local::now();
    timestamp
}
