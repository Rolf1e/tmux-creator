use crate::exception::TmuxCreatorException;
use std::path::Path;

pub const DEFAULT_CONFIG_FILE: &str = "/.config/tmux-creator.yml";

pub fn resolve_home_dir() -> Result<String, TmuxCreatorException> {
    if let Some(path) = home::home_dir() {
        let path = path.as_path();
        extract_path(path)
    } else {
        Err(TmuxCreatorException::RootPathConfig)
    }
}

fn extract_path(path: &Path) -> Result<String, TmuxCreatorException> {
    if let Some(path) = path.to_str() {
        Ok(path.to_string())
    } else {
        Err(TmuxCreatorException::RootPathConfig)
    }
}
