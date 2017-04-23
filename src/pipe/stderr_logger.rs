use std::io::Write;
use std::io::stderr;

use super::LoggerLike;

pub struct StderrLogger;

impl StderrLogger {

    pub fn new() -> Self {
        StderrLogger {}
    }
}

impl LoggerLike for StderrLogger {

    fn log(&self, content: String) {
        let r = writeln!(&mut stderr(), "Error! {}", content);
        r.expect("failed printing to stderr");
    }

}

