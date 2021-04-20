#[derive(Debug)]
pub struct Window {
    text: Vec<String>,
    key_maps: Vec<KeyMap>,
    ui_settings: Vec<(String, Type)>,
}

#[derive(Debug)]
pub struct KeyMap {
    mode: String,
    mapping: String,
    command: String,
    opts: Vec<(String, bool)>,
}

pub struct WindowBuilder {
    text: Vec<String>,
    key_maps: Vec<KeyMap>,
    ui_settings: Vec<(String, Type)>,
}

#[derive(Debug)]
pub enum Type {
    Integer(i64),
    String(String),
}

impl Window {
    pub fn get_text(&self) -> &Vec<String> {
        &self.text
    }

    pub fn get_key_maps(&self) -> &Vec<KeyMap> {
        &self.key_maps
    }

    pub fn get_ui_settings(&self) -> &Vec<(String, Type)> {
        &self.ui_settings
    }
}

impl KeyMap {
    pub fn get_mode(&self) -> &String {
        &self.mode
    }

    pub fn get_mapping(&self) -> &String {
        &self.mapping
    }

    pub fn get_command(&self) -> &String {
        &self.command
    }

    pub fn get_opts(&self) -> &Vec<(String, bool)> {
        &self.opts
    }
}

impl WindowBuilder {
    fn new() -> Self {
        WindowBuilder {
            text: Vec::new(),
            key_maps: Vec::new(),
            ui_settings: Vec::new(),
        }
    }

    pub fn build(self) -> Window {
        Window {
            text: self.text,
            key_maps: self.key_maps,
            ui_settings: self.ui_settings,
        }
    }

    pub fn set_text(&mut self, text: Vec<String>) {
        self.text = text;
    }

    pub fn set_key_maps(&mut self, key_maps: Vec<KeyMap>) {
        self.key_maps = key_maps;
    }

    pub fn set_ui_settings(&mut self, ui_settings: Vec<(String, Type)>) {
        self.ui_settings = ui_settings;
    }
}

impl Default for WindowBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl KeyMap {
    pub fn new(mode: String, mapping: String, command: String, opts: Vec<(String, bool)>) -> Self {
        KeyMap {
            mode,
            mapping,
            command,
            opts,
        }
    }
}
