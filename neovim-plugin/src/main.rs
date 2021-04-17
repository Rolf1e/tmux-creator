mod event_handler;
mod neovim_handler;
mod event;
mod messages;
mod exception;

#[tokio::main]
async fn main() {
    let (_neovim, io_handler) = neovim_handler::build_neovim().await;
    match io_handler.await {
        //TODO handle error
        Err(joinerr) => eprintln!("Error joining IO loop: '{}'", joinerr),
        Ok(Err(err)) => {}
        Ok(Ok(())) => {}
    }
    
}
