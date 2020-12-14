use simple_logger::SimpleLogger;

fn main() {
    // Initialize the logger and test it
    SimpleLogger::new().init().unwrap();
    riseria::init();
}
