use pipe::{InputHandler, InputHandlerLike, InputReaderLike, HistoryLike, LoggerLike, InputResult};

pub struct App {
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
    pub line_index: usize,
    pub input_reader: Box<InputReaderLike>,
    pub external_history: Box<HistoryLike>,
    pub input_handler: Box<InputHandlerLike>,
    pub logger: Box<LoggerLike>,
}

impl App {

    pub fn start(&mut self) -> usize {
        loop {
            match self.read_input() {
                Err(error_message) => { break; },
                Ok(input) => { 
                    self.inputs.push(input.clone());
                    let output = match self.outputs.iter().last() {
                        Some(output) => { Some(output.clone()) },
                        None => { None },
                    };
                    match self.input_handler.handle(input, output) {
                        InputResult::Error(_) => {
                            self.inputs.pop();
                            break; 
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
                            break;
                        },
                    }
                },
            }
        }
        self.add_command_to_external_history();
        0 // TODO return a real error code
    }

    // private //
    
    fn read_input(&mut self) -> Result<String, &'static str> {
        let result = self.input_reader.read_line(self.line_index);
        self.line_index += 1;
        result
    }

    fn add_command_to_external_history(&mut self) {
        self.external_history.push(
            self.inputs.iter().fold(String::new(), |acc, ref input| {
                acc + &input + " | "
            }).trim_right_matches(" | ").to_string()
        );
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pipe::AppBuilder;
    use mockers::Scenario;


    struct InputHandlerDouble;

    impl InputHandlerLike for InputHandlerDouble {

        fn handle(&self, input: String, piped_input: Option<String>) -> InputResult {
            InputResult::Success(String::new())
        }
    }

    struct HistoryDouble {
        lines: Vec<String>,
    }

    impl HistoryLike for HistoryDouble {

        fn push(&mut self, command: String) {
            self.lines.push(command);
        }

        fn last(&self) -> Option<String> {
            match self.lines.last() {
                Some(string) => Some(string.clone().to_string()),
                None => None,
            }
        }
    }

    struct LoggerDouble;
    impl LoggerLike for LoggerDouble {

        fn log(&self, content: String) {
        }
    }

    describe! start  {

        before_each {
            let mut scenario = Scenario::new();
            let mut cond = scenario.create_mock_for::<InputReaderLike>();
            let input_handler_double = InputHandlerDouble {};
            let external_history_double = HistoryDouble { lines: vec![] };
            let logger_double = LoggerDouble {};
        }

        describe! when_there_are_multiple_commands {

            before_each {
                scenario.expect(cond.read_line_call(1).and_return(Ok(String::from("ps -ef"))));
                scenario.expect(cond.read_line_call(2).and_return(Ok(String::from("grep docker"))));
                scenario.expect(cond.read_line_call(3).and_return(Err("An error occurred")));
            }

            it "updates the external history with the pipe command" {
                let mut app = AppBuilder::new()
                    .with_readline(cond)
                    .with_input_handler(input_handler_double)
                    .with_external_history(external_history_double)
                    .with_logger(logger_double)
                    .build();
                app.start();
                assert_eq!(app.external_history.last().unwrap(), "ps -ef | grep docker");
            }

            it "logs the stdout" {
                let mut app = AppBuilder::new()
                    .with_readline(cond)
                    .with_input_handler(input_handler_double)
                    .with_external_history(external_history_double)
                    .with_logger(logger_double)
                    .build();
                app.start();
                // incorrect assertion
                assert_eq!(app.external_history.last().unwrap(), "ps -ef | grep docker");
            }

            it "sends through the output to the next command" {
                let mut input_handler_mock = scenario.create_mock_for::<InputHandlerLike>();
                scenario.expect(input_handler_mock.handle_call("ps -ef".to_string(), None).and_return(InputResult::Success(String::from("ps -ef output"))));
                scenario.expect(input_handler_mock.handle_call("grep docker".to_string(), Some("ps -ef output".to_string())).and_return(InputResult::Success(String::from("grep docker output"))));

                let mut app = AppBuilder::new()
                    .with_readline(cond)
                    .with_input_handler(input_handler_mock)
                    .with_external_history(external_history_double)
                    .with_logger(logger_double)
                    .build();
                app.start();
            }
        }

        describe! when_the_user_wants_to_go_back_in_the_pipe {

            before_each {
                scenario.expect(cond.read_line_call(1).and_return(Ok(String::from("ps -ef"))));
                scenario.expect(cond.read_line_call(2).and_return(Ok(String::from("grep docker"))));
                scenario.expect(cond.read_line_call(3).and_return(Ok(String::from("back"))));
                scenario.expect(cond.read_line_call(4).and_return(Ok(String::from("grep rails"))));
                scenario.expect(cond.read_line_call(5).and_return(Err("An error occurred")));
            }

            it "sends the previous stdout output to the next command" {
                let mut input_handler_mock = scenario.create_mock_for::<InputHandlerLike>();
                scenario.expect(input_handler_mock.handle_call("ps -ef".to_string(), None).and_return(InputResult::Success(String::from("ps -ef output"))));
                scenario.expect(input_handler_mock.handle_call("grep docker".to_string(), Some("ps -ef output".to_string())).and_return(InputResult::Success(String::from("grep docker output"))));
                scenario.expect(input_handler_mock.handle_call("back".to_string(), Some("grep docker output".to_string())).and_return(InputResult::Back));
                scenario.expect(input_handler_mock.handle_call("grep rails".to_string(), Some("ps -ef output".to_string())).and_return(InputResult::Success(String::from("grep docker output"))));
                let mut app = AppBuilder::new()
                    .with_readline(cond)
                    .with_input_handler(input_handler_mock)
                    .with_external_history(external_history_double)
                    .with_logger(logger_double)
                    .build();
                app.start();
            }

            it "does not save the `back` command to the final history" {
                let mut input_handler_mock = scenario.create_mock_for::<InputHandlerLike>();
                scenario.expect(input_handler_mock.handle_call("ps -ef".to_string(), None).and_return(InputResult::Success(String::from("ps -ef output"))));
                scenario.expect(input_handler_mock.handle_call("grep docker".to_string(), Some("ps -ef output".to_string())).and_return(InputResult::Success(String::from("grep docker output"))));
                scenario.expect(input_handler_mock.handle_call("back".to_string(), Some("grep docker output".to_string())).and_return(InputResult::Back));
                scenario.expect(input_handler_mock.handle_call("grep rails".to_string(), Some("ps -ef output".to_string())).and_return(InputResult::Success(String::from("grep docker output"))));

                let mut app = AppBuilder::new()
                    .with_readline(cond)
                    .with_input_handler(input_handler_mock)
                    .with_external_history(external_history_double)
                    .with_logger(logger_double)
                    .build();
                app.start();
                assert_eq!(app.external_history.last().unwrap(), "ps -ef | grep rails");
            }
        }

        describe! when_the_user_wants_to_break_the_pipe {

            before_each {
                scenario.expect(cond.read_line_call(1).and_return(Ok(String::from("ps -ef"))));
                scenario.expect(cond.read_line_call(2).and_return(Ok(String::from("break"))));
                scenario.expect(cond.read_line_call(3).and_return(Ok(String::from("grep docker"))));
                scenario.expect(cond.read_line_call(4).and_return(Err("An error occurred")));
            }

            it "does not send the stdout to the next command" {
                let mut input_handler_mock = scenario.create_mock_for::<InputHandlerLike>();
                scenario.expect(input_handler_mock.handle_call("ps -ef".to_string(), None).and_return(InputResult::Success(String::from("ps -ef output"))));
                scenario.expect(input_handler_mock.handle_call("break".to_string(), Some("ps -ef output".to_string())).and_return(InputResult::Break));
                scenario.expect(input_handler_mock.handle_call("grep docker".to_string(), None).and_return(InputResult::Success(String::from("grep docker output"))));
                let mut app = AppBuilder::new()
                    .with_readline(cond)
                    .with_input_handler(input_handler_mock)
                    .with_external_history(external_history_double)
                    .with_logger(logger_double)
                    .build();
                app.start();
            }

            it "add the `post break` command to the external history only" {
                let mut input_handler_mock = scenario.create_mock_for::<InputHandlerLike>();
                scenario.expect(input_handler_mock.handle_call("ps -ef".to_string(), None).and_return(InputResult::Success(String::from("ps -ef output"))));
                scenario.expect(input_handler_mock.handle_call("break".to_string(), Some("ps -ef output".to_string())).and_return(InputResult::Break));
                scenario.expect(input_handler_mock.handle_call("grep docker".to_string(), None).and_return(InputResult::Success(String::from("grep docker output"))));
                let mut app = AppBuilder::new()
                    .with_readline(cond)
                    .with_input_handler(input_handler_mock)
                    .with_external_history(external_history_double)
                    .with_logger(logger_double)
                    .build();
                app.start();
                assert_eq!(app.external_history.last().unwrap(), "grep docker");
            }
        }

        describe! when_the_user_wants_to_exit {

            before_each {
                scenario.expect(cond.read_line_call(1).and_return(Ok(String::from("ps -ef"))));
                scenario.expect(cond.read_line_call(2).and_return(Ok(String::from("exit"))));
            }

            it "does NOT ask the user for more input" {
                let mut input_handler_mock = scenario.create_mock_for::<InputHandlerLike>();
                scenario.expect(input_handler_mock.handle_call("ps -ef".to_string(), None).and_return(InputResult::Success(String::from("ps -ef output"))));
                scenario.expect(input_handler_mock.handle_call("exit".to_string(), Some("ps -ef output".to_string())).and_return(InputResult::Quit));
                let mut app = AppBuilder::new()
                    .with_readline(cond)
                    .with_input_handler(input_handler_mock)
                    .with_external_history(external_history_double)
                    .with_logger(logger_double)
                    .build();
                app.start();
            }

            it "saves the built up command to the users history" {
                let mut input_handler_mock = scenario.create_mock_for::<InputHandlerLike>();
                scenario.expect(input_handler_mock.handle_call("ps -ef".to_string(), None).and_return(InputResult::Success(String::from("ps -ef output"))));
                scenario.expect(input_handler_mock.handle_call("exit".to_string(), Some("ps -ef output".to_string())).and_return(InputResult::Quit));
                let mut app = AppBuilder::new()
                    .with_readline(cond)
                    .with_input_handler(input_handler_mock)
                    .with_external_history(external_history_double)
                    .with_logger(logger_double)
                    .build();
                app.start();
                assert_eq!(app.external_history.last().unwrap(), "ps -ef");
            }
        }
    }
}
