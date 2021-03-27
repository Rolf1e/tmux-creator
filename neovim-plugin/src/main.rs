extern crate neovim_lib;
use tmux_lib::neovim::command::{Command, CommandExecutor, NeovimCommandExecutor};
use tmux_lib::neovim::event;

fn main() {
    let neovim = event::create_neovim();
    let command_executor = NeovimCommandExecutor::new(neovim);
    let mut event_handler = event::EventHandler::new(Box::new(command_executor));

    if let Err(e) = event_handler.recv() {
        let neovim = event::create_neovim();
        let mut command_executor = NeovimCommandExecutor::new(neovim);
        let command = Command::Echo(e.message());
        command_executor
            .send_to_neovim(&command.get())
            .unwrap_or_else(|e| {
                panic!(
                    "Could not communicate with Neovim threw RPC. \n {}",
                    e.message()
                )
            })
    }
}
