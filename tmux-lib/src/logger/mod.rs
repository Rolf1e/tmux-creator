use self::infra::{Logger, LoggerBuilder, LoggerLevel};

pub mod infra;
pub mod output;
pub mod exception;


pub fn init() -> Logger {
    LoggerBuilder::builder()
        .level(LoggerLevel::Debug)
        .build()
}
