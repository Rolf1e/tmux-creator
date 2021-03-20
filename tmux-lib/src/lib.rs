pub mod config;
mod exception;
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
