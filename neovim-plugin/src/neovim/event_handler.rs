use crate::neovim::command::{Command, CommandExecutor};
use crate::neovim::event::Event;
use crate::neovim::exception::NeovimException;
use crate::neovim::messages::Message;
use neovim_lib::{Neovim, Session, Value};
use tmux_lib::logger;

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
            logger::error(&e.message());
            self.send_to_neovim(Command::Error(e))
        } else {
            Ok(())
        }
    }

    fn handle_event(&mut self) -> Result<(), NeovimException> {
        let receiver = self.command_executor.receive_from_neovim();
        for (event, values) in receiver {
            let event = interprete_event(event, values)?;
            let command = event.command()?;
            self.send_to_neovim(command)?;
        }
        Ok(())
    }

    fn send_to_neovim(&mut self, command: Command) -> Result<(), NeovimException> {
        self.command_executor.send_to_neovim(&command.get())
    }
}

fn interprete_event(event: String, values: Vec<Value>) -> Result<Event, NeovimException> {
    match Message::from(event) {
        Message::Unknow(message) => Err(NeovimException::UnknowMessage(message)),
        Message::ListSessions => Ok(Event::ListSessions),
        Message::RegisteredSessions => Ok(Event::RegisteredSessions),
        Message::LaunchSession => Ok(Event::LaunchSession(extract_one_parameter(values)?)),
        Message::KillSession => Ok(Event::KillSession(extract_one_parameter(values)?)),
    }
}

fn extract_one_parameter(values: Vec<Value>) -> Result<String, NeovimException> {
    let mut values = values.iter();

    let value = values.next();
    let session_name = if value.is_none() {
        return Err(NeovimException::LaunchSession(String::from(
            "You need to provide a session name",
        )));
    } else {
        value.unwrap().as_str()
    };
    if session_name.is_none() {
        let message = format!("Can not parse session name from {:?}", value);
        logger::error(&message);
        Err(NeovimException::LaunchSession(message))
    } else {
        let session_name = session_name.unwrap();
        Ok(String::from(session_name))
    }
}
