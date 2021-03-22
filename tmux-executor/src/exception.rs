use tmux_lib::exception::TmuxCreatorException;

pub enum TmuxExecutorException {
    ParseArgument(String),
    Empty,
    ReadConfig(TmuxCreatorException),
    ListSession(TmuxCreatorException),
    ListConfigSession(TmuxCreatorException),
    NewSession(TmuxCreatorException),
    KillSession(String, TmuxCreatorException),
}

impl TmuxExecutorException {
    pub fn message(&self) -> String {
        match &self {
            TmuxExecutorException::ParseArgument(arg) => {
                format!("Failed to parse arg: {}. \n {}", arg, tmux_lib::help())
            }
            TmuxExecutorException::Empty => tmux_lib::help(),
            TmuxExecutorException::ListSession(e) => {
                format!(
                    "Failed to retrieve list session infomations. \n {}",
                    e.message()
                )
            }
            TmuxExecutorException::ListConfigSession(e) => {
                format!("Failed to read registered sessions. \n {}", e.message())
            }
            TmuxExecutorException::NewSession(e) => {
                format!("Failed to create new session. \n {}", e.message())
            }
            TmuxExecutorException::KillSession(session_name, e) => {
                format!(
                    "Failed to kill session: {}. \n {}",
                    session_name,
                    e.message()
                )
            }
            TmuxExecutorException::ReadConfig(e) => {
                format!("Failed to read config file. \n {}", e.message())
            }
        }
    }
}
