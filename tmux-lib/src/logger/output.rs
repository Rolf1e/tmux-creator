use crate::logger::exception::LoggerException;
use std::fs::OpenOptions;
use std::io::prelude::*;

pub trait OutputHandler {
    fn write(&self, message: &str) -> Result<(), LoggerException>;
}

pub struct FileLoggerOutput {
    file_name: String,
}

impl OutputHandler for FileLoggerOutput {
    fn write(&self, message: &str) -> Result<(), LoggerException> {
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&self.file_name)
            .unwrap();

        if let Err(e) = writeln!(file, "{}", message) {
            Err(LoggerException::WriteIntoFile(self.file_name.clone(), e))
        } else {
            Ok(())
        }
    }
}

impl FileLoggerOutput {
    pub fn new(file_name: String) -> Self {
        FileLoggerOutput { file_name } 
    }
}
