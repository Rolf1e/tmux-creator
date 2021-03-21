pub mod config;
pub mod exception;
mod executor;
mod parser;
mod session;

pub fn parse_file(
    file_name: &str,
) -> Result<Vec<session::TmuxSession>, exception::TmuxCreatorException> {
    parser::parse_file(file_name)
}

pub fn create_tmux_session(
    session_name: &str,
    file_name: &str,
) -> Result<(), exception::TmuxCreatorException> {
    executor::create_tmux_session(session_name, file_name)
}

pub fn list_tmux_session() -> Result<Vec<String>, exception::TmuxCreatorException> {
    executor::list_session()
}

pub fn list_config_session(
    file_name: &str,
) -> Result<Vec<String>, exception::TmuxCreatorException> {
    let sessions = parser::parse_file(file_name)?;
    Ok(sessions
        .iter()
        .map(|session| String::from(session.get_name()))
        .collect())
}

pub fn kill_session(session_name: &str) -> Result<(), exception::TmuxCreatorException> {
    executor::kill_session(session_name)
}

pub fn help() -> String {
    String::from("
    tmcr [command [args]]

    command [args]:
    -l : list loaded session. 
    -r : list all sessions available in config.
    -a {name}: load config from {name}.
    -k {name}: kill tmux session from {name}.
        ")
}
