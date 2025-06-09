use chrono::Local;

use super::base::{self, LogLevels};
use core::fmt;
use std::{error::Error, fs::{self, File}, io::Write, path::PathBuf, time::{Duration, SystemTime}};

pub enum RotationPolicy {
    HOURLY,
    DAILY,
    WEEKLY,
    MONTHLY,
}
impl fmt::Display for RotationPolicy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RotationPolicy::HOURLY => write!(f, "HOURLY"),
            RotationPolicy::DAILY => write!(f, "DAILY"),
            RotationPolicy::WEEKLY => write!(f, "WEEKLy"),
            RotationPolicy::MONTHLY => write!(f, "MONTHly"),
        }
    }
}

pub struct FileSink {
    type_id: String,
    filename: PathBuf,
    rotation_policy: RotationPolicy
}

impl base::LogMessage for FileSink {
    fn log_message(&mut self, message: &String, log_levels: &base::LogLevels) {
        // Before doing anything else, let's check if logs require rotation
        self.rotate_log_file();
        // generate timestamp
        // TODO - timestamp should be generated once, not per-sink!
        let timestamp: chrono::DateTime<Local> = Local::now();
        // prepare log message
        let prepared_message: String = self.prepare_log_message(
            message,
            log_levels);
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

    fn prepare_log_message(&mut self, message: &String, log_levels: &base::LogLevels) -> String {
        let timestamp: chrono::DateTime<Local> = Local::now();
        format!(
            "[{:?}] [{}] [{}] {}\n",
            timestamp, log_levels, self.type_id, message
        )
    }

    fn append_to_file(&mut self, message: &String) {
        self.create_if_nonexistent();

        let mut openf = File::options()
            .append(true)
            .create(true)
            .open(&self.filename).unwrap();

        openf.write(message.as_bytes())
            .expect("write failed");
    }

    fn create_if_nonexistent(&mut self) {
        let does_logfile_exist = fs::exists(self.filename
            .clone())
            .unwrap();
        if does_logfile_exist == false {
            self.create_new_log_file().unwrap();
        }
    }

    fn rotate_log_file(&mut self) {
        // check if file exists
        let does_logfile_exist = fs::exists(self.filename
            .clone())
            .unwrap();

        // if it exists, determine if its age is greater than the current rotation policy
        if does_logfile_exist == true {

            let file_metadata = fs::metadata(
                self.filename.clone()
            ).unwrap();
            
            let creation_date = file_metadata.created().unwrap();
            let duration_since = SystemTime::duration_since(
                &SystemTime::now(), 
                creation_date).unwrap();

            let mut is_stale = false;
            match self.rotation_policy {
                RotationPolicy::HOURLY => {
                    if duration_since > Duration::from_secs(3600) {
                        println!("HERE");
                        is_stale = true;
                    }
                },
                RotationPolicy::DAILY => {
                    if duration_since > Duration::from_secs(86400) {
                        is_stale = true;
                    }
                },
                RotationPolicy::WEEKLY => {
                    if duration_since > Duration::from_secs(604800) {
                        is_stale = true;
                    }
                },
                RotationPolicy::MONTHLY => {
                    if duration_since > Duration::from_secs(2419200) {
                        is_stale = true;
                    }
                },
            }

            if is_stale == true {
                self.rename_existing_log_file().unwrap();
                self.create_new_log_file().unwrap();
            }

        }
        else {
            self.create_new_log_file().unwrap();
        }
    }

    fn create_new_log_file(&mut self) -> Result<File, Box<dyn Error>> {
        Ok(File::create(self.filename.clone()).unwrap())
    }

    fn rename_existing_log_file(&mut self) -> Result<(), Box<dyn Error>> {
        let timestamp: chrono::DateTime<Local> = Local::now();
        // modify to unix timestamp
        // TODO - more sophisticated string manipulation!
        let new_logfile_name = format!("alembic.{:?}.log", timestamp);
        let _ = fs::rename(
            self.filename.clone(),
            PathBuf::from(new_logfile_name));

        Ok(())
    }
}
