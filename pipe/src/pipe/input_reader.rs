#[derive(Clone)]
pub struct InputReader;

impl InputReader {

    pub fn new() -> Self {
        InputReader {}
    }

}

impl InputReaderLike for InputReader {

    fn read_line(&self, string: String) -> Result<String, String> {
        // TODO
        Ok(String::new())
    }

}

#[derive(Mock)]
pub trait InputReaderLike {
    fn read_line(&self, string: String) -> Result<String, String>;
}
