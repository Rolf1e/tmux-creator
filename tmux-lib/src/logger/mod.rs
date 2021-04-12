use crate::logger::infra::{Logger, LoggerLevel};
use crate::logger::output::FileLoggerOutput;

pub mod exception;
pub mod infra;
pub mod output;

const DEFAULT_LOGGER_LEVEL: LoggerLevel = LoggerLevel::Error;

// TODO replace with macros
pub fn init(file_name: String) -> Logger {
    init_with_level(DEFAULT_LOGGER_LEVEL, file_name)
}

pub fn init_with_level(level: LoggerLevel, file_name: String) -> Logger {
    Logger::new(level, Box::new(FileLoggerOutput::new(file_name)))
}

