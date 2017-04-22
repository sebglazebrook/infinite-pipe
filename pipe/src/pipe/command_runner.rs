use pipe::{InputHandlerLike, LoggerLike, InputResult};

pub enum CommandResponse {
    Exit,
    Continue,
}

pub struct CommandRunner {
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
    pub logger: Box<LoggerLike>,
    pub input_handler: Box<InputHandlerLike>,
}

impl CommandRunner {

    pub fn next_command(&mut self, input: String)  -> CommandResponse {
        self.inputs.push(input.clone());
        let output = match self.outputs.iter().last() {
            Some(output) => { Some(output.clone()) },
            None => { None },
        };
        match self.input_handler.handle(input, output) {
            InputResult::Error(_) => {
                self.inputs.pop();
                return CommandResponse::Exit;
            }
            InputResult::Success(output) => {
                self.logger.log(output.clone());
                self.outputs.push(output);
            },
            InputResult::Back => {
                self.inputs.pop();
                self.inputs.pop();
                self.outputs.pop();
                match self.outputs.iter().last() {
                    None => {},
                    Some(output) => {
                        println!("{}", output);
                    }
                }
            },
                InputResult::Break => {
                    self.inputs.clear();
                    self.outputs.clear();
                },
                InputResult::Quit => {
                    self.inputs.pop();
                    return CommandResponse::Exit;
                },
        }
        CommandResponse::Continue
    }

    pub fn resultant_command(&self) -> String {
        self.inputs.iter().fold(String::new(), |acc, ref input| {
            acc + &input + " | "
        }).trim_right_matches(" | ").to_string()
    }

}
