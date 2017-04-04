use pipe::{InputHandler, InputHandlerLike, InputReaderLike, HistoryLike};

pub struct App {
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
    pub line_index: usize,
    pub input_reader: Box<InputReaderLike>,
    pub external_history: Box<HistoryLike>,
    pub input_handler: Box<InputHandlerLike>,
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
                        Err(_) => {
                            self.inputs.pop();
                            break; 
                        }
                        Ok(output) => {
                            println!("{}", output);
                            self.outputs.push(output);
                        },
                    }
                },
            }
        }
        self.add_command_to_external_history();
        0
    }

    // private //
    
    fn read_input(&mut self) -> Result<String, String> {
        let result = self.input_reader.read_line(self.line_index);
        self.line_index = self.line_index + 1;
        result
    }

    fn add_command_to_external_history(&mut self) {
        let mut full_command = self.inputs.iter().fold(String::new(), |acc, ref input| {
            acc + &input + " | "
        });
        full_command.pop();
        full_command.pop();
        full_command.pop();
        self.external_history.push(full_command);
    }
}

struct InputHandlerDouble;

impl InputHandlerLike for InputHandlerDouble {

    fn handle(&self, input: String, piped_input: Option<String>) -> Result<String, String> {
        Ok(String::new())
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

#[cfg(test)]
mod test {
    use super::*;
    use pipe::AppBuilder;
    use mockers::Scenario;

    #[test]
    fn when_there_multiple_successful_commands_it_updates_the_external_history_with_the_equivalent_pipe_command() {
        let mut scenario = Scenario::new();
        let mut cond = scenario.create_mock_for::<InputReaderLike>();
        scenario.expect(cond.read_line_call(1).and_return(Ok(String::from("ps -ef"))));
        scenario.expect(cond.read_line_call(2).and_return(Ok(String::from("grep docker"))));
        scenario.expect(cond.read_line_call(3).and_return(Err(String::from("An error occurred"))));

        let input_handler_double = InputHandlerDouble {};
        let external_history_double = HistoryDouble { lines: vec![] };
        let mut app = AppBuilder::new()
            .with_readline(cond)
            .with_input_handler(input_handler_double)
            .with_external_history(external_history_double)
            .build();

        app.start();
        assert_eq!(app.external_history.last().unwrap(), "ps -ef | grep docker");
    }

    #[test]
    fn when_there_is_a_successful_command_it_sends_through_the_output_to_the_next_command() {
        let mut scenario = Scenario::new();
        let mut input_reader_mock = scenario.create_mock_for::<InputReaderLike>();
        scenario.expect(input_reader_mock.read_line_call(1).and_return(Ok(String::from("ps -ef"))));
        scenario.expect(input_reader_mock.read_line_call(2).and_return(Ok(String::from("grep docker"))));
        scenario.expect(input_reader_mock.read_line_call(3).and_return(Err(String::from("An error occurred"))));
        // it renders the output
        // and the user enters in a new command
        // it sends the previous output with the new command for processing

        let external_history_double = HistoryDouble { lines: vec![] };

        let mut input_handler_mock = scenario.create_mock_for::<InputHandlerLike>();
        scenario.expect(input_handler_mock.handle_call("ps -ef".to_string(), None).and_return(Ok(String::from("ps -ef output"))));
        scenario.expect(input_handler_mock.handle_call("grep docker".to_string(), Some("ps -ef output".to_string())).and_return(Ok(String::from("grep docker output"))));
        let mut app = AppBuilder::new()
            .with_readline(input_reader_mock)
            .with_input_handler(input_handler_mock)
            .with_external_history(external_history_double)
            .build();

        app.start();
    }
}
