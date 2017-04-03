use std::process::Command;

pub struct InputHandler {
    input: String,
}


impl InputHandler {

    pub fn new(input: String) -> Self {
        InputHandler { input: input }
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

impl InputHandlerLike for InputHandler {

    fn handle(&self) -> Result<String, String> {
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

}

#[derive(Mock)]
pub trait InputHandlerLike {

    fn handle(&self) -> Result<String, String>;
}
