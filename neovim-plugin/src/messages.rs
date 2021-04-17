pub enum Message {
    Unknow(String),
    ListSessions,
    RegisteredSessions,
    LaunchSession,
    KillSession,
}

impl From<String> for Message {
    fn from(event: String) -> Self {
        match &event[..] {
            "list" => Message::ListSessions,
            "registered" => Message::RegisteredSessions,
            "launch" => Message::LaunchSession,
            "kill" => Message::KillSession,
            _ => Message::Unknow(event),
        }
    }
}

