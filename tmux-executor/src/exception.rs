use tmux_lib::exception::TmuxCreatorException;

pub enum TmuxExecutorException {
    ParseArgument(String),
    Empty,
    ListSession,
    ListConfigSession(TmuxCreatorException),
}

impl TmuxExecutorException {
    pub fn message(&self) -> String {
        match &self {
            TmuxExecutorException::ParseArgument(arg) => format!("Failed to parse arg: {}", arg),
            TmuxExecutorException::Empty => String::from("TODO display help"),
            TmuxExecutorException::ListSession => {
                String::from("Failed to retrieve list session infomations")
            }
            TmuxExecutorException::ListConfigSession(e) => {
                format!("Failed to read registered sessions. \n {}", e.message())
            }
        }
    }
}
