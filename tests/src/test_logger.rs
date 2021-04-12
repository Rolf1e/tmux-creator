#[cfg(test)]
pub mod test_event {
    use std::fs;
    use tmux_lib::logger;
    const LOG_TEST_FILE: &str = "./test_log.txt";

    fn before_test() {
        fs::remove_file(LOG_TEST_FILE).unwrap_or_else(|e| panic!("{}", e));
    }

    #[test]
    fn should_test_logger() {
        before_test();
        let logger = logger::init(LOG_TEST_FILE.to_string());
        logger.log("This is a test log");
    }
}
