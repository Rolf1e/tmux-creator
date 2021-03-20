use std::env;
use tmux_lib::{self, config};

mod parse;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", tmux_lib::list_tmux_session());
    match config::resolve_home_dir() {
        Ok(mut home_dir) => parse_file(&mut home_dir, &args),
        Err(e) => panic!("{}", e.message()),
    }
}


fn parse_file(home_dir: &mut String, args: &[String]) {
    home_dir.push_str(config::DEFAULT_CONFIG_FILE);
    let home_dir = home_dir.as_str();
    if args.len() == 1 {
        let configs = tmux_lib::parse_file(home_dir).unwrap_or_else(|e| panic!("{}", e.message()));
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
    if let Err(e) = tmux_lib::create_tmux_session(session_name.as_str(), home_dir) {
        println!("{}", e.message());
    }
}
