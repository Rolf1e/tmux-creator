use neovim_lib::CallError;

pub enum NeovimException {
    UnknowMessage(String),
    ListSessions(tmux_lib::exception::TmuxCreatorException),
    RegisteredListSessions(tmux_lib::exception::TmuxCreatorException),
    SendCommandToNeovim(String, CallError),
    ReadConfig(tmux_lib::exception::TmuxCreatorException),
}

impl NeovimException {
    pub fn message(&self) -> String {
        match &self {
            NeovimException::UnknowMessage(message) => {
                format!("Received an unknow event \"{}\" from Neovim", message)
            }
            NeovimException::ListSessions(e) => {
                format!("Failed to list running tmux sessions. \n {}", e.message())
            }
            NeovimException::SendCommandToNeovim(command, e) => {
                format!("Failed to send command \"{}\" to Neovim. \n {}", command, e)
            }
            NeovimException::RegisteredListSessions(e) => {
                format!("Failed to list registered tmux sessions. \n {}", e.message())
            }
            NeovimException::ReadConfig(e) => {
                format!("Failed to read tmux configs. \n {}", e.message())
            }
        }
    }
}
