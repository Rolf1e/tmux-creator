use std::env;

pub mod exception;
pub mod parser;
pub mod session;
pub mod tmux;

const FILE_NAME: &str = "config.yml";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        let configs =
            parser::parse_file(FILE_NAME).unwrap_or_else(|e| panic!("{}", e.message()));
        let names: Vec<_> = configs
            .into_iter()
            .map(|session| session.get_name().clone())
            .collect();

        println!(
            "You must specify a tmux name session. \nSessions: {:?}",
            names
        );
        return;
    }
    let session_name = args[1].clone();
    if let Err(e) = tmux::create_tmux(session_name.as_str(), FILE_NAME) {
        println!("{}", e.message());
    }
}
