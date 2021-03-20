use crate::exception::TmuxCreatorException;
use crate::parser;
use crate::session::{TmuxSession, TmuxWindow};
use std::process::{Command, Output};

const TMUX: &str = "tmux";
const NEW_SESSION: &str = "new-session";
const NEW_WINDOW: &str = "new-window";
const LIST_SESSION: &str = "list-session";

pub fn create_tmux_session(
    session_name: &str,
    file_name: &str,
) -> Result<(), TmuxCreatorException> {
    let config_sessions =
        parser::parse_file(file_name).unwrap_or_else(|e| panic!("{}", e.message()));
    if let Some(session) = find_session(&config_sessions, &session_name) {
        Ok(create_tmux_session_inner(session))
    } else {
        Err(TmuxCreatorException::ReadConfig(format!(
            "Tmux Session \"{}\" does not exist",
            session_name
        )))
    }
}

pub fn list_session() -> Result<Vec<String>, TmuxCreatorException> {
    let mut command = Command::new(TMUX);
    command.arg(LIST_SESSION);
    if let Ok(output) = command.output() {
        Ok(read_list_session_child_output(output)?)
    } else {
        Err(TmuxCreatorException::ExecuteChild(LIST_SESSION.to_string()))
    }
}

fn read_list_session_child_output(output: Output) -> Result<Vec<String>, TmuxCreatorException> {
    if output.status.success() {
        let data = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(parser::parse_list_session_output(data))
    } else {
        Err(TmuxCreatorException::ExecuteChild(LIST_SESSION.to_string()))
    }
}

fn create_tmux_session_inner(tmux_session: &TmuxSession) {
    let mut command = Command::new(TMUX);
    command
        .arg(NEW_SESSION)
        .arg("-s")
        .arg(tmux_session.get_name().as_str())
        .arg("-c")
        .arg(tmux_session.get_working_directory().as_str())
        .arg("-n")
        .arg(tmux_session.get_window_name().as_str());
    if let Some(cmd) = &tmux_session.get_enter_command() {
        command.arg("-d").arg(cmd.as_str());
    }
    if let Err(_) = command.spawn() {
        println!(
            "Failed to create tmux session \"{}\"",
            tmux_session.get_name()
        );
        return;
    }
    for window in tmux_session.get_windows() {
        let mut command = Command::new(TMUX);
        create_tmux_window(&mut command, window);
        if let Err(_) = command.spawn() {
            println!("Failed to create tmux window \"{}\"", window.get_name());
        }
    }
}

fn create_tmux_window(command: &mut Command, tmux_window: &TmuxWindow) {
    command
        .arg(NEW_WINDOW)
        .arg("-t")
        .arg(format!("{}:", tmux_window.get_session_name().as_str()))
        .arg("-n")
        .arg(tmux_window.get_name().as_str())
        .arg("-c")
        .arg(tmux_window.get_working_directory().as_str());
}

fn find_session<'a>(sessions: &'a [TmuxSession], session_name: &str) -> Option<&'a TmuxSession> {
    sessions
        .iter()
        .find(|session| session.get_name() == session_name)
}

