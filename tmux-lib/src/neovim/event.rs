use crate::neovim::command::{Command, CommandExecutor};
use crate::neovim::exception::NeovimException;
use crate::neovim::messages::Message;
use neovim_lib::{Neovim, Session, Value};

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

fn interprete_event(event: String, _values: Vec<Value>) -> Result<Command, NeovimException> {
    match Message::from(event) {
        Message::Unknow(message) => Err(NeovimException::UnknowMessage(message)),
        Message::ListSessions => list_session(),
    }
}

fn list_session() -> Result<Command, NeovimException> {
    match crate::list_tmux_session() {
        Ok(sessions) => {
            let sessions = &sessions.join(", ");
            Ok(Command::Echo(sessions.clone()))
        }
        Err(_) => Err(NeovimException::ListSessions),
    }
}
