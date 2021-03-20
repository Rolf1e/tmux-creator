
#[derive(Debug)]
pub enum TmuxCreatorException {
    ReadConfig(String),
    RootPathConfig,
}

impl TmuxCreatorException {
    pub fn message(&self) -> String {
        match &self {
            TmuxCreatorException::ReadConfig(file_name) => {
                format!("Failed to config from file {}", file_name)
            }
            TmuxCreatorException::RootPathConfig => format!("Failed to resolve root path"),
        }
    }
}
