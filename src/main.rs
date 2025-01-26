mod handler;
mod message;

fn main() {
    println!("Logpile development driver");

    let logger: handler::LogHandler = handler::LogHandler::new().unwrap();
    logger.log(message::EnvLogLevel::Debug,"This is a log message");
}
