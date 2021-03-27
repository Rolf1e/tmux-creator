use crate::neovim::exception::NeovimException;
use neovim_lib::Value;
use neovim_lib::{Neovim, NeovimApi};

pub trait CommandExecutor {
    fn receive_from_neovim(&mut self) -> Vec<(String, Vec<Value>)>;

    fn send_to_neovim(&mut self, command: &str) -> Result<(), NeovimException>;
}

pub enum Command {
    Echo(String),
}

pub struct NeovimCommandExecutor {
    neovim: Neovim,
}

impl NeovimCommandExecutor {
    pub fn new(neovim: Neovim) -> Self {
        NeovimCommandExecutor { neovim }
    }
}

impl CommandExecutor for NeovimCommandExecutor {
    fn receive_from_neovim(&mut self) -> Vec<(String, Vec<Value>)> {
        self.neovim.session.start_event_loop_channel()
            .into_iter()
            .map(|(event, values)| (event, values))
            .collect()
    }

    fn send_to_neovim(&mut self, command: &str) -> Result<(), NeovimException> {
        if let Err(e) = self.neovim.command(command) {
            Err(NeovimException::SendCommandToNeovim(
                String::from(command),
                e,
            ))
        } else {
            Ok(())
        }
    }
}

impl Command {
    pub fn get(&self) -> String {
        match &self {
            Command::Echo(cmd) => format!("echo {}", cmd),
        }
    }
}
