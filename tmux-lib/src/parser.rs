use crate::exception::TmuxCreatorException;
use crate::session::TmuxSession;
use std::fs;

pub fn parse_file(file_name: &str) -> Result<Vec<TmuxSession>, TmuxCreatorException> {
    let content = read_file(file_name)?;
    let parsed_content = serde_yaml::from_str(content.as_str());
    if let Err(e) = parsed_content {
        Err(TmuxCreatorException::ParseConfig(
            String::from(file_name),
            e,
        ))
    } else {
        Ok(parsed_content.unwrap())
    }
}

fn read_file(file_name: &str) -> Result<String, TmuxCreatorException> {
    match fs::read_to_string(file_name) {
        Ok(content) => Ok(content),
        Err(e) => Err(TmuxCreatorException::ReadConfig(file_name.to_string(), e)),
    }
}

pub fn parse_list_session_output(data: String) -> Vec<String> {
    let data: Vec<_> = data.split("\n").collect();
    data.iter()
        .map(|session| {
            let row: Vec<_> = session.split(":").collect();
            String::from(row[0])
        })
        .collect()
}

