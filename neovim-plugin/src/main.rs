extern crate neovim_lib; 

mod event;
mod messages;

fn main() {
    let neovim = event::create_neovim();
    let mut event_handler = event::EventHandler::new(neovim);
    event_handler.recv();
}
