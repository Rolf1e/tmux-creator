extern crate neovim_lib;
use tmux_lib::neovim::command::NeovimCommandExecutor;
use tmux_lib::neovim::event;

fn main() {
    let neovim = event::create_neovim();
    let command_executor = NeovimCommandExecutor::new(neovim);
    let mut event_handler = event::EventHandler::new(Box::new(command_executor));
    event_handler
        .recv()
        .unwrap_or_else(|e| panic!("{:?}", e.message()));
}
