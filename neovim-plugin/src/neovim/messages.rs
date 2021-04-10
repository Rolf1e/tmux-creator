pub enum Message {
    Unknow(String),
    ListSessions,
    RegisteredSessions,
}

impl From<String> for Message {
    fn from(event: String) -> Self {
        match &event[..] {
            "list" => Message::ListSessions,
            "registered" => Message::RegisteredSessions,
            _ => Message::Unknow(event),
        }
    }
}

