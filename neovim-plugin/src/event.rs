use crate::exception::NeovimException;
use crate::window_builder::Type;
use crate::window_builder::{Window, WindowBuilder, KeyMap};
use tmux_lib::config;

pub enum Event {
    ListSessions(i64, i64),
    RegisteredSessions(i64, i64),
    LaunchSession(String),
    KillSession(String),
}

#[derive(Debug)]
pub enum EventResponse {
    Window(Window),
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
            Event::ListSessions(width, height) => list_session(*width, *height),
            Event::RegisteredSessions(width, height) => list_registered_sessions(*width, *height),
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

fn list_registered_sessions(ui_width: i64, ui_height: i64) -> EventResponse {
    let file_name = match config::resolve_home_dir() {
        Ok(home_dir) => format!("{}{}", home_dir, config::DEFAULT_CONFIG_FILE),
        Err(e) => return EventResponse::Exception(NeovimException::ReadConfig(e)),
    };
    match tmux_lib::list_config_session(&file_name) {
        Ok(sessions) => {
            EventResponse::Window(create_basic_window(sessions, String::from(":LaunchSession <C-R><C-W> <CR>"), ui_width, ui_height))
        },
        Err(e) => EventResponse::Exception(NeovimException::RegisteredListSessions(e)),
    }
}

fn list_session(ui_width: i64, ui_height: i64) -> EventResponse {
    match tmux_lib::list_tmux_session() {
        Ok(sessions) => EventResponse::Window(create_basic_window(sessions, String::from(":KillSession <C-R><C-W> <CR>"), ui_width, ui_height)),
        Err(e) => EventResponse::Exception(NeovimException::ListSessions(e)),
    }
}

fn create_basic_window(sessions: Vec<String>, mapping: String, ui_width: i64, ui_height: i64) -> Window {
    let mut window_builder = WindowBuilder::default();
    window_builder.set_text(sessions);

    let opts = vec![
        (String::from("silent"), true),
        (String::from("nowait"), true),
        (String::from("noremap"), true),
    ];

    window_builder.set_key_maps(
        vec![
            KeyMap::new(String::from("n"), String::from("q"), String::from(":close<CR>"), opts.clone()),
            KeyMap::new(String::from("n"), String::from("<CR>"), mapping, opts.clone()),
        ]
    );
    window_builder.set_ui_settings(simple_ui_settings(ui_width, ui_height));
    window_builder.build()
}

fn simple_ui_settings(ui_width: i64, ui_height: i64) -> Vec<(String, Type)> {
    let width: i64 = 50;
    let height: i64 = 50;
    vec![
        ( String::from("relative"), Type::String(String::from("editor"))),
        ( String::from("width"), Type::Integer(width)),
        ( String::from("height"), Type::Integer(height)),
        ( String::from("col"), Type::Integer((ui_width / 2) - (width / 2))),
        ( String::from("row"), Type::Integer((ui_height / 2) - (height / 2))),
        ( String::from("anchor"), Type::String(String::from("NW"))),
        ( String::from("style"), Type::String(String::from("minimal"))),
        ( String::from("border"), Type::String(String::from("single"))),
    ]
}

