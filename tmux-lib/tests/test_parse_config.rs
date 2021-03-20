use tmux_lib::parser;
use tmux_lib::session::{TmuxSession, TmuxWindow};

pub const BASE_PATH: &str = "/home/rolfie/projects";
const TMUX_CREATOR: &str = "tmux-creator";

#[test]
fn test_parse_all_file() {
    let expected: Vec<TmuxSession> = vec![TmuxSession::new(
        String::from(TMUX_CREATOR),
        format!("{}/{}", BASE_PATH, TMUX_CREATOR),
        Some(String::from("nvim")),
        String::from("vim"),
        vec![
        TmuxWindow::new(
            String::from("rust"),
            String::from(TMUX_CREATOR),
            format!("{}/{}", BASE_PATH, TMUX_CREATOR),
        ),
        TmuxWindow::new(
            String::from("test"),
            String::from(TMUX_CREATOR),
            format!("{}/{}", BASE_PATH, TMUX_CREATOR),
        )
    ],
    )];

    if let Ok(actual) = parser::parse_file("config_test.yml") {
        assert_eq!(expected, actual);
    } else {
        assert!(false);
    }
}
