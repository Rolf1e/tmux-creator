pub enum Message {
    Unknow(String),
    ListSessions,
    Hello,
}

impl From<String> for Message {
    fn from(event: String) -> Self {
        match &event[..] {
            "hello" => Message::Hello,
            "list" => Message::ListSessions,
            _ => Message::Unknow(event),
        }
    }
}

