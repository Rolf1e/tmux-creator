pub enum Message {
    Unknow(String),
}

impl From<String> for Message {
    fn from(event: String) -> Self {
        match &event[..] {
            _ => Message::Unknow(event),
        }
    }
}

