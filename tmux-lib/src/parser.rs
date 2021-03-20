use crate::exception::TmuxCreatorException;
use crate::session::TmuxSession;
use std::fs;

pub fn parse_file(file_name: &str) -> Result<Vec<TmuxSession>, TmuxCreatorException> {
    let content = read_file(file_name);
    let parsed_content = serde_yaml::from_str(content.as_str());
    if let Err(e) = parsed_content {
        eprintln!("{}", e.to_string());
        Err(TmuxCreatorException::ReadConfig(String::from(file_name)))
    } else {
        Ok(parsed_content.unwrap())
    }
}

fn read_file(file_name: &str) -> String {
    fs::read_to_string(file_name)
        .unwrap_or_else(|e| panic!("Failed to read config from file {} \n {}", file_name, e))
}

