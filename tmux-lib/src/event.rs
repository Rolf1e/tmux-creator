use neovim_lib::{Neovim, Session, Value};
use crate::messages::Message;

pub struct EventHandler {
    neovim: Neovim,
}

impl EventHandler {
    pub fn new(neovim: Neovim) -> Self {
        EventHandler { neovim }
    }

    pub fn recv(&mut self) {
        let receiver = self.neovim.session.start_event_loop_channel();
        for (event, values) in receiver {
            interprete_event(event, values)
        }
    }
}

pub fn create_neovim() -> Neovim {
    match Session::new_parent() {
        Ok(session) => Neovim::new(session),
        Err(e) => panic!("Failed to initiate Neovim session. \n {}", e),
    }
}

fn interprete_event(event: String, values: Vec<Value>) {
    todo!()
}

