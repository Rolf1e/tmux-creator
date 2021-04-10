use crate::neovim::command::{Command, CommandExecutor};
use crate::neovim::exception::NeovimException;
use crate::neovim::messages::Message;
use neovim_lib::{Neovim, Session, Value};
use tmux_lib::config;
use tmux_lib;

pub fn create_neovim() -> Neovim {
    match Session::new_parent() {
        Ok(session) => Neovim::new(session),
        Err(e) => panic!("Failed to initiate Neovim session. \n {}", e),
    }
}

pub struct EventHandler {
    command_executor: Box<dyn CommandExecutor>,
}

impl EventHandler {
    pub fn new(command_executor: Box<dyn CommandExecutor>) -> Self {
        EventHandler { command_executor }
    }

    pub fn recv(&mut self) -> Result<(), NeovimException> {
        if let Err(e) = self.handle_event() {
            let command = Command::Echo(format!("Something went wrong. \n {}", e.message()));
            self.send_to_neovim(&command.get())
        } else {
            Ok(())
        }
    }

    fn handle_event(&mut self) -> Result<(), NeovimException> {
        let receiver = self.command_executor.receive_from_neovim();
        for (event, values) in receiver {
            let command = interprete_event(event, values)?;
            self.send_to_neovim(&command.get())?;
        }
        Ok(())
    }

    fn send_to_neovim(&mut self, command: &str) -> Result<(), NeovimException> {
        self.command_executor.send_to_neovim(command)
    }
}

use std::fs::OpenOptions;
use std::io::prelude::*;

pub fn write_into_file(file_name: &str, to_write: &str) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_name)
        .unwrap();

    if let Err(e) = writeln!(file, "{}", to_write) {
        eprintln!("Couldn't write to file: {}", e);
    }
}

fn interprete_event(event: String, _values: Vec<Value>) -> Result<Command, NeovimException> {
    match Message::from(event) {
        Message::Unknow(message) => Err(NeovimException::UnknowMessage(message)),
        Message::ListSessions => list_session(),
        Message::RegisteredSessions => list_registered_sessions(),
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
            Ok(Command::Echo(format!("Registered TMUX-Sessions: {}", sessions)))
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
