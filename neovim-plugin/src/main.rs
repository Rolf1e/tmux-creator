extern crate neovim_lib;

pub mod neovim;

use crate::neovim::command::NeovimCommandExecutor;
use crate::neovim::event;
use tmux_lib::logger;

const LOG_FILE: &str = "/home/rolfie/log.txt";

fn main() {
    let neovim = event::create_neovim();
    let command_executor = NeovimCommandExecutor::new(neovim);
    let logger = logger::init(LOG_FILE.to_string());
    let mut event_handler = event::EventHandler::new(Box::new(command_executor), logger);
    event_handler
        .recv()
        .unwrap_or_else(|e| panic!("{:?}", e.message()));
}
