use log::log;

mod logging;

fn main() {
    println!("Logpile development driver");

    let logh: logging::LogHandler = logging::LogHandler::new(logging::EnvLogLevel::INFO).unwrap();
    let logline = logh.log(
        logging::EnvLogLevel::INFO, "This is a log message".to_string()).unwrap();

    println!("{}", logline.log_message.to_string());
    println!("{:?}", logline.timestamp);
}
