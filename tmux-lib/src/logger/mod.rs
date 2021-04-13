use crate::logger::infra::Logger;
use crate::logger::output::FileLoggerOutput;

pub mod exception;
pub mod infra;
pub mod output;


// TODO replace with macros
pub fn init(file_name: String) -> Logger {
    Logger::new(Box::new(FileLoggerOutput::new(file_name)))
}

pub fn log(message: &str) {
    let logger = init("/home/rolfie/log.txt".to_string());
    logger.log(message);
}

pub fn error(message: &str) {
    let logger = init("/home/rolfie/log.txt".to_string());
    logger.error(message);
}

