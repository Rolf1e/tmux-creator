#[cfg(test)]
mod test_application {
    use tmux_lib;

    #[test]
    fn should_create_session() {
        if let Err(e) = tmux_lib::create_tmux_session("test-session", "./config_test.yml") {
            eprintln!("{}", e.message());
            assert!(false);
        } else {
            match tmux_lib::list_tmux_session() {
                Ok(session_names) => assert!(session_names.contains(&String::from("test-session"))),
                Err(_) => assert!(false),
            }
        }
    }

    #[test]
    fn should_not_create_session() {
        if let Err(_) = tmux_lib::create_tmux_session("test-session", "./bad_config_test.yml") {
            assert!(true);
        } else {
            assert!(false);
        }
    }
}
