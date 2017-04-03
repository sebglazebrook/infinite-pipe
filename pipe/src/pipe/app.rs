use std::cell::Cell;

use pipe::{InputHandler, InputHandlerLike, AppBuilder};
use pipe::{InputReader, InputReaderLike, HistoryLike, ExternalHistory};

use rl_sys::readline;
use rl_sys::history::listmgmt;


pub struct App {
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
    pub external_history: Box<HistoryLike>,
    pub input_reader: Box<InputReaderLike>,
    pub line_index: usize,
}

impl App {

    pub fn new() -> Self {
        App { inputs: vec![], outputs: vec![], external_history: Box::new(ExternalHistory::new()), input_reader: Box::new(InputReader::new()), line_index: 1 }
    }

    pub fn start(&mut self) -> usize {
        loop {
            match self.read_input() {
                Err(error_message) => { break; },
                Ok(input) => { 
                    self.inputs.push(input.clone());
                    match InputHandler::new(input).handle() {
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
        let result = self.input_reader.read_line(self.line_index.to_string());
        self.line_index = self.line_index + 1;
        result
        //match readline::readline("") {
            //Ok(Some(command)) => { Ok(command) }
            //Ok(None) => { Err("die") },  // user entered ctrl-d
            //Err(e) => { Err("an error occurred") } 
        //}
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

    fn handle(&self) -> Result<String, String> {
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
    use mockers::Scenario;

    #[test]
    fn when_there_multiple_successful_commands_it_updates_the_external_history_with_the_equivalent_pipe_command() {
        let mut scenario = Scenario::new();
        let mut cond = scenario.create_mock_for::<InputReaderLike>();
        scenario.expect(cond.read_line_call("1".to_string()).and_return(Ok(String::from("ps -ef"))));
        scenario.expect(cond.read_line_call("2".to_string()).and_return(Ok(String::from("grep docker"))));
        scenario.expect(cond.read_line_call("3".to_string()).and_return(Err(String::from("An error occurred"))));

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

    //#[test]
    //fn when_there_are_successful_and_unsuccessful_commands() {
        //// it updates the external history with only the successful ones
    //}

    //#[test]
    //fn it_updates_the_internal_history_after_each_successful_command() {
    //}

//#[test]
    //fn it_updates_the_internal_history_after_each_non_successful_command() {
    //}

//#[test]
    //fn when_the_users_input_is_control_c() {
        //// it exists the program
    //}

//#[test]
    //fn when_the_users_input_is_exit() {
        //// it exists the program
    //}

//#[test]
    //fn when_there_is_an_erroneous_command() {
        //// it displays the error output
    //}

//#[test]
    //fn when_there_is_a_successful_command() {

        //// it renders the output
    //}

//#[test]
    //fn when_the_user_enters_back() {
        //// it uses the old stdout for the next command 
    //}
}
