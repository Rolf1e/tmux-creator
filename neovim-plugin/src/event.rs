use crate::exception::NeovimException;
use tmux_lib::config;

pub enum Event {
    ListSessions,
    RegisteredSessions,
    LaunchSession(String),
    KillSession(String),
}

pub enum EventResponse {
    Window(Vec<String>),
    Normal(String),
    Exception(NeovimException),
}

impl EventResponse {
    pub fn to_neovim(&self) -> Result<nvim_rs::Value, nvim_rs::Value> {
        match self {
            EventResponse::Normal(message) => Ok(nvim_rs::Value::from(message.clone())),
            EventResponse::Exception(e) => Err(nvim_rs::Value::from(e.message())),
            EventResponse::Window(_) => Ok(nvim_rs::Value::Nil),
        }
    }
}

impl Event {
    pub fn execute(&self) -> EventResponse {
        match &self {
            Event::ListSessions => list_session(),
            Event::RegisteredSessions => list_registered_sessions(),
            Event::LaunchSession(session_name) => launch_session(&session_name),
            Event::KillSession(session_name) => kill_session(&session_name),
        }
    }
}

fn kill_session(session_name: &str) -> EventResponse {
    match tmux_lib::kill_session(session_name) {
        Ok(()) => EventResponse::Normal(format!("Killed {} sesssion", session_name)),
        Err(e) => {
            EventResponse::Exception(NeovimException::KillSession(session_name.to_string(), e))
        }
    }
}

fn launch_session(session_name: &str) -> EventResponse {
    let file_name = match config::resolve_home_dir() {
        Ok(home_dir) => format!("{}{}", home_dir, config::DEFAULT_CONFIG_FILE),
        Err(e) => return EventResponse::Exception(NeovimException::ReadConfig(e)),
    };
    if let Err(e) = tmux_lib::create_tmux_session(session_name, &file_name) {
        EventResponse::Exception(NeovimException::ReadConfig(e))
    } else {
        EventResponse::Normal(format!("Launch {} session", session_name))
    }
}

fn list_registered_sessions() -> EventResponse {
    let file_name = match config::resolve_home_dir() {
        Ok(home_dir) => format!("{}{}", home_dir, config::DEFAULT_CONFIG_FILE),
        Err(e) => return EventResponse::Exception(NeovimException::ReadConfig(e)),
    };
    match tmux_lib::list_config_session(&file_name) {
        Ok(sessions) => EventResponse::Window(sessions),
        Err(e) => EventResponse::Exception(NeovimException::RegisteredListSessions(e)),
    }
}

fn list_session() -> EventResponse {
    match tmux_lib::list_tmux_session() {
        Ok(sessions) => EventResponse::Window(sessions),
        Err(e) => EventResponse::Exception(NeovimException::ListSessions(e)),
    }
}
