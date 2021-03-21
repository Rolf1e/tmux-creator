use tmux_lib::exception::TmuxCreatorException;

pub enum TmuxExecutorException {
    ParseArgument(String),
    Empty,
    ListSession(TmuxCreatorException),
    ListConfigSession(TmuxCreatorException),
    NewSession(TmuxCreatorException),
}

impl TmuxExecutorException {
    pub fn message(&self) -> String {
        match &self {
            TmuxExecutorException::ParseArgument(arg) => format!("Failed to parse arg: {}", arg),
            TmuxExecutorException::Empty => String::from("TODO display help"),
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
        }
    }
}
