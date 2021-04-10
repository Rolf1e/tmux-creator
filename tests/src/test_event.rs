#[cfg(test)]
pub mod test_event {

    use neovim_lib::Value;
    use std::sync::mpsc;
    use tmux_lib::neovim::command;
    use tmux_lib::neovim::event;
    use tmux_lib::neovim::exception;

    pub struct TestCommandExecutorKnownMessage {}

    impl command::CommandExecutor for TestCommandExecutorKnownMessage {
        fn receive_from_neovim(&mut self) -> mpsc::Receiver<(String, Vec<Value>)> {
            let (sender, receiver) = mpsc::channel();
            sender
                .send((String::from("list-session"), Vec::new()))
                .unwrap();
            receiver
        }

        fn send_to_neovim(&mut self, _command: &str) -> Result<(), exception::NeovimException> {
            Ok(())
        }
    }

    #[test]
    fn should_interprete_neovim_event() {
        let command_executor = TestCommandExecutorKnownMessage {};
        let mut event_handler = event::EventHandler::new(Box::new(command_executor));
        if let Err(e) = event_handler.recv() {
            eprintln!("Stack trace: {}", e.message());
            assert!(false);
        } else {
            assert!(true);
        }
    }

    pub struct TestCommandExecutorUnKnownMessage {}

    impl command::CommandExecutor for TestCommandExecutorUnKnownMessage {
        fn receive_from_neovim(&mut self) -> mpsc::Receiver<(String, Vec<Value>)> {
            let (sender, receiver) = mpsc::channel();
            sender
                .send((String::from("list-session"), Vec::new()))
                .unwrap();
            receiver
        }

        fn send_to_neovim(&mut self, _command: &str) -> Result<(), exception::NeovimException> {
            Err(exception::NeovimException::UnknowMessage(String::from(
                "bad message",
            )))
        }
    }
    #[test]
    fn should_not_interprete_neovim_event() {
        let command_executor = TestCommandExecutorUnKnownMessage {};
        let mut event_handler = event::EventHandler::new(Box::new(command_executor));
        if let Err(e) = event_handler.recv() {
            assert_eq!(
                "Received an unknow event \"bad message\" from Neovim",
                e.message()
            );
        } else {
            assert!(false);
        }
    }
}
