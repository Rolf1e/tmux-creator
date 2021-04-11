pub struct LoggerBuilder {
    level: LoggerLevel,
}

pub struct Logger {
    level: LoggerLevel,
}

#[derive(Copy, Clone)]
pub enum LoggerLevel {
    Info,
    Error,
    Warning,
    Debug,
}

impl LoggerLevel {
    pub fn header(&self) -> String {
        match self {
            LoggerLevel::Info => self.message("INFO"),
            LoggerLevel::Error => self.message("ERROR"),
            LoggerLevel::Warning => self.message("WARN"),
            LoggerLevel::Debug => self.message("DEBUG"),
        }
    }

    fn message(&self, level: &str) -> String {
        format!("{} - ", level)
    }
}

impl Logger {
    pub fn new(level: LoggerLevel) -> Self {
        Logger { level }
    }

    pub fn log(&self, message: &str) -> String {
        format!("{}{}", self.level.header(), message)
    }
}

impl LoggerBuilder {
    pub fn builder() -> Self {
        LoggerBuilder {
            level: LoggerLevel::Info,
        }
    }

    pub fn level(&mut self, level: LoggerLevel) -> &mut Self {
        self.level = level;
        self
    }

    pub fn build(&mut self) -> Logger {
        Logger { level: self.level }
    }
}

#[cfg(test)]
pub mod logger_test {

    use super::{Logger, LoggerLevel};

    #[test]
    fn test_info_level() {
        let logger = Logger::new(LoggerLevel::Info);
        assert_eq!("INFO - log at info level", logger.log("log at info level"));
    }

    #[test]
    fn test_error_level() {
        let logger = Logger::new(LoggerLevel::Error);
        assert_eq!(
            "ERROR - log at error level",
            logger.log("log at error level")
        );
    }

    #[test]
    fn test_warning_level() {
        let logger = Logger::new(LoggerLevel::Warning);
        assert_eq!("WARN - log at warn level", logger.log("log at warn level"));
    }

    #[test]
    fn test_debug_level() {
        let logger = Logger::new(LoggerLevel::Debug);
        assert_eq!(
            "DEBUG - log at debug level",
            logger.log("log at debug level")
        );
    }
}
