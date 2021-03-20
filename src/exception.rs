
#[derive(Debug)]
pub enum TmuxCreatorException {
    ReadConfig(String),
}

impl TmuxCreatorException {
    pub fn message(&self) -> String {
        match &self {
            TmuxCreatorException::ReadConfig(file_name) => {
                format!("Failed to config from file {}", file_name)
            }
        }
    }
}
