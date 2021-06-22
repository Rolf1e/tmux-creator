use crate::exception::TmuxExecutorException;
use std::env;
use tmux_lib::config;

pub fn parse_command() -> Result<(), TmuxExecutorException> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        return Err(TmuxExecutorException::Empty);
    }

    let file_name = match config::resolve_home_dir() {
        Ok(home_dir) => format!("{}{}", home_dir, config::DEFAULT_CONFIG_FILE),
        Err(e) => return Err(TmuxExecutorException::ReadConfig(e)),
    };

    let command = args[1].clone();
    match command.as_str() {
        "-l" => list_session(),
        "-a" => new_session(&file_name, &args),
        "-r" => list_config_session(&file_name),
        "-k" => kill_session(&args),
        "-h" => {
            help();
            Ok(())
        }
        _ => Err(TmuxExecutorException::ParseArgument(command)),
    }
}

fn list_config_session(file_name: &str) -> Result<(), TmuxExecutorException> {
    match tmux_lib::list_config_session(file_name) {
        Ok(sessions) => {
            println!("{}", sessions.join(", "));
            Ok(())
        }
        Err(e) => Err(TmuxExecutorException::ListConfigSession(e)),
    }
}

fn list_session() -> Result<(), TmuxExecutorException> {
    match tmux_lib::list_tmux_session() {
        Ok(sessions) => {
            let sessions = &sessions.join(", ");
            println!("{}", sessions);
            Ok(())
        }
        Err(e) => Err(TmuxExecutorException::ListSession(e)),
    }
}

fn new_session(file_name: &str, args: &[String]) -> Result<(), TmuxExecutorException> {
    if args.len() <= 1 {
        let configs = tmux_lib::list_config_session(file_name);
        if let Err(e) = configs {
            return Err(TmuxExecutorException::ReadConfig(e));
        }
        let configs = configs.unwrap();
        println!(
            "You must specify a tmux name session. \nSessions: {:?}",
            configs
        );
        return Ok(());
    }
    let session_name = args[2].clone();
    match tmux_lib::create_tmux_session(session_name.as_str(), file_name) {
        Ok(_) => {
            println!("Session {} opened", session_name);
            Ok(())
        },
        Err(e) => Err(TmuxExecutorException::NewSession(e)),
    }
}

fn kill_session(args: &[String]) -> Result<(), TmuxExecutorException> {
    let session_name = args[2].clone();
    if let Err(e) = tmux_lib::kill_session(session_name.as_str()) {
        Err(TmuxExecutorException::KillSession(
            session_name.to_string(),
            e,
        ))
    } else {
        println!("Session {} killed", session_name);
        Ok(())
    }
}

fn help() {
    println!("{}", tmux_lib::help())
}
