
mod event_handler;
mod event;
mod messages;
mod exception;

#[tokio::main]
async fn main() {
    let (neovim, io_handler) = event_handler::build_neovim().await;
    match io_handler.await {
        Err(joinerr) => eprintln!("Error joining IO loop: '{}'", joinerr),
        Ok(Err(err)) => {}
        Ok(Ok(())) => {}
    }
    
}
