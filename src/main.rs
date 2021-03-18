use std::env;

pub mod session;
pub mod tmux;
pub mod config;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        
        let names: Vec<_> = config::get_all_session()
            .into_iter()
            .map(|session| session.get_name().clone())
            .collect();

        println!("You must specify a tmux name session. \nSessions: {:?}", names);


        return;
    }

    let session_name = args[1].clone();
    tmux::create_tmux(session_name.as_str());
}

