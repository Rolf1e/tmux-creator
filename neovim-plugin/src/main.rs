extern crate neovim_lib;

pub mod neovim;

use crate::neovim::command::NeovimCommandExecutor;
use crate::neovim::event_handler;


fn main() {
    let neovim = event_handler::create_neovim();
    let command_executor = NeovimCommandExecutor::new(neovim);
    let mut event_handler = event_handler::EventHandler::new(Box::new(command_executor));
    event_handler
        .recv()
        .unwrap_or_else(|e| panic!("{:?}", e.message()));
}
