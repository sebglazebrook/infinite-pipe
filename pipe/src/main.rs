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

    pub fn handle(&self) -> Result<String, String> {
        let output = Command::new(self.command())
            .args(&self.args())
            .output();
        match output {
            Err(error_message) => { Err(String::new()) } // TODO
            Ok(output) => {
                let stringed_output = String::from_utf8_lossy(&output.stdout);
                Ok(stringed_output.into_owned())
            },
        }
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

struct App {
    inputs: Vec<String>,
}

impl App {

    pub fn new() -> Self {
        App { inputs: vec![] }
    }

    pub fn start(&mut self) -> usize {
        loop {
            match self.read_input() {
                Err(error_message) => { break; },
                Ok(input) => { 
                    self.inputs.push(input.clone());
                    match InputHandler::new(input).handle() {
                        Err(_) => { break; }
                        Ok(output) => { println!("{}", output); },
                    }
                },
            }
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

// handle the resultant_command
// handle control+c
// output error messages
// update readline session history
// add custom command handling, back, exit
// update external history
