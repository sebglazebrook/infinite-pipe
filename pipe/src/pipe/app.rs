use pipe::{InputReaderLike, HistoryLike, CommandRunnerLike, CommandResponse};

pub struct App {
    pub line_index: usize,
    pub input_reader: Box<InputReaderLike>,
    pub external_history: Box<HistoryLike>,
    pub command_runner: Box<CommandRunnerLike>,
}

impl App {

    pub fn start(&mut self) -> usize {
        loop {
            match self.read_input() {
                Err(error_message) => { break; },
                Ok(input) => { 
                    match self.command_runner.next_command(input) {
                        CommandResponse::Exit => { break; }
                        _ => {  }
                    }
                }
            }
        }
        self.external_history.push(self.command_runner.resultant_command());
        0 // TODO return a real error code
    }

    // private //
    
    fn read_input(&mut self) -> Result<String, &'static str> {
        let result = self.input_reader.read_line(self.line_index);
        self.line_index += 1;
        result
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use pipe::AppBuilder;
    use mockers::Scenario;

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

    describe! start {

        before_each {
            let mut scenario = Scenario::new();
            let mut readline = scenario.create_mock_for::<InputReaderLike>();
            let external_history = HistoryDouble { lines: vec![] };
            let mut command_runner = scenario.create_mock_for::<CommandRunnerLike>();
        }

        it "reads a line of input, sends it for execution and updates the external history" {
            scenario.expect(readline.read_line_call(1).and_return(Ok(String::from("ps -ef"))));
            scenario.expect(command_runner.next_command_call(String::from("ps -ef")).and_return(CommandResponse::Exit));
            scenario.expect(command_runner.resultant_command_call().and_return(String::from("ps -ef")));
            let mut app = AppBuilder::new()
                          .with_readline(readline)
                          .with_external_history(external_history)
                          .with_command_runner(command_runner)
                          .build();
            app.start();
        }


        describe! when_the_command_runner_returns_non_exit {

            it "reads_and_runs_another_input_command" {
                scenario.expect(readline.read_line_call(1).and_return(Ok(String::from("ps -ef"))));
                scenario.expect(command_runner.next_command_call(String::from("ps -ef")).and_return(CommandResponse::Continue));
                scenario.expect(readline.read_line_call(2).and_return(Ok(String::from("grep docker"))));
                scenario.expect(command_runner.next_command_call(String::from("grep docker")).and_return(CommandResponse::Exit));
                scenario.expect(command_runner.resultant_command_call().and_return(String::from("ps -ef | grep docker")));
                let mut app = AppBuilder::new()
                    .with_readline(readline)
                    .with_external_history(external_history)
                    .with_command_runner(command_runner)
                    .build();
                app.start();
            }
        }

        describe! when_the_input_reading_is_not_successful {

            it "exits without trying again" {
                scenario.expect(readline.read_line_call(1).and_return(Err("something went wrong")));
                scenario.expect(command_runner.resultant_command_call().and_return(String::from("")));
                let mut app = AppBuilder::new()
                    .with_readline(readline)
                    .with_external_history(external_history)
                    .with_command_runner(command_runner)
                    .build();
                app.start();
            }
        }
    }
}
