use serde::{Serialize, Deserialize};

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TmuxSession {
    name: String,                  // -s
    working_directory: String,     // -c
    enter_command: Option<String>, // -d
    window_name: String,           // -n
    windows: Vec<TmuxWindow>,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TmuxWindow {
    name: String,              // -n
    session_name: String,      // -t
    working_directory: String, // -c
}

impl TmuxWindow {
    pub fn new(name: String, session_name: String, working_directory: String) -> Self {
        TmuxWindow {
            name,
            session_name,
            working_directory,
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name 
    }

    pub fn get_session_name(&self) -> &String {
        &self.session_name 
    }

    pub fn get_working_directory(&self) -> &String {
        &self.working_directory 
    }
}

impl TmuxSession {
    pub fn new(
        name: String,                  // -s
        working_directory: String,     // -c
        enter_command: Option<String>, // -d
        window_name: String,           // -n
        windows: Vec<TmuxWindow>,
    ) -> Self {
        TmuxSession {
            name,
            working_directory,
            enter_command,
            window_name,
            windows,
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name 
    }
    
    pub fn get_working_directory(&self) -> &String {
        &self.working_directory 
    }

    pub fn get_enter_command(&self) -> &Option<String> {
        &self.enter_command 
    }

    pub fn get_window_name(&self) -> &String {
        &self.window_name 
    }

    pub fn get_windows(&self) -> &Vec<TmuxWindow> {
        &self.windows 
    }
}
