use tmux_lib::logger;

use crate::event::{Event, EventResponse};
use crate::exception::NeovimException;
use crate::messages::Message;

#[derive(Copy, Clone)]
pub struct EventHandler;

impl EventHandler {
    fn new() -> Self {
        EventHandler
    }

    pub fn handle_event(&self, event: String, values: Vec<nvim_rs::Value>) -> EventResponse {
        match interprete_event(event, values) {
            Ok(event) => event.execute(),
            Err(e) => EventResponse::Exception(e),
        }
    }
}

impl Default for EventHandler {
    fn default() -> Self {
        Self::new()
    }
}

fn interprete_event(event: String, values: Vec<nvim_rs::Value>) -> Result<Event, NeovimException> {
    match Message::from(event) {
        Message::Unknow(message) => Err(NeovimException::UnknowMessage(message)),
        Message::ListSessions => Ok(Event::ListSessions),
        Message::RegisteredSessions => Ok(Event::RegisteredSessions),
        Message::LaunchSession => Ok(Event::LaunchSession(extract_one_parameter(values)?)),
        Message::KillSession => Ok(Event::KillSession(extract_one_parameter(values)?)),
    }
}

fn extract_one_parameter(values: Vec<nvim_rs::Value>) -> Result<String, NeovimException> {
    let mut values = values.iter();

    let value = values.next();
    let session_name = match value {
        Some(session_name) => session_name.as_str(),
        None => {
            return Err(NeovimException::LaunchSession(String::from(
                "You need to provide a session name",
            )))
        }
    };

    match session_name {
        Some(session_name) => Ok(String::from(session_name)),
        None => {
            let message = format!("Can not parse session name from {:?}", value);
            logger::error(&message);
            Err(NeovimException::LaunchSession(message))
        }
    }
}
