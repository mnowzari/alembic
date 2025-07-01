use std::time::{SystemTime, UNIX_EPOCH};

pub fn generate_timestamp() -> String {
    let epoch = SystemTime::now();
    let timestamp: String = epoch.duration_since(UNIX_EPOCH)
        .expect("We are time traveling")
        .as_secs()
        .to_string();
    timestamp
}