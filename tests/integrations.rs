use std::{error::Error, fs, path::PathBuf};

use alembic_log::{
    handler::Handler,
    sinks::{
        base::LogLevels, filesink::{self, FileSink, RotationPolicy}, stderr::StderrSink, stdoutsink::StdoutSink
    },
};

const FILESINK_FIXTURE: &str = "./alembic_test_fixture.log";

fn log_sender(logger: &mut Handler) -> Result<bool, Box<dyn Error>> {
    logger.debug("Alembic logger is best logger for debugging");
    logger.info("Very informational");
    logger.warn("WARNING");
    logger.error("! ERROR !");
    logger.fatal("FATALITY.");

    Ok(true)
}

fn validate_log_lines() -> Result<bool, bool> {
    let log_contents: Vec<u8> = fs::read(FILESINK_FIXTURE).unwrap();
    // Naive way of checking the contents wrote correctly, accounts for Windows and Linux EoF behavior
    match log_contents.len() >= 256 && log_contents.len() <= 260 {
        true => Ok(true),
        false => Err(false),
    }
}

fn check_if_logfile_exists() -> Result<bool, bool> {
    let does_logfile_exist: bool = fs::exists(FILESINK_FIXTURE).unwrap();
    match does_logfile_exist {
        true => Ok(true),
        false => Err(false),
    }
}

#[test]
fn stdout_sink_test() {
    let mut logger: Handler = Handler::new().unwrap();
    let new_std_out_sink: StdoutSink = StdoutSink::new().unwrap();

    logger.add_sink(Box::new(new_std_out_sink));

    assert_eq!(log_sender(&mut logger).unwrap(), true);
}

#[test]
fn stderr_sink_test() {
    let mut logger: Handler = Handler::new().unwrap();
    let new_std_err_sink: StderrSink = StderrSink::new().unwrap();

    logger.add_sink(Box::new(new_std_err_sink));

    assert_eq!(log_sender(&mut logger).unwrap(), true);
}

#[test]
fn filesink_test() {
    let mut logger: Handler = Handler::new().unwrap();
    logger.set_log_level(LogLevels::Info);

    let mut new_file_sink: FileSink = FileSink::new(
        PathBuf::from("./alembic_test_fixture.log"),
        filesink::RotationPolicy::Weekly,
    )
    .unwrap();

    let test_rotation_policy: RotationPolicy = RotationPolicy::Hourly;
    new_file_sink.set_rotation_policy(test_rotation_policy);

    logger.add_sink(Box::new(new_file_sink));

    assert_eq!(log_sender(&mut logger).unwrap(), true);
    assert_eq!(check_if_logfile_exists().unwrap(), true);
    assert_eq!(validate_log_lines().unwrap(), true);

    let _ = fs::remove_file(FILESINK_FIXTURE); // delete fixture
}
