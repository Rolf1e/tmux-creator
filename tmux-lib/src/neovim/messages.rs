pub enum Message {
    Unknow(String),
    ListSessions,
}

impl From<String> for Message {
    fn from(event: String) -> Self {
        match &event[..] {
            "list-session" => Message::ListSessions,
            _ => Message::Unknow(event),
        }
    }
}

