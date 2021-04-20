#[cfg(test)]
pub mod test_event {
    use neovim_plugin::event;
    use neovim_plugin::event_handler;
    use neovim_plugin::exception;

    //TODO find a way to passe config as parameter, so I can test know events as well

    #[test]
    fn should_not_interprete_neovim_event() {
        let event_handler = event_handler::EventHandler::default();
        if let event::EventResponse::Exception(exception::NeovimException::UnknowMessage(message)) =
            event_handler.handle_event(String::from("bad-event"), Vec::new(), (212, 100))
        {
            assert_eq!(String::from("bad-event"), message);
        } else {
            assert!(false);
        }
    }
}
