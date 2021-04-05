pub enum Message {
    Unknow(String),
    ListSessions,
}

impl From<String> for Message {
    fn from(event: String) -> Self {
        match &event[..] {
            "list" => Message::ListSessions,
            _ => Message::Unknow(event),
        }
    }
}

