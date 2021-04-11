use std::io;

#[derive(Debug)]
pub enum TmuxCreatorException {
    ReadConfig(String, io::Error),
    ParseConfig(String, serde_yaml::Error),
    RootPathConfig,
    FindSession(String),
    ExecuteChild(String, io::Error),
    ReadChild(String),
}

impl TmuxCreatorException {
    pub fn message(&self) -> String {
        match &self {
            TmuxCreatorException::ReadConfig(file_name, e) => {
                format!("Failed to config from file {}. \n {}", file_name, e)
            }
            TmuxCreatorException::RootPathConfig => String::from("Failed to resolve root path"),
            TmuxCreatorException::ParseConfig(file_name, e) => {
                format!("Failed to parse config from file {}. \n {}", file_name, e)
            }
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
            TmuxCreatorException::FindSession(session_name) => {
                format!("Tmux Session \"{}\" does not exist", session_name)
            }
        }
    }
}
