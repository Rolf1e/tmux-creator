use std::io::Error;

#[derive(Debug)]
pub enum TmuxCreatorException {
    ReadConfig(String),
    RootPathConfig,
    ExecuteChild(String, Error),
    ReadChild(String),
}

impl TmuxCreatorException {
    pub fn message(&self) -> String {
        match &self {
            TmuxCreatorException::ReadConfig(file_name) => {
                format!("Failed to config from file {}", file_name)
            }
            TmuxCreatorException::RootPathConfig => format!("Failed to resolve root path"),
            TmuxCreatorException::ExecuteChild(child, e) => {
                format!(
                    "Failed to create child process : {}, \n {}",
                    child,
                    e.to_string()
                )
            }
            TmuxCreatorException::ReadChild(output) => {
                format!("Failed to read child output {}", output)
            }
        }
    }
}
