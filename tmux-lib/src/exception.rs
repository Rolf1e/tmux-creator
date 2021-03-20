#[derive(Debug)]
pub enum TmuxCreatorException {
    ReadConfig(String),
    RootPathConfig,
    ExecuteChild(String),
}

impl TmuxCreatorException {
    pub fn message(&self) -> String {
        match &self {
            TmuxCreatorException::ReadConfig(file_name) => {
                format!("Failed to config from file {}", file_name)
            }
            TmuxCreatorException::RootPathConfig => format!("Failed to resolve root path"),
            TmuxCreatorException::ExecuteChild(child) => {
                format!("Failed to create child process : {}", child)
            }
        }
    }
}
