use std::io::Error;

pub enum LoggerException {
    WriteIntoFile(String, Error),
}

impl LoggerException {
    pub fn message(&self) -> String {
        match self {
            LoggerException::WriteIntoFile(file_name, e) => {
                format!("Failed to log into file: {}. \n {}", file_name, e)
            }
        }
    }
}
