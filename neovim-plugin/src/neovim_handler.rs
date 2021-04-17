use crate::event::EventResponse;
use crate::event_handler::EventHandler;
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
    create::new_parent(NeovimHandler::new()).await
}

#[derive(Clone)]
struct NeovimHandler {
    event_handler: EventHandler,
}

impl NeovimHandler {
    fn new() -> Self {
        NeovimHandler {
            event_handler: EventHandler::new(),
        }
    }
}

#[async_trait]
impl nvim_rs::Handler for NeovimHandler {
    type Writer = Compat<io::Stdout>;

    async fn handle_request(
        &self,
        name: String,
        args: Vec<nvim_rs::Value>,
        _neovim: nvim_rs::Neovim<Self::Writer>,
    ) -> Result<nvim_rs::Value, nvim_rs::Value> {
        let event = self.event_handler.handle_event(name, args);
        response_to_neovim(event)
    }
}

fn response_to_neovim(event_response: EventResponse) -> Result<nvim_rs::Value, nvim_rs::Value> {
    event_response.to_neovim()
}
