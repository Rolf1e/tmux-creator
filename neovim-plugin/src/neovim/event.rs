use tmux_lib::config;

use crate::neovim::command::Command;
use crate::neovim::exception::NeovimException;

pub enum Event {
    ListSessions,
    RegisteredSessions,
    LaunchSession(String),
    KillSession(String),
}

impl Event {
    pub fn command(&self) -> Result<Command, NeovimException> {
        match &self {
            Event::ListSessions => list_session(),
            Event::RegisteredSessions => list_registered_sessions(),
            Event::LaunchSession(session_name) => launch_session(&session_name),
            Event::KillSession(session_name) => kill_session(&session_name),
        }
    }
}

fn kill_session(session_name: &str) -> Result<Command, NeovimException> {
    match tmux_lib::kill_session(session_name) {
        Ok(()) => Ok(Command::Echo(format!("Killed {} sesssion", session_name))),
        Err(e) => Err(NeovimException::KillSession(session_name.to_string(), e)),
    }
}


fn launch_session(session_name: &str) -> Result<Command, NeovimException> {
    let file_name = match config::resolve_home_dir() {
        Ok(home_dir) => format!("{}{}", home_dir, config::DEFAULT_CONFIG_FILE),
        Err(e) => return Err(NeovimException::ReadConfig(e)),
    };
    if let Err(e) = tmux_lib::create_tmux_session(session_name, &file_name) {
        Err(NeovimException::ReadConfig(e))
    } else {
        Ok(Command::Echo(format!("Launch {} session", session_name)))
    }
}

fn list_registered_sessions() -> Result<Command, NeovimException> {
    let file_name = match config::resolve_home_dir() {
        Ok(home_dir) => format!("{}{}", home_dir, config::DEFAULT_CONFIG_FILE),
        Err(e) => return Err(NeovimException::ReadConfig(e)),
    };
    match tmux_lib::list_config_session(&file_name) {
        Ok(sessions) => {
            let sessions = &sessions.join(", ");
            Ok(Command::Echo(format!(
                "Registered TMUX-Sessions: {}",
                sessions
            )))
        }
        Err(e) => Err(NeovimException::RegisteredListSessions(e)),
    }
}

fn list_session() -> Result<Command, NeovimException> {
    match tmux_lib::list_tmux_session() {
        Ok(sessions) => {
            let sessions = &sessions.join(", ");
            Ok(Command::Echo(format!("Opened TMUX-Sessions: {}", sessions)))
        }
        Err(e) => Err(NeovimException::ListSessions(e)),
    }
}
