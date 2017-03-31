extern crate rl_sys;

use std::process::Command;

use rl_sys::readline;
use rl_sys::history::listmgmt;

struct InputHandler {
    input: String,
}

impl InputHandler {

    pub fn new(input: String) -> Self {
        InputHandler { input: input }
    }

    pub fn handle(&self) {
        let output = Command::new(self.command())
            .args(&self.args())
            .output()
            .expect("ls command failed to start"); // TODO
        println!("{}", String::from_utf8_lossy(&output.stdout));
    }

    // private //

    fn command(&self) -> String {
        match self.input.split_whitespace().take(1).next() {
            None => { String::new() } // TODO
            Some(command_string) => { command_string.to_string() }
        }
    }

    fn args(&self) -> Vec<String> {
        self.input.split_whitespace().skip(1).map( |element| {
            element.to_string()
        }).collect()
    }

}

struct App;

impl App {

    pub fn new() -> Self {
        App {}
    }

    pub fn start(&self) -> usize {
        loop {
            match self.read_input() {
                Err(error_message) => {},
                Ok(input) => { InputHandler::new(input).handle(); },
            }
            break;
        }
        0
    }

    // private //
    
    fn read_input(&self) -> Result<String, &'static str> {
        match readline::readline("") {
            Ok(Some(command)) => { Ok(command) }
            Ok(None) => { Err("die") },  // user entered ctrl-d
            Err(e) => { Err("an error occurred") } 
        }
    }
}

fn main() {
    let exit_code = App::new().start();
}
