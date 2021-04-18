
pub enum NeovimException {
    UnknowMessage(String),
    ListSessions(tmux_lib::exception::TmuxCreatorException),
    KillSession(String, tmux_lib::exception::TmuxCreatorException),
    LaunchSession(String),
    RegisteredListSessions(tmux_lib::exception::TmuxCreatorException),
    ReadConfig(tmux_lib::exception::TmuxCreatorException),
    Convertion(String),
    Extract(String),
    WindowCreation(Box<nvim_rs::error::CallError>),
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
            NeovimException::KillSession(session_name, e) => {
                format!(
                    "Failed to kill running session {}. \n {}",
                    session_name,
                    e.message()
                )
            }
            NeovimException::RegisteredListSessions(e) => {
                format!(
                    "Failed to list registered tmux sessions. \n {}",
                    e.message()
                )
            }
            NeovimException::ReadConfig(e) => {
                format!("Failed to read tmux configs. \n {}", e.message())
            }
            NeovimException::LaunchSession(reason) => {
                format!("Failed to launch session: {}", reason)
            }
            NeovimException::Convertion(value) => {
                format!("Failed to convert neovim arguments, value: {}", value)
            }
            NeovimException::Extract(key) => {
                format!("Failed to extract neovim arguments, key: {}", key)
            }
            NeovimException::WindowCreation(e) => {
                format!("Failed to create window : {}", e)
            }
        }
    }
}
