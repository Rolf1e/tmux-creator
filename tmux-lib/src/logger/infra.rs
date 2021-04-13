use crate::logger::output::OutputHandler;

pub struct Logger {
    formatter: Formatter,
    output: Box<dyn OutputHandler>,
}

impl Logger {
    pub fn new(output: Box<dyn OutputHandler>) -> Self {
        Logger {
            formatter: Formatter {},
            output,
        }
    }

    pub fn log(&self, message: &str) {
        let message = self.formatter.format(LoggerLevel::Info, message);
        self.output
            .write(&message)
            .unwrap_or_else(|e| panic!("Failed to log information in output. \n {}", e.message()))
    }

    pub fn error(&self, message: &str) {
        let message = self.formatter.format(LoggerLevel::Error, message);
        self.output
            .write(&message)
            .unwrap_or_else(|e| panic!("Failed to log information in output. \n {}", e.message()))

    }
}

struct Formatter { }

#[derive(Copy, Clone)]
pub enum LoggerLevel {
    Info,
    Error,
}

impl LoggerLevel {
    pub fn header(&self) -> String {
        match self {
            LoggerLevel::Info => self.message("INFO"),
            LoggerLevel::Error => self.message("ERROR"),
        }
    }

    fn message(&self, level: &str) -> String {
        format!("{} - ", level)
    }
}

impl Formatter {
    pub fn format(&self, level: LoggerLevel, message: &str) -> String {
        format!("{}{}", level.header(), message)
    }
}

// implementation tests could be removed
#[test]
fn test_info_level() {
    let formatter = Formatter { };
    assert_eq!(
        "INFO - log at info level",
        formatter.format(LoggerLevel::Info, "log at info level")
    );
}

#[test]
fn test_error_level() {
    let formatter = Formatter { };
    assert_eq!(
        "ERROR - log at error level",
        formatter.format(LoggerLevel::Error, "log at error level")
    );
}

