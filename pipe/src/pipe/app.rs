use pipe::InputHandler;

use rl_sys::readline;
use rl_sys::history::listmgmt;

pub struct App {
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
