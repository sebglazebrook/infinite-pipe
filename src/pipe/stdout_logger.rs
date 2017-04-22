pub struct StdoutLogger;

impl LoggerLike for StdoutLogger {

    fn log(&self, content: String) {
        println!("{}", content);
    }

}

#[derive(Mock)]
pub trait LoggerLike {
    fn log(&self, content: String);
}
