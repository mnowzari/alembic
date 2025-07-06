use super::base::{self};
use crate::utils;
use chrono::Local;
use core::fmt;
use std::{
    error::Error,
    fs::{self, File},
    io::Write,
    path::PathBuf,
    time::{Duration, SystemTime},
};

// Constants for file rotation policies
const HOURLY: u64 = 3600;
const DAILY: u64 = 86400;
const WEEKLY: u64 = 604800;
const MONTHLY: u64 = 2419200;

#[allow(dead_code)]
pub enum RotationPolicy {
    Hourly,
    Daily,
    Weekly,
    Monthly,
}
impl fmt::Display for RotationPolicy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RotationPolicy::Hourly => write!(f, "Hourly"),
            RotationPolicy::Daily => write!(f, "Daily"),
            RotationPolicy::Weekly => write!(f, "Weekly"),
            RotationPolicy::Monthly => write!(f, "Monthly"),
        }
    }
}

pub struct FileSink {
    type_id: String,
    filename: PathBuf,
    rotation_policy: RotationPolicy,
}

impl base::LogMessage for FileSink {
    fn log_message(
        &mut self,
        message: &str,
        timestamp: chrono::DateTime<Local>,
        log_levels: &base::LogLevels,
    ) {
        // Before doing anything else, let's check if logs require rotation
        self.rotate_log_file();
        // prepare log message
        let prepared_message: String = self.prepare_log_message(message, timestamp, log_levels);
        // append log message to file
        self.append_to_file(&prepared_message);
    }
}

impl FileSink {
    pub fn new(filename: PathBuf, rotation_policy: RotationPolicy) -> Result<Self, Box<dyn Error>> {
        Ok(FileSink {
            type_id: String::from("file"),
            filename,
            rotation_policy,
        })
    }

    pub fn set_rotation_policy(&mut self, new_policy: RotationPolicy) {
        self.rotation_policy = new_policy;
    }

    fn prepare_log_message(
        &mut self,
        message: &str,
        timestamp: chrono::DateTime<Local>,
        log_levels: &base::LogLevels,
    ) -> String {
        format!(
            "[{:?}] [{}] [{}] {}\n",
            timestamp, log_levels, self.type_id, message
        )
    }

    fn append_to_file(&mut self, message: &String) {
        let mut openf = File::options()
            .append(true)
            .create(true)
            .open(&self.filename)
            .unwrap();

        openf.write_all(message.as_bytes()).expect("write failed");
    }

    fn rotate_log_file(&mut self) {
        // check if file exists
        let does_logfile_exist = fs::exists(self.filename.clone()).unwrap();

        // if it exists, determine if its age is greater than the current rotation policy
        if does_logfile_exist {
            let file_metadata = fs::metadata(self.filename.clone()).unwrap();

            let creation_date: SystemTime = file_metadata.created().unwrap();
            let duration_since: Duration =
                SystemTime::duration_since(&SystemTime::now(), creation_date).unwrap();

            let mut is_stale = false;
            match self.rotation_policy {
                RotationPolicy::Hourly => {
                    if duration_since > Duration::from_secs(HOURLY) {
                        is_stale = true;
                    }
                }
                RotationPolicy::Daily => {
                    if duration_since > Duration::from_secs(DAILY) {
                        is_stale = true;
                    }
                }
                RotationPolicy::Weekly => {
                    if duration_since > Duration::from_secs(WEEKLY) {
                        is_stale = true;
                    }
                }
                RotationPolicy::Monthly => {
                    if duration_since > Duration::from_secs(MONTHLY) {
                        is_stale = true;
                    }
                }
            }

            if is_stale {
                self.rename_existing_log_file().unwrap();
                self.create_new_log_file().unwrap();
            }
        } else {
            self.create_new_log_file().unwrap();
        }
    }

    fn create_new_log_file(&mut self) -> Result<File, Box<dyn Error>> {
        Ok(File::create(self.filename.clone()).unwrap())
    }

    fn rename_existing_log_file(&mut self) -> Result<(), Box<dyn Error>> {
        let timestamp = utils::generate_unix_timestamp();
        // TODO - more sophisticated string manipulation!
        let new_logfile_name = format!("alembic.{:?}.log", timestamp);
        let _ = fs::rename(self.filename.clone(), PathBuf::from(new_logfile_name));

        Ok(())
    }
}
