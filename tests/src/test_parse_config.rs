#[cfg(test)]
mod test_application {
    use tmux_lib;

    #[test]
    fn should_create_session_and_kill_session() {
        let session = "test-session";
        if let Err(e) = tmux_lib::create_tmux_session(session, "./config_test.yml") {
            eprintln!("{}", e.message());
            assert!(false);
        } else {
            match tmux_lib::list_tmux_session() {
                Ok(session_names) => assert!(session_names.contains(&String::from(session))),
                Err(_) => assert!(false),
            }
        }

        // kill session at this end
        if let Err(e) = tmux_lib::kill_session(session) {
            eprintln!("{}", e.message());
            assert!(false);
        } else {
            match tmux_lib::list_tmux_session() {
                Ok(session_names) => {
                    assert!(!session_names.contains(&String::from(session)));
                }
                Err(_) => assert!(false),
            }
            assert!(true);
        }
    }

    #[test]
    fn should_not_create_session() {
        if let Err(e) = tmux_lib::create_tmux_session("test-session", "./bad_config_test.yml") {
            assert_eq!("Failed to config from file ./bad_config_test.yml. \n No such file or directory (os error 2)", e.message());
        } else {
            assert!(false);
        }
    }

    #[test]
    fn should_list_config_session() {
        if let Ok(sessions) = tmux_lib::list_config_session("./config_test.yml") {
            assert_eq!(1, sessions.len());
        } else {
            assert!(false);
        }
    }

    #[test]
    fn should_not_list_config_session() {
        if let Err(e) = tmux_lib::list_config_session("./bad_config_test.yml") {
            assert_eq!("Failed to config from file ./bad_config_test.yml. \n No such file or directory (os error 2)", e.message());
        } else {
            assert!(false);
        }
    }
}
