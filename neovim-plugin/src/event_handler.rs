use crate::event::Event;
use crate::exception::NeovimException;
use crate::messages::Message;
use tmux_lib::logger;
use async_trait::async_trait;
use nvim_rs::compat::tokio::Compat;
use nvim_rs::create::tokio as create;
use nvim_rs::error;
use tokio::io;
use tokio::task;

pub async fn build_neovim() -> (
    nvim_rs::Neovim<Compat<io::Stdout>>,
    task::JoinHandle<Result<(), Box<error::LoopError>>>,
) {
    create::new_parent(EventHandler).await
}

#[derive(Clone)]
struct EventHandler;

pub struct EventResponse(Event);

#[async_trait]
impl nvim_rs::Handler for EventHandler {
    type Writer = Compat<io::Stdout>;

    async fn handle_request(
        &self,
        name: String,
        args: Vec<nvim_rs::Value>,
        _neovim: nvim_rs::Neovim<Self::Writer>,
    ) -> Result<nvim_rs::Value, nvim_rs::Value> {
        match interprete_event(name, args) {
            Ok(event) => response_to_neovim(event),
            Err(e) => Err(nvim_rs::Value::from(e.message())),
        }
    }
}


impl EventResponse {
    pub fn to_neovim(&self) -> Result<nvim_rs::Value, nvim_rs::Value> {
        let EventResponse(event) = self;
        match event.execute() {
            Ok(result) => Ok(nvim_rs::Value::from(result)),
            Err(e) => Err(nvim_rs::Value::from(e.message())),
        }
    }
}

fn response_to_neovim(event: Event) -> Result<nvim_rs::Value, nvim_rs::Value> {
    EventResponse(event).to_neovim()
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
