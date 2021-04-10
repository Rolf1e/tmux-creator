use neovim_lib::CallError;

pub enum NeovimException {
    UnknowMessage(String),
    ListSessions,
    SendCommandToNeovim(String, CallError),
}

impl NeovimException {
    pub fn message(&self) -> String {
        match &self {
            NeovimException::UnknowMessage(message) => {
                format!("Received an unknow event \"{}\" from Neovim", message)
            }
            NeovimException::ListSessions => String::from("Failed to list running tmux sessions"),
            NeovimException::SendCommandToNeovim(command, e) => {
                format!("Failed to send command \"{}\" to Neovim. \n {}", command, e)
            }
        }
    }
}
