use crate::config::{self, *};
use crate::session::{TmuxSession, TmuxWindow};
use std::process::Command;

pub fn create_tmux(session_name: &str) {
    if let Some(session) = found_session(&config::get_all_session(), &session_name) {
        create_tmux_session(session);
    } else {
        println!("Tmux Session \"{}\" does not exist", session_name);
    }
}

fn create_tmux_session(tmux_session: &TmuxSession) {
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

fn found_session<'a>(sessions: &'a [TmuxSession], session_name: &str) -> Option<&'a TmuxSession> {
    sessions
        .iter()
        .find(|session| session.get_name() == session_name)
}
