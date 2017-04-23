use std::process::{Command, Stdio};
use std::io::prelude::*;

pub enum InputResult {
    Success(String),
    Error(String),
    Back,
    Break,
    Quit,
}

pub struct InputHandler;

impl InputHandler {

    pub fn new() -> Self {
        InputHandler { }
    }

    // private //

    fn command(&self, input: &String) -> String {
        match input.split_whitespace().take(1).next() {
            None => { String::new() } // TODO
            Some(command_string) => { command_string.to_string() }
        }
    }

    fn args(&self, input: &String) -> Vec<String> {
        input.split_whitespace().skip(1).map( |element| {
            element.to_string()
        }).collect()
    }

}

impl InputHandlerLike for InputHandler {

    fn handle(&self, input: String, piped_input: Option<String>) -> InputResult {
        if input == "back" {
            return InputResult::Back;
        }
        if input == "break" {
            return InputResult::Break;
        }
        if input == "exit" || input == "" {
            return InputResult::Quit;
        }
        let process;
        match Command::new(self.command(&input))
            .args(&self.args(&input))
            .stdout(Stdio::piped())
            .stdin(Stdio::piped())
            .spawn() {
                Ok(p) => { process = p; }
                Err(_) => { return InputResult::Error(String::from("something went wrong")) }
            }

        if piped_input.is_some() {
            let _ = process.stdin.unwrap().write_all(piped_input.unwrap().as_bytes());
        }


        let mut output = String::new();
        let _ = process.stdout.unwrap().read_to_string(&mut output);

        InputResult::Success(output)
    }

}

#[derive(Mock)]
pub trait InputHandlerLike {

    fn handle(&self, input: String, piped_input: Option<String>) -> InputResult;
}
