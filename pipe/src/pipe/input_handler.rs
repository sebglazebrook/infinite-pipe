use std::process::Command;

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

    fn handle(&self, input: String) -> Result<String, String> {
        let output = Command::new(self.command(&input))
            .args(&self.args(&input))
            .output();
        match output {
            Err(error_message) => { Err(String::new()) } // TODO
            Ok(output) => {
                let stringed_output = String::from_utf8_lossy(&output.stdout);
                Ok(stringed_output.into_owned())
            },
        }
    }

}

#[derive(Mock)]
pub trait InputHandlerLike {

    fn handle(&self, input: String) -> Result<String, String>;
}
